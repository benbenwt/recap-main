use std::str::FromStr;

use derive_more::derive::Display;

/// Standard mouse buttons
/// Some mice have more than 3 buttons. These are not defined, and different
/// OSs will give different `Button::Unknown` values.
#[derive(Debug, Copy, Eq, PartialEq, Hash, Clone, Display, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Button {
    #[display("Left")]
    Left,
    #[display("Right")]
    Right,
    #[display("Middle")]
    Middle,
    #[display("Unknown({_0})")]
    Unknown(u8),
}

impl From<u8> for Button {
    fn from(value: u8) -> Self {
        match value {
            0 => Button::Left,
            1 => Button::Right,
            2 => Button::Middle,
            _ => Button::Unknown(value),
        }
    }
}

impl FromStr for Button {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Left" => Ok(Button::Left),
            "Right" => Ok(Button::Right),
            "Middle" => Ok(Button::Middle),
            _ => {
                if let Ok(value) = s.parse::<u8>() {
                    Ok(Button::from(value))
                } else {
                    Err(format!("Invalid button string: {s}"))
                }
            }
        }
    }
}
