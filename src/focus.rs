//! Keyboard focus for iced's standard button, which is pointer-only in 0.14.
//! Rendering, layout and pointer behavior remain the built-in button's work.

use iced::advanced::Renderer as _;
use iced::{
    Element, Event, Length, Rectangle, Renderer, Size, Theme,
    advanced::{
        Clipboard, Layout, Shell, Widget, layout, renderer,
        widget::{self, Operation, Tree, operation::Focusable, tree},
    },
    keyboard, mouse,
};

/// Makes `content` reachable with Tab and activated with Enter or Space.
pub fn focusable<'a, Message: Clone + 'a>(
    id: impl Into<widget::Id>,
    content: impl Into<Element<'a, Message>>,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    Element::new(KeyboardButton {
        id: id.into(),
        content: content.into(),
        on_press,
        cancel_drag: false,
    })
}

/// Lets Escape abandon a pointer gesture in `content`, such as a slider drag.
/// The widget forgets the press, so releasing it later changes nothing. The app
/// restores the value itself, for example from an Escape shortcut.
pub fn cancellable<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    Element::new(KeyboardButton {
        id: widget::Id::unique(),
        content: content.into(),
        on_press: None,
        cancel_drag: true,
    })
}

struct KeyboardButton<'a, Message> {
    id: widget::Id,
    content: Element<'a, Message>,
    on_press: Option<Message>,
    cancel_drag: bool,
}

/// A click focuses the button for Enter/Space, but like CSS `:focus-visible`
/// only keyboard focus shows the ring.
#[derive(Default)]
struct Focus {
    focused: bool,
    ring: bool,
}
impl Focusable for Focus {
    fn is_focused(&self) -> bool {
        self.focused
    }
    fn focus(&mut self) {
        self.focused = true;
        self.ring = true;
    }
    fn unfocus(&mut self) {
        self.focused = false;
        self.ring = false;
    }
}

impl<Message: Clone> Widget<Message, Theme, Renderer>
    for KeyboardButton<'_, Message>
{
    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Focus>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(Focus::default())
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
        let state = tree.state.downcast_mut::<Focus>();
        if self.on_press.is_some() {
            operation.focusable(Some(&self.id), layout.bounds(), state);
        } else {
            state.unfocus();
        }
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
        if self.cancel_drag
            && matches!(
                event,
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(keyboard::key::Named::Escape),
                    ..
                })
            )
        {
            tree.children[0] = Tree::new(&self.content);
            return;
        }
        let state = tree.state.downcast_mut::<Focus>();
        if matches!(
            event,
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
        ) {
            state.focused =
                self.on_press.is_some() && cursor.is_over(layout.bounds());
            state.ring = false;
        }
        if state.focused
            && let Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) =
                event
            && matches!(
                key,
                keyboard::Key::Named(
                    keyboard::key::Named::Enter | keyboard::key::Named::Space
                )
            )
            && let Some(message) = &self.on_press
        {
            shell.publish(message.clone());
            shell.capture_event();
            return;
        }
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
        if tree.state.downcast_ref::<Focus>().ring {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border: iced::Border {
                        color: crate::color::FOCUS,
                        width: 2.0,
                        radius: 8.0.into(),
                    },
                    ..renderer::Quad::default()
                },
                iced::Color::TRANSPARENT,
            );
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::{
        Point,
        advanced::{
            clipboard, renderer::Headless,
            widget::operation::focusable as focus,
        },
        widget::{button, checkbox, slider},
    };

    #[derive(Debug, Clone, PartialEq)]
    enum Message {
        Press,
        Value(u16),
        Released,
    }

    struct Harness {
        renderer: Renderer,
        content: Element<'static, Message>,
        tree: Tree,
        node: layout::Node,
    }
    impl Harness {
        fn new(mut content: Element<'static, Message>) -> Self {
            let renderer = pollster::block_on(<Renderer as Headless>::new(
                iced::Font::DEFAULT,
                16.0.into(),
                Some("tiny-skia"),
            ))
            .expect("software renderer");
            let mut tree = Tree::new(&content);
            let node = content.as_widget_mut().layout(
                &mut tree,
                &renderer,
                &layout::Limits::new(Size::ZERO, Size::new(320.0, 80.0)),
            );
            Self {
                renderer,
                content,
                tree,
                node,
            }
        }
        fn focus(&mut self, id: &str) {
            let mut operation =
                focus::focus::<()>(widget::Id::from(id.to_owned()));
            self.content.as_widget_mut().operate(
                &mut self.tree,
                Layout::new(&self.node),
                &self.renderer,
                &mut operation,
            );
        }
        fn event(
            &mut self,
            event: &Event,
            cursor: mouse::Cursor,
        ) -> (Vec<Message>, iced::event::Status) {
            let mut messages = Vec::new();
            let mut shell = Shell::new(&mut messages);
            self.content.as_widget_mut().update(
                &mut self.tree,
                event,
                Layout::new(&self.node),
                cursor,
                &self.renderer,
                &mut clipboard::Null,
                &mut shell,
                &self.node.bounds(),
            );
            let status = shell.event_status();
            (messages, status)
        }
    }

    fn key(named: keyboard::key::Named) -> Event {
        Event::Keyboard(keyboard::Event::KeyPressed {
            key: keyboard::Key::Named(named),
            modified_key: keyboard::Key::Named(named),
            physical_key: keyboard::key::Physical::Unidentified(
                keyboard::key::NativeCode::Unidentified,
            ),
            location: keyboard::Location::Standard,
            modifiers: keyboard::Modifiers::empty(),
            text: None,
            repeat: false,
        })
    }

    #[test]
    fn a_focused_button_publishes_once_for_enter_and_space() {
        let mut control = Harness::new(focusable(
            "apply",
            button("Apply").on_press(Message::Press),
            Some(Message::Press),
        ));
        control.focus("apply");
        for named in [keyboard::key::Named::Enter, keyboard::key::Named::Space]
        {
            let (messages, status) =
                control.event(&key(named), mouse::Cursor::Unavailable);
            assert_eq!(messages, [Message::Press]);
            assert_eq!(status, iced::event::Status::Captured);
        }
    }
    #[test]
    fn a_disabled_button_cannot_focus_or_publish() {
        let mut control =
            Harness::new(focusable("disabled", button("Disabled"), None));
        control.focus("disabled");
        assert!(!control.tree.state.downcast_ref::<Focus>().is_focused());
        for named in [keyboard::key::Named::Enter, keyboard::key::Named::Space]
        {
            let (messages, status) =
                control.event(&key(named), mouse::Cursor::Unavailable);
            assert!(messages.is_empty());
            assert_eq!(status, iced::event::Status::Ignored);
        }
    }
    #[test]
    fn a_focused_checkbox_toggles_once_with_space() {
        let checkbox =
            checkbox(false).label("Check").on_toggle(|_| Message::Press);
        let mut control =
            Harness::new(focusable("check", checkbox, Some(Message::Press)));
        control.focus("check");
        let (messages, status) = control.event(
            &key(keyboard::key::Named::Space),
            mouse::Cursor::Unavailable,
        );
        assert_eq!(messages, [Message::Press]);
        assert_eq!(status, iced::event::Status::Captured);
    }
    #[test]
    fn escape_cancels_a_slider_drag_without_a_release_commit() {
        let slider = slider(1_u16..=32000, 800, Message::Value)
            .on_release(Message::Released);
        let mut control = Harness::new(cancellable(slider));
        let bounds = control.node.bounds();
        let cursor = mouse::Cursor::Available(Point::new(
            bounds.center_x(),
            bounds.center_y(),
        ));
        let press =
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));
        let release =
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left));
        let (messages, _) = control.event(&press, cursor);
        assert!(matches!(messages[..], [Message::Value(_)]));
        control.event(&key(keyboard::key::Named::Escape), cursor);
        let (messages, _) = control.event(&release, cursor);
        assert!(messages.is_empty());
        control.event(&press, cursor);
        let (messages, _) = control.event(&release, cursor);
        assert_eq!(messages, [Message::Released]);
    }
}
