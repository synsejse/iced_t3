# Migrating to api-and-components

The two consuming applications remain pinned to their existing commits. These
are all source-breaking changes in this branch:

| Previous call or field | Replacement |
|---|---|
| `icon_tile(glyph, 32)` | `icon_tile(glyph, TileSize::Compact)` |
| `icon_tile(glyph, 36)` | `icon_tile(glyph, TileSize::Standard)` |
| `icon_tile(glyph, 48)` | `icon_tile(glyph, TileSize::Large)` |
| `sidebar_device(id, media, title, detail, selected, on_press)` | `sidebar_device(id, SidebarDevice { media: media.into(), title: title.into(), detail: detail.into(), selected }, on_press)` |
| `sidebar_choice(id, glyph, label, marker, selected, on_press)` | `sidebar_choice(id, SidebarChoice { glyph, label: label.into(), marker: marker.into(), selected }, on_press)` |
| `sidebar_item(id, media, title, trailing, selected, on_press)` | `sidebar_item(id, SidebarItem { media: media.into(), title: title.into(), trailing: trailing.into(), selected }, on_press)` |
| `SidebarRow { id: "row", ... }` | `sidebar_choice(id, glyph, label, marker, selected, on_press)` | `sidebar_choice(id, SidebarChoice { glyph, label: label.into(), marker: marker.into(), selected }, on_press)` |
| `sidebar_item(id, media, title, trailing, selected, on_press)` | `sidebar_item(id, SidebarItem { media: media.into(), title: title.into(), trailing: trailing.into(), selected }, on_press)` |
| `SidebarRow { id: "row".into(), ... }`; the field now accepts an owned `iced::advanced::widget::Id` |

`SidebarDevice` titles and details use `Cow<str>`: call `.into()` on borrowed
strings or owned formatted strings. No temporary references are necessary.

For a custom tile size, use `container(t3::icon(glyph))`, set even dimensions
with `center_x`/`center_y`, and apply `style::icon_tile`.

Behavior changes: `sidebar_device(..., None)` now uses the disabled button
appearance, preserving selected state and skipping keyboard focus. For a passive
label without disabled styling, use a plain row. Standard and small buttons now
use even horizontal padding (12px and 10px respectively).

Optional setup simplifications:

```rust,ignore
.settings(iced_t3::settings_for("your.desktop.entry"))
.window(iced::window::Settings {
    size: iced::Size::new(1120.0, 780.0),
    ..iced_t3::chrome::window_settings_for("your.desktop.entry")
})
```

These helpers return normal Iced settings. Font loading is already included.
Keep app-specific fonts with `.font(...)`. Both helpers take the desktop entry
basename; the window helper sets Wayland's application ID on Linux.

Replace repeated input styles and padding with
`t3::input(placeholder, value, Some(Message::Changed))` or `t3::search(...)`.
They return native `TextInput` builders, so `.id(...)`, `.width(...)`,
`.secure(...)`, and `.on_submit(...)` remain available. Disabled inputs take
`None::<fn(String) -> Message>`. They retain native editing and keyboard focus;
all press-based controls use `focusable`.

New checkbox callbacks follow the switch convention:
`Some(Message::Toggled)`, or `None::<fn(bool) -> Message>` while disabled.
