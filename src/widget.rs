//! Ready-made controls and layouts. Interactive widgets take an
//! `Option<Message>`: `None` draws them disabled and removes them from Tab order.
use crate::{Tone, color, focusable, font, style};
use iced::advanced::text::IntoFragment;
use iced::advanced::widget::Id;
use iced::widget::text::LineHeight;
use iced::widget::{
    Column, Row, Scrollable, Space, Text, button as iced_button, column,
    container, mouse_area, opaque, row, scrollable, stack, text as iced_text,
    toggler, tooltip as iced_tooltip,
};
use iced::{Color, Element, Length, Padding};

pub use crate::components::*;
pub use crate::loader::{dots, dots_in, spinner};
pub use iced::widget::tooltip::Position;

/// Side padding of a page, so that a [`composer`] below it can overhang it.
pub const PAGE_GUTTER: f32 = 32.0;
const COMPOSER_GUTTER: f32 = 12.0;

/// A line height that keeps layouts on the pixel grid: a whole, even number near
/// Iced's default of 1.3 times the size. Iced truncates fractional positions, so
/// fractional or odd heights leave neighbours a pixel or more out of line.
pub fn line_height(size: u32) -> LineHeight {
    let half = (size as f32 * 1.3 / 2.0).round();
    LineHeight::Absolute((half * 2.0).into())
}
/// Text on the pixel grid. Use this rather than Iced's `text`, or set its
/// `line_height` from [`line_height`].
pub fn text<'a>(content: impl IntoFragment<'a>, size: u32) -> Text<'a> {
    iced_text(content).size(size).line_height(line_height(size))
}
/// A Lucide glyph from [`crate::icon`], at 16px unless resized. Keep sizes even:
/// a glyph is as wide as it is tall, so an odd one can't center in an even box.
pub fn icon<'a>(glyph: char) -> Text<'a> {
    text(glyph.to_string(), 16).font(font::ICONS)
}
/// Secondary text at 12px.
pub fn muted<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    text(content, 12).color(color::MUTED)
}
/// Secondary text that needs attention.
pub fn warning<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    text(content, 12).color(color::WARNING)
}
/// A horizontal rule inside a raised surface.
pub fn divider<'a, Message: 'a>() -> Element<'a, Message> {
    container(Space::new())
        .height(1)
        .width(Length::Fill)
        .style(style::divider(color::HAIRLINE))
        .into()
}
/// The rule between the sidebar and the page.
pub fn sidebar_divider<'a, Message: 'a>() -> Element<'a, Message> {
    container(Space::new())
        .width(1)
        .height(Length::Fill)
        .style(style::sidebar_divider)
        .into()
}

fn text_button<'a, Message: Clone + 'a>(
    label: String,
    size: u32,
    padding: [u16; 2],
    on_press: Option<Message>,
    style: fn(&iced::Theme, iced_button::Status) -> iced_button::Style,
) -> Element<'a, Message> {
    focusable(
        label.clone(),
        iced_button(text(label, size).font(font::MEDIUM))
            .padding(padding)
            .style(style)
            .on_press_maybe(on_press.clone()),
        on_press,
    )
}
fn busy_button<'a, Message: Clone + 'a>(
    label: String,
    size: u32,
    padding: [u16; 2],
) -> Element<'a, Message> {
    // The hidden label keeps the button's width, so nothing shifts.
    iced_button(stack![
        text(label, size)
            .font(font::MEDIUM)
            .color(Color::TRANSPARENT),
        container(dots_in(Color::WHITE)).center(Length::Fill),
    ])
    .padding(padding)
    .style(style::primary)
    .into()
}
/// The main action of a surface.
pub fn primary<'a, Message: Clone + 'a>(
    label: impl Into<String>,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    text_button(label.into(), 13, [6, 12], on_press, style::primary)
}
/// A destructive action, such as the confirmation in a [`dialog`].
pub fn danger<'a, Message: Clone + 'a>(
    label: impl Into<String>,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    text_button(label.into(), 13, [6, 12], on_press, style::danger)
}
/// [`primary`] while its action runs: dots replace the label.
pub fn primary_busy<'a, Message: Clone + 'a>(
    label: impl Into<String>,
) -> Element<'a, Message> {
    busy_button(label.into(), 13, [6, 12])
}
/// [`primary`] for dense rows.
pub fn primary_small<'a, Message: Clone + 'a>(
    label: impl Into<String>,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    text_button(label.into(), 12, [4, 10], on_press, style::primary)
}
/// [`primary_small`] while its action runs.
pub fn primary_small_busy<'a, Message: Clone + 'a>(
    label: impl Into<String>,
) -> Element<'a, Message> {
    busy_button(label.into(), 12, [4, 10])
}
fn icon_label<'a, Message: Clone + 'a>(
    glyph: char,
    label: String,
    on_press: Option<Message>,
    style: fn(&iced::Theme, iced_button::Status) -> iced_button::Style,
) -> Element<'a, Message> {
    focusable(
        label.clone(),
        iced_button(
            row![
                icon(glyph).size(14).line_height(line_height(13)),
                text(label, 13).font(font::MEDIUM)
            ]
            .spacing(6)
            .align_y(iced::Center),
        )
        .padding([6, 12])
        .style(style)
        .on_press_maybe(on_press.clone()),
        on_press,
    )
}
pub fn outline<'a, Message: Clone + 'a>(
    glyph: char,
    label: impl Into<String>,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    icon_label(glyph, label.into(), on_press, style::outline)
}
pub fn ghost<'a, Message: Clone + 'a>(
    glyph: char,
    label: impl Into<String>,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    icon_label(glyph, label.into(), on_press, style::ghost)
}
/// A destructive action that isn't the main one on its surface.
pub fn ghost_danger<'a, Message: Clone + 'a>(
    glyph: char,
    label: impl Into<String>,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    icon_label(glyph, label.into(), on_press, style::ghost_danger)
}
fn square_button<'a, Message: Clone + 'a>(
    id: impl Into<Id>,
    glyph: char,
    on_press: Option<Message>,
    style: impl Fn(&iced::Theme, iced_button::Status) -> iced_button::Style + 'a,
) -> Element<'a, Message> {
    focusable(
        id,
        iced_button(container(icon(glyph)).center_x(26).center_y(26))
            .padding(0)
            .style(style)
            .on_press_maybe(on_press.clone()),
        on_press,
    )
}
/// A square toolbar button. Pair it with [`tooltip`] to name it.
pub fn icon_button<'a, Message: Clone + 'a>(
    id: impl Into<Id>,
    glyph: char,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    square_button(id, glyph, on_press, style::ghost_muted)
}
/// An [`icon_button`] that holds a state, such as mute. `active` tints it red,
/// so swap `glyph` too rather than relying on color alone.
pub fn icon_toggle<'a, Message: Clone + 'a>(
    id: impl Into<Id>,
    glyph: char,
    active: bool,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    square_button(id, glyph, on_press, style::icon_toggle(active))
}
/// [`icon_toggle`] for a choice that isn't a warning, such as linking two values.
/// `active` tints it blue, so swap `glyph` too.
pub fn icon_choice<'a, Message: Clone + 'a>(
    id: impl Into<Id>,
    glyph: char,
    active: bool,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    square_button(id, glyph, on_press, style::icon_choice(active))
}
/// A round color choice, ringed while `selected`.
pub fn swatch<'a, Message: Clone + 'a>(
    id: impl Into<Id>,
    color: Color,
    selected: bool,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    focusable(
        id,
        iced_button(Space::new().width(20).height(20))
            .padding(0)
            .style(style::swatch(color, selected))
            .on_press_maybe(on_press.clone()),
        on_press,
    )
}
/// A switch, with its own label when no [`setting`] row names it.
pub fn switch<'a, Message: Clone + 'a>(
    id: impl Into<Id>,
    label: Option<&'a str>,
    value: bool,
    on_toggle: Option<impl Fn(bool) -> Message + 'a>,
) -> Element<'a, Message> {
    let keyboard = on_toggle.as_ref().map(|change| change(!value));
    let mut control = toggler(value)
        .size(18)
        .style(style::switch)
        .on_toggle_maybe(on_toggle);
    if let Some(label) = label {
        control = control
            .label(label)
            .font(font::MEDIUM)
            .text_size(12)
            .text_line_height(line_height(12))
            .spacing(8);
    }
    focusable(id, control, keyboard)
}
/// One choice of a [`segmented`] control.
pub fn segment<'a, Message: Clone + 'a>(
    id: impl Into<Id>,
    label: impl Into<String>,
    selected: bool,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    focusable(
        id,
        iced_button(text(label.into(), 12).font(font::MEDIUM))
            .padding([4, 10])
            .style(style::segment(selected))
            .on_press_maybe(on_press.clone()),
        on_press,
    )
}
pub fn segmented<'a, Message: 'a>(
    segments: impl IntoIterator<Item = Element<'a, Message>>,
) -> Element<'a, Message> {
    container(Row::with_children(segments).spacing(2))
        .padding(2)
        .style(style::segmented)
        .into()
}
pub fn tooltip<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    tip: impl IntoFragment<'a>,
    position: Position,
) -> Element<'a, Message> {
    iced_tooltip(
        content,
        container(text(tip, 12))
            .padding([6, 8])
            .style(style::popover),
        position,
    )
    .gap(6)
    .into()
}
pub fn badge<'a, Message: 'a>(
    label: impl IntoFragment<'a>,
    tone: Tone,
) -> Element<'a, Message> {
    container(text(label, 11).font(font::MEDIUM))
        .padding([1, 6])
        .style(style::badge(tone))
        .into()
}
pub fn dot<'a, Message: 'a>(tone: Tone) -> Element<'a, Message> {
    container(Space::new())
        .width(8)
        .height(8)
        .style(style::dot(tone))
        .into()
}
/// A two-letter colored square, like T3 Code's project avatars.
pub fn tile<'a, Message: 'a>(
    label: impl IntoFragment<'a>,
    color: Color,
) -> Element<'a, Message> {
    container(text(label, 8).font(font::SEMIBOLD))
        .center_x(16)
        .center_y(16)
        .style(style::tile(color))
        .into()
}
pub fn alert<'a, Message: 'a>(
    glyph: char,
    message: impl IntoFragment<'a>,
    tone: Tone,
) -> Element<'a, Message> {
    container(
        row![icon(glyph), text(message, 13)]
            .spacing(10)
            .align_y(iced::Center),
    )
    .padding([12, 14])
    .width(Length::Fill)
    .style(style::alert(tone))
    .into()
}
/// An [`alert`] the user can close.
pub fn alert_dismissible<'a, Message: Clone + 'a>(
    id: impl Into<Id>,
    glyph: char,
    message: impl IntoFragment<'a>,
    tone: Tone,
    on_dismiss: Message,
) -> Element<'a, Message> {
    container(
        row![
            icon(glyph),
            text(message, 13).width(Length::Fill),
            icon_button(id, crate::icon::X, Some(on_dismiss)),
        ]
        .spacing(10)
        .align_y(iced::Center),
    )
    .padding(Padding::from([8, 14]).right(8))
    .width(Length::Fill)
    .style(style::alert(tone))
    .into()
}
/// Sizes used for sidebar media and empty-state illustrations.
#[derive(Debug, Clone, Copy)]
pub enum TileSize {
    Compact,
    Standard,
    Large,
}
impl TileSize {
    pub const fn pixels(self) -> u32 {
        match self {
            Self::Compact => 32,
            Self::Standard => 36,
            Self::Large => 48,
        }
    }
}
/// A glyph in a small bordered square.
pub fn icon_tile<'a, Message: 'a>(
    glyph: char,
    size: TileSize,
) -> Element<'a, Message> {
    container(icon(glyph))
        .center_x(size.pixels())
        .center_y(size.pixels())
        .style(style::icon_tile)
        .into()
}
/// A centered placeholder for a page with nothing to show. `media` is usually
/// an [`icon_tile`], or [`dots`] while content loads.
pub fn empty_state<'a, Message: 'a>(
    media: impl Into<Element<'a, Message>>,
    title: impl IntoFragment<'a>,
    lines: impl IntoIterator<Item = &'a str>,
) -> Element<'a, Message> {
    let mut content = column![
        media.into(),
        Space::new().height(8),
        text(title, 17).font(font::MEDIUM),
    ]
    .spacing(8)
    .align_x(iced::Center);
    for line in lines {
        content = content.push(muted(line).size(13));
    }
    container(content)
        .padding([72, 24])
        .center_x(Length::Fill)
        .into()
}
/// The card of a [`modal`]: a title, what is at stake and the actions, last one
/// rightmost.
pub fn dialog<'a, Message: 'a>(
    title: impl IntoFragment<'a>,
    description: impl IntoFragment<'a>,
    actions: impl IntoIterator<Item = Element<'a, Message>>,
) -> Element<'a, Message> {
    container(
        column![
            text(title, 16).font(font::SEMIBOLD),
            text(description, 13).color(color::MUTED),
            Space::new().height(6),
            container(Row::with_children(actions).spacing(8))
                .align_right(Length::Fill),
        ]
        .spacing(8),
    )
    .padding(20)
    .max_width(440)
    .style(style::dialog)
    .into()
}
/// Puts `dialog` in the middle of `base`, which stops taking input. Pressing the
/// scrim around it sends `on_dismiss`.
pub fn modal<'a, Message: Clone + 'a>(
    base: impl Into<Element<'a, Message>>,
    dialog: impl Into<Element<'a, Message>>,
    on_dismiss: Message,
) -> Element<'a, Message> {
    let scrim =
        container(Space::new().width(Length::Fill).height(Length::Fill))
            .style(style::scrim);
    stack![
        base.into(),
        mouse_area(opaque(scrim)).on_press(on_dismiss),
        container(opaque(dialog)).center(Length::Fill),
    ]
    .into()
}
/// A vertical scroller with T3 Code's thin overlay scrollbar.
pub fn scroll<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Scrollable<'a, Message> {
    scrollable(content)
        .direction(scrollable::Direction::Vertical(
            scrollable::Scrollbar::new()
                .width(6)
                .scroller_width(6)
                .margin(2),
        ))
        .style(style::scrollbar)
}
/// The titlebar path: muted parents, then the current page.
pub fn breadcrumb<'a, Message: 'a>(
    glyph: char,
    parents: impl IntoIterator<Item = &'a str>,
    current: impl IntoFragment<'a>,
) -> Element<'a, Message> {
    let mut path = row![icon(glyph).size(14).color(color::LABEL)]
        .spacing(8)
        .align_y(iced::Center);
    for parent in parents {
        path = path
            .push(text(parent, 13).color(color::MUTED))
            .push(text("/", 13).color(color::LABEL));
    }
    path.push(text(current, 13).font(font::MEDIUM)).into()
}
fn setting_row<'a, Message: 'a>(
    label: impl Into<Element<'a, Message>>,
    control: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    container(
        row![container(label).width(Length::Fill), control.into()]
            .spacing(24)
            .align_y(iced::Center),
    )
    .padding([12, 16])
    .width(Length::Fill)
    .into()
}
/// One row of a settings panel: what it is on the left, its control on the right.
pub fn setting<'a, Message: 'a>(
    title: impl IntoFragment<'a>,
    detail: Text<'a>,
    control: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    setting_row(
        column![text(title, 14).font(font::MEDIUM), detail].spacing(2),
        control,
    )
}
/// A [`setting`] whose title says all there is to say.
pub fn setting_titled<'a, Message: 'a>(
    title: impl IntoFragment<'a>,
    control: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    setting_row(text(title, 14).font(font::MEDIUM), control)
}
pub fn section_title<'a>(title: impl IntoFragment<'a>) -> Text<'a> {
    text(title, 16).font(font::SEMIBOLD)
}
/// [`section_title`] followed by a short description.
pub fn section_heading<'a, Message: 'a>(
    title: impl IntoFragment<'a>,
    detail: impl IntoFragment<'a>,
) -> Element<'a, Message> {
    row![section_title(title), muted(detail)]
        .spacing(8)
        .align_y(iced::Center)
        .into()
}
/// A heading above a raised panel. Rows share the panel, split by dividers.
pub fn section<'a, Message: 'a>(
    heading: impl Into<Element<'a, Message>>,
    action: Option<Element<'a, Message>>,
    rows: impl IntoIterator<Item = Element<'a, Message>>,
) -> Element<'a, Message> {
    let mut header = row![heading.into(), Space::new().width(Length::Fill)]
        .align_y(iced::Center)
        .height(28);
    if let Some(action) = action {
        header = header.push(action);
    }
    let mut panel = Column::new();
    for (index, row) in rows.into_iter().enumerate() {
        if index > 0 {
            panel = panel.push(divider());
        }
        panel = panel.push(row);
    }
    column![
        container(header).padding([0, 16]),
        container(panel).width(Length::Fill).style(style::panel)
    ]
    .spacing(10)
    .into()
}
/// A card docked below a page of up to `page_width`. It is wider than the page
/// by more than its corner radius, so content scrolling behind it ends on the
/// card's straight edge. Pad the page with [`PAGE_GUTTER`] for this to hold.
pub fn composer<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    page_width: f32,
    attention: bool,
) -> Element<'a, Message> {
    container(
        container(content)
            .padding(Padding::from([8, 8]).left(14))
            .max_width(page_width + 2.0 * (PAGE_GUTTER - COMPOSER_GUTTER))
            .style(style::composer(attention)),
    )
    .padding(Padding::from([0.0, COMPOSER_GUTTER]).bottom(COMPOSER_GUTTER))
    .center_x(Length::Fill)
    .into()
}
/// The sidebar column. `body` scrolls if the window gets short.
pub fn sidebar<'a, Message: 'a>(
    header: impl Into<Element<'a, Message>>,
    body: impl Into<Element<'a, Message>>,
    footer: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    container(column![
        header.into(),
        scroll(body).height(Length::Fill),
        footer.into()
    ])
    .padding([0, 6])
    .width(262)
    .height(Length::Fill)
    .style(style::sidebar)
    .into()
}
/// A group label in the sidebar, with an optional count on the right.
pub fn sidebar_label<'a, Message: 'a>(
    label: impl IntoFragment<'a>,
    count: Option<String>,
) -> Element<'a, Message> {
    let mut content = row![
        text(label, 12)
            .font(font::MEDIUM)
            .color(color::SUBDUED)
            .width(Length::Fill)
    ];
    if let Some(count) = count {
        content = content.push(muted(count));
    }
    container(content.align_y(iced::Center))
        .padding([0, 10])
        .center_y(32)
        .into()
}
/// A page link: an icon and a label, filled while selected.
pub fn sidebar_nav<'a, Message: Clone + 'a>(
    id: impl Into<Id>,
    glyph: char,
    label: impl IntoFragment<'a>,
    selected: bool,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    focusable(
        id,
        iced_button(
            row![
                icon(glyph),
                text(label, 14).font(if selected {
                    font::MEDIUM
                } else {
                    font::REGULAR
                })
            ]
            .spacing(10)
            .align_y(iced::Center),
        )
        .width(Length::Fill)
        .padding([8, 10])
        .style(style::sidebar_row(selected))
        .on_press_maybe(on_press.clone()),
        on_press,
    )
}
/// A labelled card of [`sidebar_choice`] rows.
pub fn sidebar_group<'a, Message: 'a>(
    label: impl IntoFragment<'a>,
    choices: impl IntoIterator<Item = Element<'a, Message>>,
) -> Element<'a, Message> {
    column![
        sidebar_label(label, None),
        container(
            Column::with_children(choices)
                .spacing(1)
                .width(Length::Fill)
        )
        .padding(3)
        .style(style::sidebar_card),
    ]
    .into()
}
/// Named content for a single-choice sidebar option.
pub struct SidebarChoice<'a, Message> {
    pub glyph: Option<char>,
    pub label: std::borrow::Cow<'a, str>,
    pub marker: Element<'a, Message>,
    pub selected: bool,
}
/// One option in a [`sidebar_group`]. `marker` is usually a [`radio`], or a
/// loader while the choice is being applied.
pub fn sidebar_choice<'a, Message: Clone + 'a>(
    id: impl Into<Id>,
    item: SidebarChoice<'a, Message>,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let SidebarChoice {
        glyph,
        label,
        marker,
        selected,
    } = item;
    let mut content = row![].spacing(9).align_y(iced::Center);
    if let Some(glyph) = glyph {
        content = content.push(icon(glyph));
    }
    focusable(
        id,
        iced_button(
            content
                .push(text(label, 13).width(Length::Fill))
                .push(marker),
        )
        .width(Length::Fill)
        .padding([7, 9])
        .style(style::sidebar_choice(selected))
        .on_press_maybe(on_press.clone()),
        on_press,
    )
}
/// A green check for a change that just succeeded. It fits the slot of a [`radio`]
/// or [`dots`], so swapping between them moves nothing.
pub fn check<'a, Message: 'a>() -> Element<'a, Message> {
    icon(crate::icon::CIRCLE_CHECK)
        .size(14)
        .line_height(LineHeight::Absolute(14.0.into()))
        .color(color::SUCCESS)
        .into()
}
/// The marker of a single-choice option.
pub fn radio<'a, Message: 'a>(selected: bool) -> Element<'a, Message> {
    let center: Element<'a, Message> = if selected {
        container(Space::new())
            .width(6)
            .height(6)
            .style(style::radio_dot)
            .into()
    } else {
        Space::new().into()
    };
    container(center)
        .center_x(14)
        .center_y(14)
        .style(style::radio_ring(selected))
        .into()
}
/// Named content for a device or account row.
pub struct SidebarDevice<'a, Message> {
    pub media: Element<'a, Message>,
    pub title: std::borrow::Cow<'a, str>,
    pub detail: std::borrow::Cow<'a, str>,
    pub selected: bool,
}
/// A device or account at the foot of the sidebar: `media` beside a title and a
/// detail line. Without `on_press` it is disabled.
pub fn sidebar_device<'a, Message: Clone + 'a>(
    id: impl Into<Id>,
    item: SidebarDevice<'a, Message>,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let SidebarDevice {
        media,
        title,
        detail,
        selected,
    } = item;
    let content = row![
        media,
        column![text(title, 13).font(font::MEDIUM), muted(detail).size(11)]
            .spacing(2)
            .width(Length::Fill),
    ]
    .spacing(10)
    .align_y(iced::Center);
    focusable(
        id,
        iced_button(content)
            .width(Length::Fill)
            .padding([6, 8])
            .style(style::sidebar_row(selected))
            .on_press_maybe(on_press.clone()),
        on_press,
    )
}
/// A sidebar row laid out like a T3 Code thread.
pub struct SidebarRow<'a> {
    pub id: Id,
    /// Two letters and a color for the [`tile`].
    pub tile: (&'a str, Color),
    pub meta: String,
    pub trailing: String,
    pub title: &'a str,
    pub detail: Option<String>,
    pub selected: bool,
}
pub fn sidebar_row<'a, Message: Clone + 'a>(
    item: SidebarRow<'a>,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let mut content = column![
        row![
            tile(item.tile.0, item.tile.1),
            text(item.meta, 12).color(color::LABEL).width(Length::Fill),
            text(item.trailing, 12).color(color::LABEL),
        ]
        .spacing(6)
        .align_y(iced::Center),
        text(item.title, 14).font(if item.selected {
            font::MEDIUM
        } else {
            font::REGULAR
        }),
    ]
    .spacing(4);
    if let Some(detail) = item.detail {
        content = content.push(text(detail, 12).color(color::LABEL));
    }
    focusable(
        item.id,
        iced_button(content)
            .width(Length::Fill)
            .padding([8, 10])
            .style(style::sidebar_row(item.selected))
            .on_press_maybe(on_press.clone()),
        on_press,
    )
}
/// Named content for a compact sidebar row.
pub struct SidebarItem<'a, Message> {
    pub media: Element<'a, Message>,
    pub title: std::borrow::Cow<'a, str>,
    pub trailing: std::borrow::Cow<'a, str>,
    pub selected: bool,
}
/// A single-line sidebar row, like T3 Code's settled threads.
pub fn sidebar_item<'a, Message: Clone + 'a>(
    id: impl Into<Id>,
    item: SidebarItem<'a, Message>,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let SidebarItem {
        media,
        title,
        trailing,
        selected,
    } = item;
    focusable(
        id,
        iced_button(
            row![
                media,
                text(title, 14).width(Length::Fill),
                text(trailing, 12).color(color::LABEL),
            ]
            .spacing(10)
            .align_y(iced::Center),
        )
        .width(Length::Fill)
        .padding([8, 10])
        .style(style::sidebar_row(selected))
        .on_press_maybe(on_press.clone()),
        on_press,
    )
}
