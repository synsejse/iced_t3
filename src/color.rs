//! Design tokens, taken from T3 Code's stylesheet and its rendered dark theme.
use iced::Color;

const fn white(alpha: f32) -> Color {
    Color::from_rgba(1.0, 1.0, 1.0, alpha)
}
pub const BACKGROUND: Color = Color::BLACK;
pub const SIDEBAR: Color = Color::from_rgb8(11, 11, 11);
pub const SIDEBAR_BORDER: Color = Color::from_rgb8(21, 21, 21);
/// Insets such as code blocks.
pub const INSET: Color = Color::from_rgb8(17, 17, 17);
/// Panels, settings groups and the composer.
pub const RAISED: Color = Color::from_rgb8(22, 22, 22);
/// Selected sidebar rows and message bubbles.
pub const SELECTED: Color = Color::from_rgb8(35, 35, 35);
pub const FOREGROUND: Color = Color::from_rgb8(245, 245, 245);
/// Sidebar titles that are not selected.
pub const SUBDUED: Color = Color::from_rgb8(191, 191, 191);
pub const MUTED: Color = Color::from_rgb8(161, 161, 161);
/// Row metadata, a step below [`SUBDUED`].
pub const LABEL: Color = Color::from_rgb8(143, 143, 143);
pub const PRIMARY: Color = Color::from_rgb8(52, 107, 241);
/// Keyboard focus: light enough to show on a [`PRIMARY`] button as well.
pub const FOCUS: Color = Color::from_rgb8(150, 182, 255);
pub const SUCCESS: Color = Color::from_rgb8(0, 212, 146);
pub const WARNING: Color = Color::from_rgb8(255, 185, 0);
pub const DANGER: Color = Color::from_rgb8(255, 100, 103);
pub const VIOLET: Color = Color::from_rgb8(166, 132, 255);
pub const BORDER: Color = white(0.08);
/// Dividers inside raised surfaces and chart grids.
pub const HAIRLINE: Color = white(0.05);
pub const INPUT: Color = white(0.11);
pub const TRACK: Color = white(0.13);
pub const HOVER: Color = white(0.06);
