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
                        color: theme.palette().primary,
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
