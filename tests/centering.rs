//! Text and icons must land on the center of their box. Iced truncates fractional
//! positions, so this only holds while layouts stay on the pixel grid, which
//! `widget::text` and `widget::line_height` are there to ensure.
use iced::advanced::{
    Layout, layout,
    renderer::{Headless, Style},
    widget::Tree,
};
use iced::widget::container;
use iced::{Color, Element, Length, Size};
use iced_t3::{Tone, color, font, icon, widget as t3};

/// Half a pixel: ink an odd number of rows tall can't center in an even box.
const TOLERANCE: f32 = 0.5;
/// An odd box puts the content on a half pixel, which Iced then truncates.
const ODD_BOX_TOLERANCE: f32 = 1.0;

type View = Element<'static, ()>;

struct Ink {
    size: Size<u32>,
    height: f32,
    pixels: Vec<u8>,
}
impl Ink {
    /// Renders `view` at its natural size, or `max` if it fills.
    fn render(mut view: View, max: Size) -> Self {
        for bytes in font::ALL {
            iced::advanced::graphics::text::font_system()
                .write()
                .unwrap()
                .load_font(bytes.into());
        }
        let mut renderer =
            pollster::block_on(<iced::Renderer as Headless>::new(
                font::REGULAR,
                14.0.into(),
                Some("tiny-skia"),
            ))
            .expect("software renderer");
        let mut tree = Tree::new(&view);
        let node = view.as_widget_mut().layout(
            &mut tree,
            &renderer,
            &layout::Limits::new(Size::ZERO, max),
        );
        let bounds = node.size();
        let theme = iced_t3::theme();
        view.as_widget().draw(
            &tree,
            &mut renderer,
            &theme,
            &Style {
                text_color: Color::WHITE,
            },
            Layout::new(&node),
            iced::mouse::Cursor::Unavailable,
            &iced::Rectangle::with_size(bounds),
        );
        let size =
            Size::new(bounds.width.ceil() as u32, bounds.height.ceil() as u32);
        let pixels = renderer.screenshot(size, 1.0, Color::BLACK);
        Self {
            size,
            height: bounds.height,
            pixels,
        }
    }
    /// How far the ink sits from the box's horizontal center.
    fn horizontal_offset(&self, bright: impl Fn(u8, u8, u8) -> bool) -> f32 {
        let (mut left, mut right) = (self.size.width, 0);
        for y in 0..self.size.height {
            for x in 0..self.size.width {
                let i = ((y * self.size.width + x) * 4) as usize;
                if bright(
                    self.pixels[i],
                    self.pixels[i + 1],
                    self.pixels[i + 2],
                ) {
                    left = left.min(x);
                    right = right.max(x + 1);
                }
            }
        }
        assert!(right > left, "no ink found");
        (left + right) as f32 / 2.0 - self.size.width as f32 / 2.0
    }
    /// How far the ink in columns `from..to` sits from the box's vertical center.
    /// `bright` picks ink out of a filled background by its channel values.
    fn vertical_offset(
        &self,
        columns: std::ops::Range<u32>,
        bright: impl Fn(u8, u8, u8) -> bool,
    ) -> f32 {
        let (mut top, mut bottom) = (self.size.height, 0);
        for y in 0..self.size.height {
            for x in columns.clone() {
                let i = ((y * self.size.width + x) * 4) as usize;
                if bright(
                    self.pixels[i],
                    self.pixels[i + 1],
                    self.pixels[i + 2],
                ) {
                    top = top.min(y);
                    bottom = bottom.max(y + 1);
                }
            }
        }
        assert!(bottom > top, "no ink found");
        (top + bottom) as f32 / 2.0 - self.height / 2.0
    }
}

fn check(name: &str, offset: f32, tolerance: f32) {
    assert!(
        offset.abs() <= tolerance,
        "{name} is {offset:+.1}px from center"
    );
}
fn all_columns(ink: &Ink) -> std::ops::Range<u32> {
    0..ink.size.width
}

#[test]
fn text_is_centered_at_every_size_and_weight() {
    for weight in [font::REGULAR, font::MEDIUM, font::SEMIBOLD] {
        for size in [8, 10, 11, 12, 13, 14, 16, 17] {
            for height in [59.0, 60.0] {
                let view = container(
                    t3::text("H", size).font(weight).color(Color::WHITE),
                )
                .center(Length::Fill);
                let ink = Ink::render(view.into(), Size::new(60.0, height));
                check(
                    &format!("{size}px text in a {height}px box"),
                    ink.vertical_offset(all_columns(&ink), |r, _, _| r > 64),
                    if height == 60.0 {
                        TOLERANCE
                    } else {
                        ODD_BOX_TOLERANCE
                    },
                );
            }
        }
    }
}
#[test]
fn icons_are_centered_at_every_size() {
    for size in [14, 15, 16] {
        for height in [59.0, 60.0] {
            let view = container(
                t3::icon(icon::CIRCLE_ALERT).size(size).color(Color::WHITE),
            )
            .center(Length::Fill);
            let ink = Ink::render(view.into(), Size::new(60.0, height));
            check(
                &format!("{size}px icon in a {height}px box"),
                ink.vertical_offset(all_columns(&ink), |r, _, _| r > 64),
                TOLERANCE,
            );
        }
    }
}
#[test]
fn widgets_center_their_labels() {
    let max = Size::new(300.0, 100.0);
    let white = |r: u8, _: u8, _: u8| r > 170;
    let thin = |r: u8, _: u8, _: u8| r > 120;
    let green = |_: u8, g: u8, _: u8| g > 120;
    for (name, view, bright) in [
        (
            "primary",
            t3::primary("HH", Some(())),
            &white as &dyn Fn(u8, u8, u8) -> bool,
        ),
        ("primary_small", t3::primary_small("HH", Some(())), &white),
        ("segment", t3::segment("id", "HH", true, Some(())), &thin),
        ("badge", t3::badge("HH", Tone::Success), &green),
        ("tile", t3::tile("A", color::SUCCESS), &green),
    ] {
        let ink = Ink::render(view, max);
        check(
            name,
            ink.vertical_offset(all_columns(&ink), bright),
            TOLERANCE,
        );
    }
}
#[test]
fn square_widgets_center_both_ways() {
    let max = Size::new(100.0, 100.0);
    let tile = Ink::render(t3::tile("A", color::SUCCESS), max);
    let green = |_: u8, g: u8, _: u8| g > 120;
    check("tile, across", tile.horizontal_offset(green), TOLERANCE);
    let button = Ink::render(t3::icon_button("id", icon::X, Some(())), max);
    let bright = |r: u8, _: u8, _: u8| r > 90;
    check(
        "icon button, across",
        button.horizontal_offset(bright),
        TOLERANCE,
    );
    check(
        "icon button, down",
        button.vertical_offset(all_columns(&button), bright),
        TOLERANCE,
    );
}
#[test]
fn alert_centers_its_icon_and_message() {
    let view = t3::alert(icon::CIRCLE_ALERT, "HH", Tone::Danger);
    let ink = Ink::render(view, Size::new(300.0, 100.0));
    let bright = |r: u8, _: u8, _: u8| r > 170;
    check("alert icon", ink.vertical_offset(12..34, bright), TOLERANCE);
    check(
        "alert message",
        ink.vertical_offset(38..80, bright),
        TOLERANCE,
    );
}
#[test]
fn sidebar_rows_center_their_contents() {
    let view = t3::sidebar_nav("id", icon::HEADPHONES, "HH", true, Some(()));
    let ink = Ink::render(view, Size::new(240.0, 100.0));
    let bright = |r: u8, _: u8, _: u8| r > 120;
    check("nav icon", ink.vertical_offset(8..28, bright), TOLERANCE);
    check("nav label", ink.vertical_offset(34..80, bright), TOLERANCE);
}
#[test]
fn dots_move_around_the_center_of_a_button() {
    // They are mid-bounce, so allow for the swing itself.
    let view = t3::primary_busy("Apply EQ");
    let ink = Ink::render(view, Size::new(300.0, 100.0));
    check(
        "busy dots",
        ink.vertical_offset(all_columns(&ink), |r, _, _| r > 170),
        1.5,
    );
}
