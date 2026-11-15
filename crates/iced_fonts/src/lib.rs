use iced_core::Font;

pub mod bootstrap;

pub use bootstrap::Bootstrap;

/// The default icon font bytes for loading the font into iced.
pub const BOOTSTRAP_FONT_BYTES: &[u8] = include_bytes!("../fonts/bootstrap-icons.ttf");

/// The bootstrap icon font.
pub const BOOTSTRAP_FONT: Font = Font::with_name("bootstrap-icons");
