//! Client-side window decorations: a draggable titlebar, window controls and
//! resize edges for a window opened without native decorations.
//!
//! Open the window with [`window_settings`], wrap the view in [`frame`], and
//! hand every [`Action`] to [`perform`] from `update`.
use crate::{focusable, icon, style, widget};
use iced::{
    Element, Event, Length, Point, Rectangle, Renderer, Size, Task, Theme,
    Vector,
    advanced::{
        Clipboard, Layout, Shell, Widget, layout, mouse, overlay, renderer,
        widget::{Operation, Tree, tree},
    },
    widget::{Space, button, column, container, mouse_area, row, stack, text},
    window::{self, Direction},
};

const EDGE: f32 = 4.0;
const CORNER: f32 = 10.0;
/// How far the pointer travels before a press on a drag region moves the window.
const DRAG_THRESHOLD: f32 = 4.0;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Drag,
    Minimize,
    ToggleMaximize,
    Close,
    Resize(Direction),
}

/// Window settings without native decorations.
pub fn window_settings() -> window::Settings {
    window::Settings {
        decorations: false,
        ..Default::default()
    }
}

/// Undecorated window settings with a desktop application identity.
pub fn window_settings_for(id: impl Into<String>) -> window::Settings {
    let id = id.into();
    #[cfg(target_os = "linux")]
    return window::Settings {
        platform_specific: window::settings::PlatformSpecific {
            application_id: id,
            ..Default::default()
        },
        ..window_settings()
    };
    #[cfg(not(target_os = "linux"))]
    {
        let _ = id;
        window_settings()
    }
}

/// Runs an [`Action`] on the application's window.
pub fn perform<Message: Send + 'static>(action: Action) -> Task<Message> {
    window::latest().and_then(move |id| match action {
        Action::Drag => window::drag(id),
        Action::Minimize => window::minimize(id, true),
        Action::ToggleMaximize => window::toggle_maximize(id),
        Action::Close => window::close(id),
        Action::Resize(direction) => window::drag_resize(id, direction),
    })
}

/// Wraps the whole window content and adds invisible resize edges around it.
pub fn frame<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    on_action: fn(Action) -> Message,
) -> Element<'a, Message> {
    use mouse::Interaction::{
        ResizingDiagonallyDown as Nwse, ResizingDiagonallyUp as Nesw,
        ResizingHorizontally as Ew, ResizingVertically as Ns,
    };
    let edge = |direction, interaction, width: Length, height: Length| {
        Element::from(
            mouse_area(Space::new().width(width).height(height))
                .interaction(interaction)
                .on_press(on_action(Action::Resize(direction))),
        )
    };
    let (edge_size, corner, fill) =
        (Length::Fixed(EDGE), Length::Fixed(CORNER), Length::Fill);
    stack![
        content.into(),
        column![
            row![
                edge(Direction::NorthWest, Nwse, corner, edge_size),
                edge(Direction::North, Ns, fill, edge_size),
                edge(Direction::NorthEast, Nesw, corner, edge_size),
            ],
            row![
                edge(Direction::West, Ew, edge_size, fill),
                Space::new().width(fill).height(fill),
                edge(Direction::East, Ew, edge_size, fill),
            ]
            .height(fill),
            row![
                edge(Direction::SouthWest, Nesw, corner, edge_size),
                edge(Direction::South, Ns, fill, edge_size),
                edge(Direction::SouthEast, Nwse, corner, edge_size),
            ],
        ],
    ]
    .into()
}

/// Dragging `content` moves the window; double-clicking it toggles maximize.
/// Buttons inside keep their clicks.
pub fn drag_region<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    on_action: fn(Action) -> Message,
) -> Element<'a, Message> {
    Element::new(DragRegion {
        content: content.into(),
        on_drag: on_action(Action::Drag),
        on_double_click: on_action(Action::ToggleMaximize),
    })
}

/// Minimize, maximize and close, in T3 Code's symbols.
pub fn controls<'a, Message: Clone + 'a>(
    on_action: fn(Action) -> Message,
) -> Element<'a, Message> {
    let control = |id: &'static str, glyph, size, action| {
        let message = on_action(action);
        focusable(
            id,
            button(
                container(widget::icon(glyph).size(size))
                    .center_x(26)
                    .center_y(26),
            )
            .padding(0)
            .style(style::ghost_muted)
            .on_press(message.clone()),
            Some(message),
        )
    };
    // The diamond fills its glyph box; the others use about half of theirs.
    row![
        control("window-minimize", icon::CHEVRON_DOWN, 16, Action::Minimize),
        control("window-maximize", icon::DIAMOND, 12, Action::ToggleMaximize),
        control("window-close", icon::X, 16, Action::Close),
    ]
    .spacing(2)
    .into()
}

/// A complete titlebar: a title, optional tools and the window controls.
pub fn titlebar<'a, Message: Clone + 'a>(
    title: impl text::IntoFragment<'a>,
    tools: Option<Element<'a, Message>>,
    on_action: fn(Action) -> Message,
) -> Element<'a, Message> {
    bar(
        widget::text(title, 14).font(crate::font::MEDIUM),
        tools,
        on_action,
    )
}

/// A titlebar with any content on the left, such as [`widget::breadcrumb`].
pub fn bar<'a, Message: Clone + 'a>(
    leading: impl Into<Element<'a, Message>>,
    tools: Option<Element<'a, Message>>,
    on_action: fn(Action) -> Message,
) -> Element<'a, Message> {
    // Clipped, so a long title can't push the window controls out of the window.
    let mut content = row![container(leading).width(Length::Fill).clip(true)]
        .spacing(8)
        .align_y(iced::Center);
    if let Some(tools) = tools {
        content = content.push(tools);
    }
    drag_region(
        container(content.push(controls(on_action)))
            .padding(iced::Padding::from([0, 8]).left(16))
            .center_y(44),
        on_action,
    )
}

/// A window drag hands the pointer to the compositor, which would swallow the
/// second click of a double-click, so it starts only once the pointer moves.
struct DragRegion<'a, Message> {
    content: Element<'a, Message>,
    on_drag: Message,
    on_double_click: Message,
}

#[derive(Default)]
struct DragState {
    press: Option<Point>,
    previous_click: Option<mouse::click::Click>,
}

impl<Message: Clone> Widget<Message, Theme, Renderer>
    for DragRegion<'_, Message>
{
    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<DragState>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(DragState::default())
    }
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }
    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }
    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content.as_widget_mut().layout(
            &mut tree.children[0],
            renderer,
            limits,
        )
    }
    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        self.content.as_widget_mut().operate(
            &mut tree.children[0],
            layout,
            renderer,
            operation,
        );
    }
    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
        if shell.is_event_captured() {
            return;
        }
        let state = tree.state.downcast_mut::<DragState>();
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let Some(position) = cursor.position_over(layout.bounds())
                else {
                    return;
                };
                let click = mouse::click::Click::new(
                    position,
                    mouse::Button::Left,
                    state.previous_click,
                );
                state.previous_click = Some(click);
                if click.kind() == mouse::click::Kind::Double {
                    state.press = None;
                    shell.publish(self.on_double_click.clone());
                } else {
                    state.press = Some(position);
                }
                shell.capture_event();
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if state
                    .press
                    .is_some_and(|p| p.distance(*position) >= DRAG_THRESHOLD)
                {
                    // The compositor keeps the release, so forget the press now.
                    state.press = None;
                    shell.publish(self.on_drag.clone());
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                state.press = None;
            }
            _ => {}
        }
    }
    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }
    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }
    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::advanced::{clipboard, renderer::Headless};

    fn press() -> Event {
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
    }
    fn release() -> Event {
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
    }
    fn moved(x: f32) -> Event {
        Event::Mouse(mouse::Event::CursorMoved {
            position: Point::new(x, 10.0),
        })
    }
    /// Feeds pointer events to a drag region and returns what it asked for.
    fn actions(events: &[Event]) -> Vec<Action> {
        let mut region = drag_region(
            Space::new().width(100).height(40),
            std::convert::identity,
        );
        let mut tree = Tree::new(&region);
        let renderer = pollster::block_on(<Renderer as Headless>::new(
            crate::font::REGULAR,
            14.0.into(),
            Some("tiny-skia"),
        ))
        .expect("software renderer");
        let size = Size::new(100.0, 40.0);
        let node = region.as_widget_mut().layout(
            &mut tree,
            &renderer,
            &layout::Limits::new(size, size),
        );
        let mut messages = Vec::new();
        let mut cursor = Point::new(10.0, 10.0);
        for event in events {
            if let Event::Mouse(mouse::Event::CursorMoved { position }) = event
            {
                cursor = *position;
            }
            region.as_widget_mut().update(
                &mut tree,
                event,
                Layout::new(&node),
                mouse::Cursor::Available(cursor),
                &renderer,
                &mut clipboard::Null,
                &mut Shell::new(&mut messages),
                &Rectangle::with_size(size),
            );
        }
        messages
    }

    #[test]
    fn a_long_title_leaves_the_window_controls_in_the_window() {
        let title = "A title much too long for the window it sits in".repeat(4);
        let mut bar =
            bar(widget::text(title, 14), None, std::convert::identity);
        let mut tree = Tree::new(&bar);
        let renderer = pollster::block_on(<Renderer as Headless>::new(
            crate::font::REGULAR,
            14.0.into(),
            Some("tiny-skia"),
        ))
        .expect("software renderer");
        let size = Size::new(300.0, 44.0);
        let node = bar.as_widget_mut().layout(
            &mut tree,
            &renderer,
            &layout::Limits::new(size, size),
        );
        // The middle of the close button: 8px of padding and half of its 26px.
        let position = Point::new(size.width - 21.0, 22.0);
        let mut messages = Vec::new();
        for event in [
            Event::Mouse(mouse::Event::CursorMoved { position }),
            press(),
            release(),
        ] {
            bar.as_widget_mut().update(
                &mut tree,
                &event,
                Layout::new(&node),
                mouse::Cursor::Available(position),
                &renderer,
                &mut clipboard::Null,
                &mut Shell::new(&mut messages),
                &Rectangle::with_size(size),
            );
        }
        assert!(matches!(messages[..], [Action::Close]), "{messages:?}");
    }
    #[test]
    fn a_plain_click_never_starts_a_drag() {
        assert!(actions(&[press(), moved(12.0), release()]).is_empty());
    }
    #[test]
    fn moving_past_the_threshold_drags_once() {
        let actions = actions(&[press(), moved(20.0), moved(40.0)]);
        assert!(matches!(actions[..], [Action::Drag]));
    }
    #[test]
    fn a_double_click_maximizes_without_dragging() {
        let actions = actions(&[press(), release(), press(), release()]);
        assert!(matches!(actions[..], [Action::ToggleMaximize]));
    }
    #[test]
    fn moving_after_release_does_not_drag() {
        assert!(actions(&[press(), release(), moved(40.0)]).is_empty());
    }
}
