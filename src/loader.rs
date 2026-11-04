//! Loading indicators. They schedule their own frames, so an app only has to
//! show them; once out of the view they cost nothing.
use crate::color;
use iced::{
    Color, Element, Event, Length, Point, Rectangle, Renderer, Size, Theme,
    advanced::{
        Clipboard, Layout, Renderer as _, Shell, Widget, layout, mouse,
        renderer,
        widget::{Tree, tree},
    },
    time::Instant,
    window,
};
use std::f32::consts::TAU;

const DOT: f32 = 5.0;
const DOT_GAP: f32 = 3.0;
/// How far the dots swing either side of the center.
const SWING: f32 = 2.0;
const BOUNCE_SECONDS: f32 = 0.9;
const SPINNER_SIZE: f32 = 16.0;
const SPINNER_DOTS: usize = 8;
const SPINNER_SECONDS: f32 = 0.8;

/// Three bouncing dots, for a control that is working.
pub fn dots<'a, Message: 'a>() -> Element<'a, Message> {
    dots_in(color::MUTED)
}
/// [`dots`] in a color of your choice, for example on a filled button.
pub fn dots_in<'a, Message: 'a>(color: Color) -> Element<'a, Message> {
    Element::new(Loader {
        kind: Kind::Dots,
        color,
    })
}
/// A spinner, for a slot the size of an icon.
pub fn spinner<'a, Message: 'a>() -> Element<'a, Message> {
    Element::new(Loader {
        kind: Kind::Spinner,
        color: color::MUTED,
    })
}

#[derive(Clone, Copy)]
enum Kind {
    Dots,
    Spinner,
}

struct Loader {
    kind: Kind,
    color: Color,
}

struct Clock {
    start: Instant,
    now: Instant,
}
impl Clock {
    fn seconds(&self) -> f32 {
        self.now.duration_since(self.start).as_secs_f32()
    }
}

fn circle(center: Point, diameter: f32) -> renderer::Quad {
    renderer::Quad {
        bounds: Rectangle {
            x: center.x - diameter / 2.0,
            y: center.y - diameter / 2.0,
            width: diameter,
            height: diameter,
        },
        border: iced::Border {
            radius: (diameter / 2.0).into(),
            ..Default::default()
        },
        // Snapping to whole pixels would squash these dots and make them jitter.
        snap: false,
        ..Default::default()
    }
}

impl<Message> Widget<Message, Theme, Renderer> for Loader {
    fn size(&self) -> Size<Length> {
        let (width, height) = match self.kind {
            Kind::Dots => (DOT * 3.0 + DOT_GAP * 2.0, DOT + SWING * 2.0 + 1.0),
            Kind::Spinner => (SPINNER_SIZE, SPINNER_SIZE),
        };
        Size::new(Length::Fixed(width), Length::Fixed(height))
    }
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Clock>()
    }
    fn state(&self) -> tree::State {
        let now = Instant::now();
        tree::State::new(Clock { start: now, now })
    }
    fn layout(
        &mut self,
        _: &mut Tree,
        _: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = Widget::<Message, Theme, Renderer>::size(self);
        layout::atomic(limits, size.width, size.height)
    }
    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        _: Layout<'_>,
        _: mouse::Cursor,
        _: &Renderer,
        _: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _: &Rectangle,
    ) {
        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            tree.state.downcast_mut::<Clock>().now = *now;
            shell.request_redraw();
        }
    }
    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        _: &Theme,
        _: &renderer::Style,
        layout: Layout<'_>,
        _: mouse::Cursor,
        _: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let seconds = tree.state.downcast_ref::<Clock>().seconds();
        match self.kind {
            Kind::Dots => {
                for index in 0..3 {
                    // A full wave, so the row stays centered in its box.
                    let wave = (seconds / BOUNCE_SECONDS * TAU
                        - index as f32 * 0.8)
                        .sin();
                    let center = Point::new(
                        bounds.x + DOT / 2.0 + index as f32 * (DOT + DOT_GAP),
                        bounds.center_y() - wave * SWING,
                    );
                    renderer.fill_quad(
                        circle(center, DOT),
                        self.color.scale_alpha(0.75 + 0.25 * wave),
                    );
                }
            }
            Kind::Spinner => {
                let head = seconds / SPINNER_SECONDS * SPINNER_DOTS as f32;
                let middle = bounds.center();
                for index in 0..SPINNER_DOTS {
                    let angle = index as f32 / SPINNER_DOTS as f32 * TAU;
                    let trail =
                        (head - index as f32).rem_euclid(SPINNER_DOTS as f32);
                    let center = Point::new(
                        middle.x + angle.cos() * 6.0,
                        middle.y + angle.sin() * 6.0,
                    );
                    renderer.fill_quad(
                        circle(center, 2.6),
                        self.color.scale_alpha(
                            1.0 - 0.85 * trail / SPINNER_DOTS as f32,
                        ),
                    );
                }
            }
        }
    }
}
