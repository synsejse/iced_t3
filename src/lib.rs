//! T3 Code's dark design language for Iced 0.14.
//!
//! ```no_run
//! use iced::widget::column;
//! use iced::{Element, Task};
//! use iced_t3::{chrome, widget as t3};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Window(chrome::Action),
//!     Save,
//! }
//!
//! fn update(_: &mut (), message: Message) -> Task<Message> {
//!     match message {
//!         Message::Window(action) => chrome::perform(action),
//!         Message::Save => Task::none(),
//!     }
//! }
//!
//! fn view(_: &()) -> Element<'_, Message> {
//!     let page = column![
//!         chrome::titlebar("My app", None, Message::Window),
//!         t3::primary("Save", Some(Message::Save)),
//!     ];
//!     chrome::frame(page, Message::Window)
//! }
//!
//! fn main() -> iced::Result {
//!     iced::application(|| (), update, view)
//!         .theme(|_: &()| iced_t3::theme())
//!         .settings(iced_t3::settings())
//!         .window(chrome::window_settings())
//!         .run()
//! }
//! ```
pub mod chrome;
pub mod color;
mod focus;
pub mod font;
pub mod icon;
mod loader;
pub mod style;
pub mod widget;

pub use focus::focusable;
pub use style::Tone;

/// The Iced theme. Widgets without a style from [`style`] pick up these colors.
pub fn theme() -> iced::Theme {
    iced::Theme::custom(
        "T3",
        iced::theme::Palette {
            background: color::BACKGROUND,
            text: color::FOREGROUND,
            primary: color::PRIMARY,
            success: color::SUCCESS,
            warning: color::WARNING,
            danger: color::DANGER,
        },
    )
}

/// Application settings that load the bundled fonts and make Inter the default.
pub fn settings() -> iced::Settings {
    iced::Settings {
        default_font: font::REGULAR,
        default_text_size: 14.0.into(),
        fonts: font::ALL
            .iter()
            .map(|bytes| std::borrow::Cow::Borrowed(*bytes))
            .collect(),
        ..Default::default()
    }
}
