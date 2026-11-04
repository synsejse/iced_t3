//! Style functions for Iced's built-in widgets.
use crate::color::{
    BACKGROUND, BORDER, DANGER, FOREGROUND, HOVER, INPUT, MUTED, PRIMARY,
    RAISED, SELECTED, SIDEBAR, SIDEBAR_BORDER, SUBDUED, SUCCESS, TRACK,
    WARNING,
};
use iced::{
    Border, Color, Theme,
    widget::{
        button, container, scrollable, slider as iced_slider, text_input,
        toggler,
    },
};
/// `disabled:opacity-64`
const DISABLED_OPACITY: f32 = 0.64;
const CONTROL_RADIUS: f32 = 8.0;
const PANEL_RADIUS: f32 = 16.0;

fn white(alpha: f32) -> Color {
    Color::WHITE.scale_alpha(alpha)
}

/// The meaning of a badge, status dot or alert.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Neutral,
    Success,
    Warning,
    Danger,
}
impl Tone {
    pub fn color(self) -> Color {
        match self {
            Self::Neutral => MUTED,
            Self::Success => SUCCESS,
            Self::Warning => WARNING,
            Self::Danger => DANGER,
        }
    }
}

fn surface(color: Color, border: Color, radius: f32) -> container::Style {
    container::Style {
        background: Some(color.into()),
        border: Border {
            color: border,
            width: 1.0,
            radius: radius.into(),
        },
        ..Default::default()
    }
}
fn filled(color: Color, radius: f32) -> container::Style {
    surface(color, Color::TRANSPARENT, radius)
}
/// A raised panel, such as a settings group.
pub fn panel(_: &Theme) -> container::Style {
    surface(RAISED, white(0.04), PANEL_RADIUS)
}
/// The docked composer. `attention` tints its edge, for example for unsaved work.
pub fn composer(attention: bool) -> impl Fn(&Theme) -> container::Style {
    move |_| {
        surface(
            RAISED,
            if attention {
                WARNING.scale_alpha(0.32)
            } else {
                white(0.04)
            },
            PANEL_RADIUS,
        )
    }
}
pub fn sidebar(_: &Theme) -> container::Style {
    container::Style {
        background: Some(SIDEBAR.into()),
        ..Default::default()
    }
}
pub fn popover(_: &Theme) -> container::Style {
    surface(RAISED, BORDER, 6.0)
}
pub fn icon_tile(_: &Theme) -> container::Style {
    container::Style {
        text_color: Some(FOREGROUND),
        ..surface(RAISED, BORDER, 6.0)
    }
}
/// A horizontal or vertical rule inside a surface.
pub fn divider(color: Color) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        background: Some(color.into()),
        ..Default::default()
    }
}
/// The rule between the sidebar and the page.
pub fn sidebar_divider(theme: &Theme) -> container::Style {
    divider(SIDEBAR_BORDER)(theme)
}
pub fn segmented(_: &Theme) -> container::Style {
    filled(white(0.044), CONTROL_RADIUS)
}
pub fn badge(tone: Tone) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        text_color: Some(tone.color()),
        ..filled(
            match tone {
                Tone::Neutral => HOVER,
                _ => tone.color().scale_alpha(0.16),
            },
            4.0,
        )
    }
}
pub fn dot(tone: Tone) -> impl Fn(&Theme) -> container::Style {
    move |_| filled(tone.color(), 4.0)
}
/// The small colored square that identifies a sidebar row.
pub fn tile(color: Color) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        text_color: Some(color),
        ..filled(color.scale_alpha(0.18), 4.0)
    }
}
pub fn alert(tone: Tone) -> impl Fn(&Theme) -> container::Style {
    move |_| {
        let color = tone.color();
        container::Style {
            text_color: Some(color),
            ..surface(
                color.scale_alpha(0.16),
                color.scale_alpha(0.32),
                PANEL_RADIUS,
            )
        }
    }
}

fn hovered(status: button::Status) -> bool {
    matches!(status, button::Status::Hovered | button::Status::Pressed)
}
fn control(
    background: Option<Color>,
    text_color: Color,
    border: Color,
    radius: f32,
    status: button::Status,
) -> button::Style {
    let opacity = if status == button::Status::Disabled {
        DISABLED_OPACITY
    } else {
        1.0
    };
    button::Style {
        background: background.map(|c| c.scale_alpha(opacity).into()),
        text_color: text_color.scale_alpha(opacity),
        border: Border {
            color: border.scale_alpha(opacity),
            width: 1.0,
            radius: radius.into(),
        },
        ..Default::default()
    }
}
pub fn primary(_: &Theme, status: button::Status) -> button::Style {
    let fill = if hovered(status) {
        PRIMARY.scale_alpha(0.9)
    } else {
        PRIMARY
    };
    control(Some(fill), Color::WHITE, fill, CONTROL_RADIUS, status)
}
pub fn outline(_: &Theme, status: button::Status) -> button::Style {
    let fill = if hovered(status) {
        white(0.07)
    } else {
        white(0.035)
    };
    control(Some(fill), FOREGROUND, INPUT, CONTROL_RADIUS, status)
}
pub fn ghost(_: &Theme, status: button::Status) -> button::Style {
    let fill = hovered(status).then_some(HOVER);
    control(fill, FOREGROUND, Color::TRANSPARENT, CONTROL_RADIUS, status)
}
/// Titlebar and toolbar icons: muted until hovered.
pub fn ghost_muted(_: &Theme, status: button::Status) -> button::Style {
    let fill = hovered(status).then_some(HOVER);
    let text = if hovered(status) { FOREGROUND } else { MUTED };
    control(fill, text, Color::TRANSPARENT, 6.0, status)
}
/// An icon button holding a state: red while active.
pub fn icon_toggle(
    active: bool,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |theme, status| {
        if !active {
            return ghost_muted(theme, status);
        }
        let tint = if hovered(status) { 0.24 } else { 0.16 };
        control(
            Some(DANGER.scale_alpha(tint)),
            DANGER,
            Color::TRANSPARENT,
            6.0,
            status,
        )
    }
}
pub fn segment(
    selected: bool,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_, status| {
        let fill = if selected {
            Some(white(0.08))
        } else {
            hovered(status).then_some(white(0.035))
        };
        let text = if selected || hovered(status) {
            FOREGROUND
        } else {
            MUTED
        };
        control(fill, text, Color::TRANSPARENT, 6.0, status)
    }
}
pub fn sidebar_row(
    selected: bool,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_, status| {
        let fill = if selected {
            Some(SELECTED)
        } else {
            hovered(status).then_some(HOVER)
        };
        let text = if selected || hovered(status) {
            FOREGROUND
        } else {
            SUBDUED
        };
        control(fill, text, Color::TRANSPARENT, CONTROL_RADIUS, status)
    }
}
/// A row in a [`sidebar_group`](crate::widget::sidebar_group): choices show
/// their state with a marker, so only hover fills them.
pub fn sidebar_choice(
    selected: bool,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_, status| {
        let text = if selected || hovered(status) {
            FOREGROUND
        } else {
            SUBDUED
        };
        control(
            hovered(status).then_some(HOVER),
            text,
            Color::TRANSPARENT,
            CONTROL_RADIUS,
            status,
        )
    }
}
/// The ring of a [`radio`](crate::widget::radio) marker.
pub fn radio_ring(selected: bool) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        border: Border {
            color: if selected { PRIMARY } else { INPUT },
            width: 1.5,
            radius: 7.0.into(),
        },
        ..Default::default()
    }
}
/// The center of a selected [`radio`](crate::widget::radio) marker.
pub fn radio_dot(_: &Theme) -> container::Style {
    filled(PRIMARY, 3.0)
}
/// A card of grouped choices in the sidebar.
pub fn sidebar_card(_: &Theme) -> container::Style {
    surface(RAISED, white(0.04), 12.0)
}
pub fn switch(_: &Theme, status: toggler::Status) -> toggler::Style {
    let (on, opacity) = match status {
        toggler::Status::Active { is_toggled }
        | toggler::Status::Hovered { is_toggled } => (is_toggled, 1.0),
        toggler::Status::Disabled { is_toggled } => {
            (is_toggled, DISABLED_OPACITY)
        }
    };
    toggler::Style {
        background: if on { PRIMARY } else { INPUT }
            .scale_alpha(opacity)
            .into(),
        background_border_width: 0.0,
        background_border_color: Color::TRANSPARENT,
        foreground: if on { BACKGROUND } else { MUTED }
            .scale_alpha(opacity)
            .into(),
        foreground_border_width: 0.0,
        foreground_border_color: Color::TRANSPARENT,
        text_color: Some(FOREGROUND.scale_alpha(opacity)),
        border_radius: None,
        padding_ratio: 0.11,
    }
}
pub fn input(_: &Theme, status: text_input::Status) -> text_input::Style {
    let disabled = status == text_input::Status::Disabled;
    text_input::Style {
        background: white(0.035).into(),
        border: Border {
            color: if matches!(status, text_input::Status::Focused { .. }) {
                PRIMARY
            } else {
                INPUT
            },
            width: 1.0,
            radius: CONTROL_RADIUS.into(),
        },
        icon: MUTED,
        placeholder: MUTED,
        value: if disabled { MUTED } else { FOREGROUND },
        selection: PRIMARY.scale_alpha(0.4),
    }
}
/// A text input for dense grids of values: bordered only on hover and focus.
pub fn input_quiet(
    theme: &Theme,
    status: text_input::Status,
) -> text_input::Style {
    let border = match status {
        text_input::Status::Focused { .. } => PRIMARY,
        text_input::Status::Hovered => INPUT,
        _ => Color::TRANSPARENT,
    };
    let style = input(theme, status);
    text_input::Style {
        background: Color::TRANSPARENT.into(),
        border: Border {
            color: border,
            ..style.border
        },
        ..style
    }
}
/// A horizontal or vertical slider: the rail fills up to a ring-shaped handle.
/// A disabled slider turns gray, because Iced's sliders have no disabled state.
pub fn slider(
    enabled: bool,
) -> impl Fn(&Theme, iced_slider::Status) -> iced_slider::Style {
    move |_, status| iced_slider::Style {
        rail: iced_slider::Rail {
            backgrounds: (
                if enabled {
                    PRIMARY
                } else {
                    MUTED.scale_alpha(0.35)
                }
                .into(),
                TRACK.into(),
            ),
            width: 6.0,
            border: Border {
                radius: 3.0.into(),
                ..Default::default()
            },
        },
        handle: iced_slider::Handle {
            shape: iced_slider::HandleShape::Circle { radius: 7.0 },
            background: if status == iced_slider::Status::Dragged {
                PRIMARY
            } else {
                BACKGROUND
            }
            .into(),
            border_width: 2.0,
            border_color: if enabled {
                PRIMARY
            } else {
                MUTED.scale_alpha(0.5)
            },
        },
    }
}
pub fn scrollbar(
    theme: &Theme,
    status: scrollable::Status,
) -> scrollable::Style {
    let mut style = scrollable::default(theme, status);
    let hovered = matches!(
        status,
        scrollable::Status::Hovered {
            is_vertical_scrollbar_hovered: true,
            ..
        } | scrollable::Status::Dragged {
            is_vertical_scrollbar_dragged: true,
            ..
        }
    );
    style.vertical_rail = scrollable::Rail {
        background: None,
        border: Border::default(),
        scroller: scrollable::Scroller {
            background: white(if hovered { 0.12 } else { 0.08 }).into(),
            border: Border {
                radius: 3.0.into(),
                ..Default::default()
            },
        },
    };
    style
}
