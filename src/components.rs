//! Form controls and small surfaces built from the shared design tokens.
use crate::{Tone, color, focusable, font, icon, style, widget as t3};
use iced::advanced::text::IntoFragment;
use iced::advanced::widget::Id;
use iced::widget::{
    Column, Row, Space, button, column, container, row, text_input,
};
use iced::{Element, Length};

/// A labelled checkbox. The callback is absent while disabled.
pub fn checkbox<'a, Message: Clone + 'a>(
    id: impl Into<Id>,
    label: impl Into<String>,
    checked: bool,
    on_toggle: Option<impl Fn(bool) -> Message + 'a>,
) -> Element<'a, Message> {
    let press = on_toggle.map(|change| change(!checked));
    let disabled = press.is_none();
    let mark = container(if checked {
        t3::icon(icon::CHECK)
            .size(12)
            .line_height(t3::line_height(12))
    } else {
        t3::text("", 12)
    })
    .center_x(18)
    .center_y(18)
    .style(move |theme| {
        let mut result = style::radio_ring(checked)(theme);
        result.border.radius = 4.0.into();
        result.border.width = 1.0;
        if disabled {
            result.border.color = result.border.color.scale_alpha(0.4);
        }
        result
    });
    focusable(
        id,
        button(
            row![mark, t3::text(label.into(), 13)]
                .spacing(8)
                .align_y(iced::Center),
        )
        .padding([6, 8])
        .style(style::ghost)
        .on_press_maybe(press.clone()),
        press,
    )
}

/// A styled native editor. Native text editing retains its own Tab and key
/// handling; wrapping it as a press target would steal spaces from the value.
pub fn input<'a, Message: Clone + 'a>(
    placeholder: &str,
    value: &str,
    on_input: Option<impl Fn(String) -> Message + 'a>,
) -> iced::widget::TextInput<'a, Message> {
    let input = text_input(placeholder, value)
        .font(font::REGULAR)
        .size(14)
        .line_height(t3::line_height(14))
        .padding([6, 10])
        .style(style::input);
    match on_input {
        Some(change) => input.on_input(change),
        None => input,
    }
}

/// A search editor with a Lucide leading icon.
pub fn search<'a, Message: Clone + 'a>(
    placeholder: &str,
    value: &str,
    on_input: Option<impl Fn(String) -> Message + 'a>,
) -> iced::widget::TextInput<'a, Message> {
    input(placeholder, value, on_input).icon(text_input::Icon {
        font: font::ICONS,
        code_point: icon::SEARCH,
        size: Some(16.into()),
        spacing: 8.0,
        side: text_input::Side::Left,
    })
}

/// One page in a horizontal tab strip.
pub fn tab<'a, Message: Clone + 'a>(
    id: impl Into<Id>,
    label: impl Into<String>,
    selected: bool,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    focusable(
        id,
        button(t3::text(label.into(), 13).font(font::MEDIUM))
            .padding([8, 12])
            .style(style::tab(selected))
            .on_press_maybe(on_press.clone()),
        on_press,
    )
}
pub fn tabs<'a, Message: 'a>(
    items: impl IntoIterator<Item = Element<'a, Message>>,
) -> Element<'a, Message> {
    column![Row::with_children(items).spacing(4), t3::divider()].into()
}

/// A raised surface. Content and width remain caller controlled.
pub fn card<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    container(content).padding(16).style(style::panel)
}

/// A compact metric with a caption and supporting detail.
pub fn stat<'a, Message: 'a>(
    label: impl IntoFragment<'a>,
    value: impl IntoFragment<'a>,
    detail: impl IntoFragment<'a>,
) -> Element<'a, Message> {
    card(
        column![
            t3::muted(label),
            t3::text(value, 24).font(font::SEMIBOLD),
            t3::muted(detail)
        ]
        .spacing(4),
    )
    .width(Length::Fill)
    .into()
}

/// Determinate progress in 0..=1. Non-finite values display as zero.
pub fn progress<'a, Message: 'a>(
    value: f32,
    tone: Tone,
) -> Element<'a, Message> {
    let value = if value.is_finite() {
        value.clamp(0.0, 1.0)
    } else {
        0.0
    };
    iced::widget::progress_bar(0.0..=1.0, value)
        .girth(6)
        .style(move |_| iced::widget::progress_bar::Style {
            background: color::INPUT.into(),
            bar: tone.color().into(),
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
        })
        .into()
}

/// A passive keyboard hint; the application owns the actual shortcut.
pub fn shortcut<'a, Message: 'a>(
    label: impl IntoFragment<'a>,
) -> Element<'a, Message> {
    container(t3::text(label, 11).color(color::MUTED))
        .padding([2, 6])
        .style(style::icon_tile)
        .into()
}

/// Initials in a round 32px avatar.
pub fn avatar<'a, Message: 'a>(
    initials: impl IntoFragment<'a>,
    tint: iced::Color,
) -> Element<'a, Message> {
    container(t3::text(initials, 12).font(font::SEMIBOLD))
        .center_x(32)
        .center_y(32)
        .style(move |theme| {
            let mut style = style::tile(tint)(theme);
            style.border.radius = 16.0.into();
            style
        })
        .into()
}

/// A selectable tag. Use a badge for a passive label.
pub fn chip<'a, Message: Clone + 'a>(
    id: impl Into<Id>,
    label: impl Into<String>,
    selected: bool,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    t3::segment(id, label, selected, on_press)
}

/// A clickable row whose cells can be sized into columns by the caller.
pub fn list_row<'a, Message: Clone + 'a>(
    id: impl Into<Id>,
    cells: impl IntoIterator<Item = Element<'a, Message>>,
    selected: bool,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    focusable(
        id,
        button(Row::with_children(cells).spacing(16).align_y(iced::Center))
            .width(Length::Fill)
            .padding([10, 12])
            .style(style::sidebar_row(selected))
            .on_press_maybe(on_press.clone()),
        on_press,
    )
}

/// An expandable section; the application owns its open state.
pub fn accordion<'a, Message: Clone + 'a>(
    id: impl Into<Id>,
    title: impl Into<String>,
    expanded: bool,
    content: impl Into<Element<'a, Message>>,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let header = focusable(
        id,
        button(
            row![
                t3::icon(if expanded {
                    icon::CHEVRON_DOWN
                } else {
                    icon::CHEVRON_RIGHT
                }),
                t3::text(title.into(), 14).font(font::MEDIUM),
            ]
            .spacing(8)
            .align_y(iced::Center),
        )
        .width(Length::Fill)
        .padding(12)
        .style(style::ghost)
        .on_press_maybe(on_press.clone()),
        on_press,
    );
    let mut body = Column::new().push(header);
    if expanded {
        body = body.push(container(content).padding([12, 16]));
    }
    container(body).style(style::panel).into()
}

/// A static placeholder, deliberately without shimmer or a frame subscription.
/// Dimensions are rounded up to even pixels.
pub fn skeleton<'a, Message: 'a>(
    width: u32,
    height: u32,
) -> Element<'a, Message> {
    let even = |v: u32| v.saturating_add(v % 2);
    container(Space::new())
        .width(even(width))
        .height(even(height))
        .style(style::skeleton)
        .into()
}
