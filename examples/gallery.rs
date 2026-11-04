//! Every `iced_t3` widget in one window: `cargo run -p iced_t3 --example gallery`.
use iced::widget::{Space, column, container, row, slider, text_input};
use iced::{Element, Fill, Size, Task};
use iced_t3::{Tone, chrome, color, icon, style, widget as t3};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum Page {
    #[default]
    Controls,
    Feedback,
}

#[derive(Default)]
struct Gallery {
    page: Page,
    choice: usize,
    enabled: bool,
    level: f32,
    name: String,
}

#[derive(Debug, Clone)]
enum Message {
    Window(chrome::Action),
    Page(Page),
    Choice(usize),
    Enabled(bool),
    Level(f32),
    Name(String),
    Noop,
}

impl Gallery {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Window(action) => return chrome::perform(action),
            Message::Page(page) => self.page = page,
            Message::Choice(choice) => self.choice = choice,
            Message::Enabled(enabled) => self.enabled = enabled,
            Message::Level(level) => self.level = level,
            Message::Name(name) => self.name = name,
            Message::Noop => (),
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let page = match self.page {
            Page::Controls => self.controls(),
            Page::Feedback => feedback(),
        };
        let title = match self.page {
            Page::Controls => "Controls",
            Page::Feedback => "Feedback",
        };
        let main = column![
            chrome::bar(
                t3::breadcrumb(icon::PALETTE, ["Gallery"], title),
                Some(t3::tooltip(
                    t3::icon_button(
                        "settings",
                        icon::SETTINGS,
                        Some(Message::Noop)
                    ),
                    "Settings",
                    t3::Position::Bottom,
                )),
                Message::Window,
            ),
            t3::scroll(
                container(page.max_width(760))
                    .padding(
                        iced::Padding::from([24.0, t3::PAGE_GUTTER]).top(8)
                    )
                    .center_x(Fill)
            )
            .height(Fill),
            t3::composer(
                row![
                    t3::muted("A composer docks actions below the page.")
                        .width(Fill),
                    t3::ghost(icon::UNDO_2, "Revert", Some(Message::Noop)),
                    t3::primary("Apply", Some(Message::Noop)),
                ]
                .spacing(8)
                .align_y(iced::Center),
                760.0,
                false,
            ),
        ];
        chrome::frame(
            row![self.sidebar(), t3::sidebar_divider(), main],
            Message::Window,
        )
    }

    fn sidebar(&self) -> Element<'_, Message> {
        let nav = |page, id, tile, title, detail: &str| {
            t3::sidebar_row(
                t3::SidebarRow {
                    id,
                    tile,
                    meta: "iced_t3".into(),
                    trailing: "1h".into(),
                    title,
                    detail: Some(detail.into()),
                    selected: self.page == page,
                },
                Some(Message::Page(page)),
            )
        };
        t3::sidebar(
            chrome::drag_region(
                container(t3::section_title("Gallery").size(14))
                    .padding([0, 10])
                    .center_y(44)
                    .width(Fill),
                Message::Window,
            ),
            column![
                nav(
                    Page::Controls,
                    "controls",
                    ("CT", color::PRIMARY),
                    "Controls",
                    "Buttons, switches and sliders",
                ),
                nav(
                    Page::Feedback,
                    "feedback",
                    ("FB", color::VIOLET),
                    "Feedback",
                    "Alerts, badges and empty states",
                ),
                t3::sidebar_group(
                    "Choices",
                    ["One", "Two", "Three"].iter().enumerate().map(
                        |(index, label)| {
                            let selected = self.choice == index;
                            let marker = if index == 2 {
                                t3::dots()
                            } else {
                                t3::radio(selected)
                            };
                            t3::sidebar_choice(
                                format!("choice-{index}"),
                                Some(icon::CIRCLE),
                                *label,
                                marker,
                                selected,
                                Some(Message::Choice(index)),
                            )
                        },
                    ),
                ),
                t3::sidebar_label("Recent", Some("2".into())),
                t3::sidebar_item(
                    "first",
                    t3::tile("A", color::SUCCESS),
                    "First item",
                    "1d",
                    false,
                    Some(Message::Noop),
                ),
                t3::sidebar_item(
                    "second",
                    t3::tile("B", color::WARNING),
                    "Second item",
                    "3d",
                    false,
                    None,
                ),
            ]
            .spacing(2),
            container(t3::muted("Footer")).padding(10),
        )
    }

    fn controls(&self) -> iced::widget::Column<'_, Message> {
        let choices =
            ["One", "Two", "Three"]
                .iter()
                .enumerate()
                .map(|(index, label)| {
                    t3::segment(
                        *label,
                        *label,
                        self.choice == index,
                        Some(Message::Choice(index)),
                    )
                });
        column![
            t3::section(
                t3::section_heading("Settings", "Rows share one panel"),
                Some(t3::switch(
                    "section-switch",
                    Some("Enabled"),
                    self.enabled,
                    Some(Message::Enabled),
                )),
                [
                    t3::setting(
                        "Choice",
                        t3::muted("A segmented control."),
                        t3::segmented(choices),
                    ),
                    t3::setting(
                        "Switch",
                        t3::muted("Without its own label."),
                        t3::switch(
                            "row-switch",
                            None,
                            self.enabled,
                            Some(Message::Enabled),
                        ),
                    ),
                    t3::setting(
                        "Level",
                        t3::muted("Any Iced slider with `style::slider`."),
                        slider(0.0..=100.0, self.level, Message::Level)
                            .width(200)
                            .height(22)
                            .style(style::slider(true)),
                    ),
                    t3::setting(
                        "Name",
                        t3::muted("Any Iced text input with `style::input`."),
                        text_input("Type here", &self.name)
                            .on_input(Message::Name)
                            .padding([6, 10])
                            .line_height(t3::line_height(14))
                            .width(200)
                            .style(style::input),
                    ),
                ],
            ),
            t3::section(
                t3::section_title("Buttons"),
                None,
                [container(
                    row![
                        t3::primary("Primary", Some(Message::Noop)),
                        t3::primary("Disabled", None),
                        t3::primary_small("Small", Some(Message::Noop)),
                        t3::outline(
                            icon::REFRESH_CW,
                            "Outline",
                            Some(Message::Noop)
                        ),
                        t3::ghost(icon::UNDO_2, "Ghost", Some(Message::Noop)),
                        t3::icon_toggle(
                            "mute",
                            if self.enabled {
                                icon::VOLUME_X
                            } else {
                                icon::VOLUME_2
                            },
                            self.enabled,
                            Some(Message::Enabled(!self.enabled)),
                        ),
                    ]
                    .spacing(8)
                    .align_y(iced::Center),
                )
                .padding([12, 16])
                .into()],
            ),
        ]
        .spacing(24)
    }
}

fn feedback<'a>() -> iced::widget::Column<'a, Message> {
    column![
        t3::alert(icon::CIRCLE_ALERT, "Something failed.", Tone::Danger),
        t3::alert(icon::TRIANGLE_ALERT, "Check this first.", Tone::Warning),
        row![
            t3::badge("Live", Tone::Success),
            t3::badge("Updating", Tone::Warning),
            t3::badge("Failed", Tone::Danger),
            t3::badge("Demo", Tone::Neutral),
            t3::dot(Tone::Success),
            Space::new().width(Fill),
        ]
        .spacing(8)
        .align_y(iced::Center),
        row![
            t3::dots(),
            t3::spinner(),
            t3::primary_busy("Saving"),
            t3::primary_small_busy("Apply"),
            Space::new().width(Fill),
        ]
        .spacing(16)
        .align_y(iced::Center),
        t3::empty_state(
            t3::icon_tile(icon::UNPLUG, 36),
            "Nothing here yet",
            ["An empty state fills a page without content."],
        ),
    ]
    .spacing(16)
}

fn main() -> iced::Result {
    iced::application(Gallery::default, Gallery::update, Gallery::view)
        .title("iced_t3 gallery")
        .theme(|_: &Gallery| iced_t3::theme())
        .settings(iced_t3::settings())
        .window(iced::window::Settings {
            size: Size::new(1000.0, 720.0),
            min_size: Some(Size::new(760.0, 480.0)),
            ..chrome::window_settings()
        })
        .run()
}
