//! Interactive component catalogue: cargo run --example gallery.
use iced::widget::{Column, Space, column, container, row, slider};
use iced::{Element, Fill, Size, Task};
use iced_t3::{Tone, chrome, color, icon, style, widget as t3};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum Page {
    #[default]
    Buttons,
    Forms,
    Navigation,
    Surfaces,
    Feedback,
    Typography,
}
impl Page {
    const ALL: [Self; 6] = [
        Self::Buttons,
        Self::Forms,
        Self::Navigation,
        Self::Surfaces,
        Self::Feedback,
        Self::Typography,
    ];
    fn title(self) -> &'static str {
        match self {
            Self::Buttons => "Buttons",
            Self::Forms => "Forms",
            Self::Navigation => "Navigation",
            Self::Surfaces => "Surfaces",
            Self::Feedback => "Feedback",
            Self::Typography => "Typography",
        }
    }
}
struct Gallery {
    page: Page,
    choice: usize,
    enabled: bool,
    expanded: bool,
    level: f32,
    name: String,
    notice: bool,
    modal: bool,
    clicks: u32,
    dialog_confirm: bool,
}
impl Default for Gallery {
    fn default() -> Self {
        Self {
            page: Page::default(),
            choice: 0,
            enabled: true,
            expanded: true,
            level: 42.0,
            name: String::new(),
            notice: true,
            modal: false,
            clicks: 0,
            dialog_confirm: true,
        }
    }
}
#[derive(Debug, Clone)]
enum Message {
    Window(chrome::Action),
    Page(Page),
    Choice(usize),
    Enabled(bool),
    Expanded,
    Level(f32),
    Name(String),
    Notice,
    Modal(bool),
    Click,
    Tab(bool),
    Escape,
}
const TONES: [(Tone, &str); 4] = [
    (Tone::Neutral, "Neutral"),
    (Tone::Success, "Success"),
    (Tone::Warning, "Warning"),
    (Tone::Danger, "Danger"),
];
fn group<'a>(
    title: &'a str,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    column![t3::section_title(title), content.into()]
        .spacing(12)
        .into()
}
impl Gallery {
    fn subscription(&self) -> iced::Subscription<Message> {
        iced::event::listen_with(|event, _, _| match event {
            iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                key,
                modifiers,
                ..
            }) => match key {
                iced::keyboard::Key::Named(iced::keyboard::key::Named::Tab) => {
                    Some(Message::Tab(modifiers.shift()))
                }
                iced::keyboard::Key::Named(
                    iced::keyboard::key::Named::Escape,
                ) => Some(Message::Escape),
                _ => None,
            },
            _ => None,
        })
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Window(action) => return chrome::perform(action),
            Message::Page(page) => {
                self.page = page;
                self.notice = true;
                return iced::widget::operation::snap_to(
                    "gallery-scroll",
                    iced::widget::scrollable::RelativeOffset::START,
                );
            }
            Message::Choice(choice) => self.choice = choice,
            Message::Enabled(value) => self.enabled = value,
            Message::Expanded => self.expanded = !self.expanded,
            Message::Level(value) => self.level = value,
            Message::Name(value) => self.name = value,
            Message::Notice => self.notice = !self.notice,
            Message::Modal(value) => {
                self.modal = value;
                self.dialog_confirm = true;
                return iced::widget::operation::focus(if value {
                    "Confirm"
                } else {
                    "Open dialog"
                });
            }
            Message::Tab(reverse) => {
                if self.modal {
                    self.dialog_confirm = !self.dialog_confirm;
                    return iced::widget::operation::focus(
                        if self.dialog_confirm {
                            "Confirm"
                        } else {
                            "Cancel"
                        },
                    );
                }
                return if reverse {
                    iced::widget::operation::focus_previous()
                } else {
                    iced::widget::operation::focus_next()
                };
            }
            Message::Escape => {
                if self.modal {
                    self.modal = false;
                    return iced::widget::operation::focus("Open dialog");
                }
            }
            Message::Click => self.clicks += 1,
        }
        Task::none()
    }
    fn view(&self) -> Element<'_, Message> {
        let nav = Column::with_children(Page::ALL.map(|page| {
            t3::sidebar_nav(
                format!("page-{page:?}"),
                icon::LAYOUT_GRID,
                page.title(),
                page == self.page,
                Some(Message::Page(page)),
            )
        }))
        .spacing(4);
        let sidebar = t3::sidebar(
            chrome::drag_region(
                container(t3::section_title("iced_t3"))
                    .padding(12)
                    .height(44),
                Message::Window,
            ),
            column![
                t3::sidebar_label("COMPONENT GALLERY", Some("6".into())),
                nav
            ],
            t3::sidebar_device(
                "gallery-account",
                t3::SidebarDevice {
                    media: t3::avatar("UI", color::PRIMARY),
                    title: "Component library".into(),
                    detail: "Iced 0.14 · Inter · Lucide".into(),
                    selected: false,
                },
                None,
            ),
        );
        let page = match self.page {
            Page::Buttons => self.buttons(),
            Page::Forms => self.forms(),
            Page::Navigation => self.navigation(),
            Page::Surfaces => self.surfaces(),
            Page::Feedback => self.feedback(),
            Page::Typography => self.typography(),
        };
        let main = column![
            chrome::bar(
                t3::breadcrumb(icon::PALETTE, ["Gallery"], self.page.title()),
                Some(t3::tooltip(
                    t3::icon_button("help", icon::INFO, Some(Message::Click)),
                    "Keyboard: Tab, Enter and Space",
                    t3::Position::Bottom
                )),
                Message::Window
            ),
            t3::scroll(
                container(page.width(Fill).max_width(880))
                    .padding([24, 32])
                    .center_x(Fill)
            )
            .id("gallery-scroll")
            .height(Fill),
            t3::composer(
                row![
                    t3::muted(format!(
                        "{} actions · Tab to explore controls",
                        self.clicks
                    ))
                    .width(Fill),
                    t3::primary("Open dialog", Some(Message::Modal(true)))
                ]
                .spacing(12)
                .align_y(iced::Center),
                880.0,
                self.enabled
            ),
        ];
        let base: Element<'_, Message> =
            row![sidebar, t3::sidebar_divider(), main].into();
        let content = if self.modal {
            t3::modal(
                base,
                t3::dialog(
                    "Confirm action",
                    "A dialog blocks the page until dismissed.",
                    [
                        t3::ghost(
                            icon::X,
                            "Cancel",
                            Some(Message::Modal(false)),
                        ),
                        t3::primary("Confirm", Some(Message::Modal(false))),
                    ],
                ),
                Message::Modal(false),
            )
        } else {
            base
        };
        chrome::frame(content, Message::Window)
    }

    fn buttons(&self) -> Column<'_, Message> {
        let mut page = column![t3::section_heading(
            "Actions",
            "Enabled, disabled and busy"
        )]
        .spacing(24);
        for enabled in [true, false] {
            let press = enabled.then_some(Message::Click);
            page = page.push(group(
                if enabled { "Enabled" } else { "Disabled" },
                column![
                    row![
                        t3::primary("Primary", press.clone()),
                        t3::primary_small("Small", press.clone()),
                        t3::danger("Danger", press.clone())
                    ]
                    .spacing(12),
                    row![
                        t3::outline(icon::PLUS, "Outline", press.clone()),
                        t3::ghost(icon::COPY, "Ghost", press.clone()),
                        t3::ghost_danger(
                            icon::TRASH_2,
                            "Delete",
                            press.clone()
                        )
                    ]
                    .spacing(12),
                    row![
                        t3::icon_button(
                            format!("icon-{enabled}"),
                            icon::SETTINGS,
                            press.clone()
                        ),
                        t3::icon_toggle(
                            format!("toggle-off-{enabled}"),
                            icon::VOLUME_2,
                            false,
                            press.clone()
                        ),
                        t3::icon_toggle(
                            format!("toggle-on-{enabled}"),
                            icon::VOLUME_X,
                            true,
                            press.clone()
                        ),
                        t3::icon_choice(
                            format!("choice-off-{enabled}"),
                            icon::LOCK_OPEN,
                            false,
                            press.clone()
                        ),
                        t3::icon_choice(
                            format!("choice-on-{enabled}"),
                            icon::LOCK,
                            true,
                            press.clone()
                        ),
                        t3::swatch(
                            format!("swatch-off-{enabled}"),
                            color::VIOLET,
                            false,
                            press.clone()
                        ),
                        t3::swatch(
                            format!("swatch-on-{enabled}"),
                            color::SUCCESS,
                            true,
                            press
                        )
                    ]
                    .spacing(12)
                    .align_y(iced::Center),
                ]
                .spacing(12),
            ));
        }
        page.push(group(
            "Busy",
            row![t3::primary_busy("Saving"), t3::primary_small_busy("Saving")]
                .spacing(12),
        ))
    }
    fn forms(&self) -> Column<'_, Message> {
        let mut page =
            column![t3::section_heading("Forms", "Edits update shared state")]
                .spacing(24);
        for enabled in [true, false] {
            let change = enabled.then_some(Message::Enabled);
            page = page.push(group(
                if enabled {
                    "Enabled controls"
                } else {
                    "Disabled controls"
                },
                column![
                    row![
                        t3::checkbox(
                            format!("check-off-{enabled}"),
                            "Unchecked",
                            false,
                            change
                        ),
                        t3::checkbox(
                            format!("check-on-{enabled}"),
                            "Checked",
                            true,
                            change
                        ),
                        t3::switch(
                            format!("switch-off-{enabled}"),
                            Some("Off"),
                            false,
                            change
                        ),
                        t3::switch(
                            format!("switch-on-{enabled}"),
                            Some("On"),
                            true,
                            change
                        )
                    ]
                    .spacing(12)
                    .align_y(iced::Center),
                    t3::input(
                        "Name",
                        &self.name,
                        enabled.then_some(Message::Name)
                    ),
                    t3::search(
                        "Search components",
                        &self.name,
                        enabled.then_some(Message::Name)
                    ),
                ]
                .spacing(12),
            ));
        }
        page.push(t3::section(
            t3::section_heading("Settings", "Shared panel"),
            None,
            [
                t3::setting(
                    "Notifications",
                    t3::muted("A labelled setting row."),
                    t3::switch(
                        "setting-switch",
                        None,
                        self.enabled,
                        Some(Message::Enabled),
                    ),
                ),
                t3::setting_titled(
                    "Level",
                    iced_t3::cancellable(
                        slider(0.0..=100.0, self.level, Message::Level)
                            .width(240)
                            .height(22)
                            .style(style::slider(true)),
                    ),
                ),
                t3::setting_titled(
                    "Segmented",
                    t3::segmented((0..3).map(|i| {
                        t3::segment(
                            format!("segment-{i}"),
                            ["One", "Two", "Disabled"][i],
                            self.choice == i,
                            (i < 2).then_some(Message::Choice(i)),
                        )
                    })),
                ),
            ],
        ))
        .push(t3::checkbox(
            "interactive-check",
            "Interactive checkbox",
            self.enabled,
            Some(Message::Enabled),
        ))
    }
    fn navigation(&self) -> Column<'_, Message> {
        column![
            group(
                "Tabs",
                t3::tabs((0..3).map(|i| t3::tab(
                    format!("tab-{i}"),
                    ["Overview", "Details", "Disabled"][i],
                    self.choice == i,
                    (i < 2).then_some(Message::Choice(i))
                )))
            ),
            t3::tab(
                "tab-disabled-selected",
                "Selected disabled tab",
                true,
                None
            ),
            t3::text(format!("Selected page: {}", self.choice + 1), 14),
            group(
                "Chips",
                row![
                    t3::chip(
                        "chip-a",
                        "First",
                        self.choice == 0,
                        Some(Message::Choice(0))
                    ),
                    t3::chip(
                        "chip-b",
                        "Second",
                        self.choice == 1,
                        Some(Message::Choice(1))
                    ),
                    t3::chip("chip-disabled", "Disabled", false, None),
                    t3::chip(
                        "chip-disabled-selected",
                        "Selected disabled",
                        true,
                        None
                    ),
                ]
                .spacing(8)
            ),
            group(
                "Disabled navigation",
                column![
                    t3::sidebar_nav(
                        "nav-disabled",
                        icon::FOLDER,
                        "Unavailable page",
                        false,
                        None
                    ),
                    t3::sidebar_nav(
                        "nav-selected-disabled",
                        icon::FOLDER,
                        "Selected unavailable page",
                        true,
                        None
                    ),
                ]
            ),
            group(
                "Thread rows",
                column![
                    t3::sidebar_row(
                        t3::SidebarRow {
                            id: "thread".into(),
                            tile: ("UI", color::VIOLET),
                            meta: "Library".into(),
                            trailing: "2m".into(),
                            title: "Design system",
                            detail: Some("Selected thread with detail".into()),
                            selected: true
                        },
                        Some(Message::Click)
                    ),
                    t3::sidebar_row(
                        t3::SidebarRow {
                            id: "thread-disabled".into(),
                            tile: ("DS", color::PRIMARY),
                            meta: "Library".into(),
                            trailing: "1d".into(),
                            title: "Disabled thread",
                            detail: None,
                            selected: false
                        },
                        None
                    ),
                    t3::sidebar_item(
                        "item",
                        t3::SidebarItem {
                            media: t3::tile("A", color::SUCCESS),
                            title: "Settled item".into(),
                            trailing: "1d".into(),
                            selected: false,
                        },
                        Some(Message::Click)
                    ),
                    t3::sidebar_item(
                        "item-disabled",
                        t3::SidebarItem {
                            media: t3::tile("B", color::WARNING),
                            title: "Disabled item".into(),
                            trailing: "2d".into(),
                            selected: true,
                        },
                        None
                    ),
                ]
                .spacing(4)
            ),
            t3::sidebar_group(
                "Choices",
                (0..4).map(|i| t3::sidebar_choice(
                    format!("side-choice-{i}"),
                    t3::SidebarChoice {
                        glyph: if i == 0 { None } else { Some(icon::CIRCLE) },
                        label: ["Radio", "Busy", "Confirmed", "Disabled"][i]
                            .into(),
                        marker: match i {
                            1 => t3::dots(),
                            2 => t3::check(),
                            _ => t3::radio(self.choice == i),
                        },
                        selected: self.choice == i,
                    },
                    (i < 3).then_some(Message::Choice(i))
                ))
            ),
            group(
                "Accounts",
                column![
                    t3::sidebar_device(
                        "account-active",
                        t3::SidebarDevice {
                            media: t3::avatar("AB", color::VIOLET),
                            title: "Selected account".into(),
                            detail: "Click to select".into(),
                            selected: true
                        },
                        Some(Message::Click)
                    ),
                    t3::sidebar_device(
                        "account-disabled",
                        t3::SidebarDevice {
                            media: t3::icon_tile(
                                icon::USER,
                                t3::TileSize::Compact
                            ),
                            title: "Disabled account".into(),
                            detail: "Unavailable".into(),
                            selected: false
                        },
                        None
                    ),
                ]
            ),
        ]
        .spacing(24)
    }
    fn surfaces(&self) -> Column<'_, Message> {
        column![
            group(
                "Stats",
                row![
                    t3::stat("Items", "128", "Across all projects"),
                    t3::stat("Completed", "86%", "This week")
                ]
                .spacing(16)
            ),
            group(
                "Card",
                t3::card(
                    column![
                        t3::section_title("A flexible surface"),
                        t3::muted("Cards accept any content."),
                        t3::primary("Action", Some(Message::Click))
                    ]
                    .spacing(12)
                )
                .width(Fill)
            ),
            group(
                "List rows",
                column![
                    t3::list_row(
                        "list-header",
                        [
                            t3::text("Name", 12).width(Fill).into(),
                            t3::muted("Status").into()
                        ],
                        false,
                        None
                    ),
                    t3::list_row(
                        "list-one",
                        [
                            t3::text("First item", 14).width(Fill).into(),
                            t3::badge("Ready", Tone::Success)
                        ],
                        self.choice == 0,
                        Some(Message::Choice(0))
                    ),
                    t3::list_row(
                        "list-two",
                        [
                            t3::text("Second item", 14).width(Fill).into(),
                            t3::badge("Pending", Tone::Warning)
                        ],
                        self.choice == 1,
                        Some(Message::Choice(1))
                    ),
                    t3::list_row(
                        "list-disabled",
                        [t3::text("Unavailable", 14).into()],
                        true,
                        None
                    ),
                ]
            ),
            t3::accordion(
                "accordion",
                "Expandable details",
                self.expanded,
                t3::muted("This content follows the header in Tab order."),
                Some(Message::Expanded)
            ),
            t3::accordion(
                "accordion-closed",
                "Alternate section",
                !self.expanded,
                t3::muted("Additional details."),
                Some(Message::Expanded)
            ),
            t3::accordion(
                "accordion-disabled",
                "Disabled",
                false,
                t3::muted("Hidden"),
                None
            ),
            group(
                "Skeleton",
                row![
                    t3::skeleton(48, 48),
                    column![t3::skeleton(240, 16), t3::skeleton(160, 12)]
                        .spacing(12)
                ]
                .spacing(16)
            ),
        ]
        .spacing(24)
    }
    fn feedback(&self) -> Column<'_, Message> {
        let mut page =
            column![t3::section_heading("Feedback", "All semantic tones")]
                .spacing(16);
        for (tone, label) in TONES {
            page = page
                .push(
                    row![
                        t3::badge(label, tone),
                        t3::dot(tone),
                        t3::progress(self.level / 100.0, tone)
                    ]
                    .spacing(12)
                    .align_y(iced::Center),
                )
                .push(t3::alert(
                    icon::INFO,
                    format!("{label}: a neutral example message."),
                    tone,
                ));
        }
        if self.notice {
            for (tone, label) in TONES {
                page = page.push(t3::alert_dismissible(
                    format!("notice-{label}"),
                    icon::INFO,
                    format!("{label}: dismiss this notice group."),
                    tone,
                    Message::Notice,
                ));
            }
        }
        page.push(t3::ghost(
            icon::REFRESH_CW,
            "Toggle notice",
            Some(Message::Notice),
        ))
        .push(
            row![
                t3::dots(),
                t3::dots_in(color::PRIMARY),
                t3::spinner(),
                t3::check(),
                t3::radio(false),
                t3::radio(true)
            ]
            .spacing(16)
            .align_y(iced::Center),
        )
        .push(group(
            "Progress endpoints",
            column![
                t3::progress(0.0, Tone::Neutral),
                t3::progress(1.0, Tone::Success)
            ]
            .spacing(8),
        ))
        .push(t3::empty_state(
            t3::icon_tile(icon::FOLDER, t3::TileSize::Standard),
            "Nothing here yet",
            ["Create an item to get started."],
        ))
    }
    fn typography(&self) -> Column<'_, Message> {
        column![
            chrome::titlebar("Standalone titlebar", None, Message::Window),
            group("Text", column![t3::text("Inter · Regular", 24), t3::section_title("Section title"),
                t3::muted("Muted supporting text"), t3::warning("Warning text")].spacing(12)),
            t3::divider(),
            group("Icons and media", row![t3::icon(icon::PALETTE),
                t3::icon_tile(icon::FOLDER, t3::TileSize::Compact), t3::icon_tile(icon::FOLDER, t3::TileSize::Standard),
                t3::icon_tile(icon::FOLDER, t3::TileSize::Large), t3::tile("UI", color::PRIMARY),
                t3::avatar("AB", color::VIOLET), t3::shortcut("Ctrl"), t3::shortcut("K")
            ].spacing(16).align_y(iced::Center)),
            t3::breadcrumb(icon::FOLDER, ["Library", "Components"], "Typography"),
            t3::card(column![t3::text("Window controls", 14), chrome::controls(Message::Window)].spacing(12)),
            Space::new().height(16),
            t3::muted("Tooltips appear on hover. The footer opens an interactive modal."),
        ].spacing(24)
    }
}
fn main() -> iced::Result {
    iced::application(
        || {
            let mut gallery = Gallery::default();
            if let Some(name) = std::env::args().nth(1)
                && let Some(page) = Page::ALL
                    .into_iter()
                    .find(|page| page.title().eq_ignore_ascii_case(&name))
            {
                gallery.page = page;
            }
            gallery
        },
        Gallery::update,
        Gallery::view,
    )
    .subscription(Gallery::subscription)
    .title("iced_t3 gallery")
    .theme(|_: &Gallery| iced_t3::theme())
    .settings(iced_t3::settings_for("iced-t3-gallery"))
    .window(iced::window::Settings {
        size: Size::new(1200.0, 1000.0),
        min_size: Some(Size::new(900.0, 640.0)),
        ..chrome::window_settings_for("iced-t3-gallery")
    })
    .run()
}

#[cfg(test)]
mod render_tests {
    use super::*;
    use iced::advanced::{
        Layout, layout,
        renderer::{Headless, Style},
        widget::Tree,
    };

    fn render(app: &Gallery, width: u32, height: u32, name: &str) {
        for bytes in iced_t3::font::ALL {
            iced::advanced::graphics::text::font_system()
                .write()
                .unwrap()
                .load_font(bytes.into());
        }
        let mut renderer =
            pollster::block_on(<iced::Renderer as Headless>::new(
                iced_t3::font::REGULAR,
                14.into(),
                Some("tiny-skia"),
            ))
            .unwrap();
        let mut view = app.view();
        let mut tree = Tree::new(&view);
        let size = Size::new(width as f32, height as f32);
        let node = view.as_widget_mut().layout(
            &mut tree,
            &renderer,
            &layout::Limits::new(size, size),
        );
        view.as_widget_mut().update(
            &mut tree,
            &iced::Event::Window(iced::window::Event::RedrawRequested(
                std::time::Instant::now(),
            )),
            Layout::new(&node),
            iced::mouse::Cursor::Unavailable,
            &renderer,
            &mut iced::advanced::clipboard::Null,
            &mut iced::advanced::Shell::new(&mut Vec::new()),
            &iced::Rectangle::with_size(size),
        );
        view.as_widget().draw(
            &tree,
            &mut renderer,
            &iced_t3::theme(),
            &Style {
                text_color: color::FOREGROUND,
            },
            Layout::new(&node),
            iced::mouse::Cursor::Unavailable,
            &iced::Rectangle::with_size(size),
        );
        let pixels = renderer.screenshot(
            Size::new(width, height),
            1.0,
            color::BACKGROUND,
        );
        assert_eq!(pixels.len(), (width * height * 4) as usize);
        if let Ok(directory) = std::env::var("GALLERY_CAPTURE_DIR") {
            std::fs::create_dir_all(&directory).unwrap();
            image::save_buffer(
                format!("{directory}/{name}-{width}x{height}.png"),
                &pixels,
                width,
                height,
                image::ColorType::Rgba8,
            )
            .unwrap();
        }
    }
    #[test]
    fn every_page_and_modal_render_at_two_sizes() {
        for page in Page::ALL {
            let app = Gallery {
                page,
                ..Gallery::default()
            };
            for (width, height) in [(1200, 1100), (900, 640)] {
                render(&app, width, height, page.title());
            }
        }
        render(
            &Gallery {
                page: Page::Forms,
                name: "Example text".into(),
                enabled: false,
                ..Gallery::default()
            },
            1200,
            1100,
            "Forms-filled",
        );
        render(
            &Gallery {
                page: Page::Surfaces,
                expanded: false,
                ..Gallery::default()
            },
            1200,
            1100,
            "Surfaces-collapsed",
        );
        render(
            &Gallery {
                page: Page::Feedback,
                ..Gallery::default()
            },
            1200,
            1500,
            "Feedback-full",
        );
        render(
            &Gallery {
                modal: true,
                ..Gallery::default()
            },
            1200,
            1000,
            "Dialog",
        );
    }
}
