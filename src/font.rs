//! Inter for text and Lucide for icons. [`crate::settings`] loads them all.
use iced::{Font, font};

pub const REGULAR: Font = Font::with_name("Inter");
pub const MEDIUM: Font = Font {
    weight: font::Weight::Medium,
    ..REGULAR
};
pub const SEMIBOLD: Font = Font {
    weight: font::Weight::Semibold,
    ..REGULAR
};
pub const ICONS: Font = Font::with_name("lucide");
pub const ALL: [&[u8]; 4] = [
    include_bytes!("../assets/fonts/inter/Inter-Regular.ttf"),
    include_bytes!("../assets/fonts/inter/Inter-Medium.ttf"),
    include_bytes!("../assets/fonts/inter/Inter-SemiBold.ttf"),
    include_bytes!("../assets/fonts/lucide/Lucide.ttf"),
];
