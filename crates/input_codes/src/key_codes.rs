use derive_more::Display;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Display, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", derive(strum::EnumIter))]
#[non_exhaustive]
/// Key codes with names that are consistent across platforms.
///
/// Impls `FromStr` for easy parsing from strings. \
/// The `Unknown` variant is used for keys that are not in the list. \
/// Impls `Display` for easy conversion to strings. \
///
/// Impls From [`rdev::Key`] and [`device_query::Keycode`]. \
/// and TryInto [`rdev::Key`] and [`device_query::Keycode`] for easy conversion.
///
pub enum Keycode {
    // Numbers
    #[display("0")]
    Num0,
    #[display("1")]
    Num1,
    #[display("2")]
    Num2,
    #[display("3")]
    Num3,
    #[display("4")]
    Num4,
    #[display("5")]
    Num5,
    #[display("6")]
    Num6,
    #[display("7")]
    Num7,
    #[display("8")]
    Num8,
    #[display("9")]
    Num9,

    // KeyPad
    #[display("0")]
    Kp0,
    #[display("1")]
    Kp1,
    #[display("2")]
    Kp2,
    #[display("3")]
    Kp3,
    #[display("4")]
    Kp4,
    #[display("5")]
    Kp5,
    #[display("6")]
    Kp6,
    #[display("7")]
    Kp7,
    #[display("8")]
    Kp8,
    #[display("9")]
    Kp9,
    #[display("Return")]
    KpReturn,
    #[display("Enter")]
    KpEnter,
    #[display("Minus")]
    KpMinus,
    #[display("Plus")]
    KpPlus,
    #[display("Multiply")]
    KpMultiply,
    #[display("Divide")]
    KpDivide,
    #[display("Decimal")]
    KpDecimal,
    #[display("Equal")]
    KpEqual,
    #[display("Comma")]
    KpComma,

    // Letters
    #[display("a")]
    A,
    #[display("b")]
    B,
    #[display("c")]
    C,
    #[display("d")]
    D,
    #[display("e")]
    E,
    #[display("f")]
    F,
    #[display("g")]
    G,
    #[display("h")]
    H,
    #[display("i")]
    I,
    #[display("j")]
    J,
    #[display("k")]
    K,
    #[display("l")]
    L,
    #[display("m")]
    M,
    #[display("n")]
    N,
    #[display("o")]
    O,
    #[display("p")]
    P,
    #[display("q")]
    Q,
    #[display("r")]
    R,
    #[display("s")]
    S,
    #[display("t")]
    T,
    #[display("u")]
    U,
    #[display("v")]
    V,
    #[display("w")]
    W,
    #[display("x")]
    X,
    #[display("y")]
    Y,
    #[display("z")]
    Z,

    // F keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    // Arrow keys
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,

    // Symbols
    LeftBracket,
    RightBracket,
    /// also known as "backtick", "grave accent" and "tilda"
    BackQuote,
    SemiColon,
    Quote,
    BackSlash,
    IntlBackslash,
    Slash,
    Comma,
    Period,
    Minus,
    Equal,
    CapsLock,

    // Volume keys
    VolumeUp,
    VolumeDown,
    VolumeMute,

    // Modifier keys
    LeftShift,
    RightShift,
    LeftControl,
    RightControl,
    /// Alt key on Linux and Windows (option key on macOS)
    LeftAlt,
    // also AltGr on some systems
    RightAlt,
    /// also known as "windows", "super", and "command"
    LeftMeta,
    /// also known as "windows", "super", and "command"
    RightMeta,

    // Other keys
    Escape,
    Delete,
    Backspace,
    Space,
    Enter,
    Tab,
    PageUp,
    PageDown,
    Home,
    End,
    Insert,
    PrintScreen,
    ScrollLock,
    Pause,
    NumLock,
    Function,

    // Other
    #[display("{_0}")]
    Unknown(UnknownKeyCode),
}

impl Keycode {
    pub fn iter() -> impl Iterator<Item = Keycode> {
        use strum::IntoEnumIterator;
        <Self as IntoEnumIterator>::iter().filter(|key| !matches!(key, Keycode::Unknown(_)))
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Display, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[display("Unknown({_variant})")]
pub enum UnknownKeyCode {
    #[display("{_0}")]
    Str(String),
    #[display("{_0}")]
    Int(u32),
}

impl<T> From<T> for UnknownKeyCode
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self::Str(value.into())
    }
}

impl Default for UnknownKeyCode {
    fn default() -> Self {
        Self::Int(0)
    }
}

impl FromStr for Keycode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::Num0),
            "1" => Ok(Self::Num1),
            "2" => Ok(Self::Num2),
            "3" => Ok(Self::Num3),
            "4" => Ok(Self::Num4),
            "5" => Ok(Self::Num5),
            "6" => Ok(Self::Num6),
            "7" => Ok(Self::Num7),
            "8" => Ok(Self::Num8),
            "9" => Ok(Self::Num9),
            // Letters
            "A" | "a" => Ok(Self::A),
            "B" | "b" => Ok(Self::B),
            "C" | "c" => Ok(Self::C),
            "D" | "d" => Ok(Self::D),
            "E" | "e" => Ok(Self::E),
            "F" | "f" => Ok(Self::F),
            "G" | "g" => Ok(Self::G),
            "H" | "h" => Ok(Self::H),
            "I" | "i" => Ok(Self::I),
            "J" | "j" => Ok(Self::J),
            "K" | "k" => Ok(Self::K),
            "L" | "l" => Ok(Self::L),
            "M" | "m" => Ok(Self::M),
            "N" | "n" => Ok(Self::N),
            "O" | "o" => Ok(Self::O),
            "P" | "p" => Ok(Self::P),
            "Q" | "q" => Ok(Self::Q),
            "R" | "r" => Ok(Self::R),
            "S" | "s" => Ok(Self::S),
            "T" | "t" => Ok(Self::T),
            "U" | "u" => Ok(Self::U),
            "V" | "v" => Ok(Self::V),
            "W" | "w" => Ok(Self::W),
            "X" | "x" => Ok(Self::X),
            "Y" | "y" => Ok(Self::Y),
            "Z" | "z" => Ok(Self::Z),
            // F keys
            "F1" => Ok(Self::F1),
            "F2" => Ok(Self::F2),
            "F3" => Ok(Self::F3),
            "F4" => Ok(Self::F4),
            "F5" => Ok(Self::F5),
            "F6" => Ok(Self::F6),
            "F7" => Ok(Self::F7),
            "F8" => Ok(Self::F8),
            "F9" => Ok(Self::F9),
            "F10" => Ok(Self::F10),
            "F11" => Ok(Self::F11),
            "F12" => Ok(Self::F12),
            "F13" => Ok(Self::F13),
            "F14" => Ok(Self::F14),
            "F15" => Ok(Self::F15),
            "F16" => Ok(Self::F16),
            "F17" => Ok(Self::F17),
            "F18" => Ok(Self::F18),
            "F19" => Ok(Self::F19),
            "F20" => Ok(Self::F20),
            "F21" => Ok(Self::F21),
            "F22" => Ok(Self::F22),
            "F23" => Ok(Self::F23),
            "F24" => Ok(Self::F24),
            // Arrow keys
            "UpArrow" => Ok(Self::UpArrow),
            "DownArrow" => Ok(Self::DownArrow),
            "LeftArrow" => Ok(Self::LeftArrow),
            "RightArrow" => Ok(Self::RightArrow),
            // Symbols
            "LeftBracket" => Ok(Self::LeftBracket),
            "RightBracket" => Ok(Self::RightBracket),
            "BackQuote" => Ok(Self::BackQuote),
            "SemiColon" => Ok(Self::SemiColon),
            "Quote" => Ok(Self::Quote),
            "BackSlash" => Ok(Self::BackSlash),
            "IntlBackslash" => Ok(Self::IntlBackslash),
            "Slash" => Ok(Self::Slash),
            "Comma" => Ok(Self::Comma),
            "Period" => Ok(Self::Period),
            "Minus" => Ok(Self::Minus),
            "Equal" => Ok(Self::Equal),
            "CapsLock" => Ok(Self::CapsLock),
            // Volume keys
            "VolumeUp" => Ok(Self::VolumeUp),
            "VolumeDown" => Ok(Self::VolumeDown),
            "VolumeMute" => Ok(Self::VolumeMute),
            // Modifier keys
            "LeftShift" => Ok(Self::LeftShift),
            "RightShift" => Ok(Self::RightShift),
            "LeftControl" => Ok(Self::LeftControl),
            "RightControl" => Ok(Self::RightControl),
            "LeftAlt" => Ok(Self::LeftAlt),
            "RightAlt" => Ok(Self::RightAlt),
            "LeftMeta" => Ok(Self::LeftMeta),
            "RightMeta" => Ok(Self::RightMeta),
            // Other keys
            "Escape" => Ok(Self::Escape),
            "Delete" => Ok(Self::Delete),
            "Backspace" => Ok(Self::Backspace),
            "Space" => Ok(Self::Space),
            "Enter" => Ok(Self::Enter),
            "Tab" => Ok(Self::Tab),
            "PageUp" => Ok(Self::PageUp),
            "PageDown" => Ok(Self::PageDown),
            "Home" => Ok(Self::Home),
            "End" => Ok(Self::End),
            "Insert" => Ok(Self::Insert),
            "PrintScreen" => Ok(Self::PrintScreen),
            "ScrollLock" => Ok(Self::ScrollLock),
            "Pause" => Ok(Self::Pause),
            "NumLock" => Ok(Self::NumLock),
            "Function" => Ok(Self::Function),
            key => {
                if key.starts_with("Unknown(") {
                    let key = key.trim_start_matches("Unknown(").trim_end_matches(')');
                    if let Ok(int) = key.parse() {
                        return Ok(Self::Unknown(UnknownKeyCode::Int(int)));
                    }
                    Ok(Self::Unknown(UnknownKeyCode::Str(key.to_string())))
                } else {
                    Err(format!("Unknown key code: {key}"))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keycodes() {
        // test numbers
        assert_eq!(Keycode::Num0.to_string(), "0");
        assert_eq!(Keycode::Num1.to_string(), "1");
        assert_eq!(Keycode::Num2.to_string(), "2");
        assert_eq!(Keycode::Num3.to_string(), "3");
        assert_eq!(Keycode::Num4.to_string(), "4");
        assert_eq!(Keycode::Num5.to_string(), "5");
        assert_eq!(Keycode::Num6.to_string(), "6");
        assert_eq!(Keycode::Num7.to_string(), "7");
        assert_eq!(Keycode::Num8.to_string(), "8");
        assert_eq!(Keycode::Num9.to_string(), "9");

        // test letters
        assert_eq!(Keycode::A.to_string(), "a");
        assert_eq!(Keycode::B.to_string(), "b");
        assert_eq!(Keycode::C.to_string(), "c");
        assert_eq!(Keycode::D.to_string(), "d");
        assert_eq!(Keycode::E.to_string(), "e");
        assert_eq!(Keycode::F.to_string(), "f");
        assert_eq!(Keycode::G.to_string(), "g");
        assert_eq!(Keycode::H.to_string(), "h");
        assert_eq!(Keycode::I.to_string(), "i");
        assert_eq!(Keycode::J.to_string(), "j");
        assert_eq!(Keycode::K.to_string(), "k");
        assert_eq!(Keycode::L.to_string(), "l");
        assert_eq!(Keycode::M.to_string(), "m");
        assert_eq!(Keycode::N.to_string(), "n");
        assert_eq!(Keycode::O.to_string(), "o");
        assert_eq!(Keycode::P.to_string(), "p");
        assert_eq!(Keycode::Q.to_string(), "q");
        assert_eq!(Keycode::R.to_string(), "r");
        assert_eq!(Keycode::S.to_string(), "s");
        assert_eq!(Keycode::T.to_string(), "t");
        assert_eq!(Keycode::U.to_string(), "u");
        assert_eq!(Keycode::V.to_string(), "v");
        assert_eq!(Keycode::W.to_string(), "w");
        assert_eq!(Keycode::X.to_string(), "x");
        assert_eq!(Keycode::Y.to_string(), "y");
        assert_eq!(Keycode::Z.to_string(), "z");

        // test F keys
        assert_eq!(Keycode::F1.to_string(), "F1");
        assert_eq!(Keycode::F2.to_string(), "F2");
        assert_eq!(Keycode::F3.to_string(), "F3");
        assert_eq!(Keycode::F4.to_string(), "F4");
        assert_eq!(Keycode::F5.to_string(), "F5");
        assert_eq!(Keycode::F6.to_string(), "F6");
        assert_eq!(Keycode::F7.to_string(), "F7");
        assert_eq!(Keycode::F8.to_string(), "F8");
        assert_eq!(Keycode::F9.to_string(), "F9");
        assert_eq!(Keycode::F10.to_string(), "F10");
        assert_eq!(Keycode::F11.to_string(), "F11");
        assert_eq!(Keycode::F12.to_string(), "F12");
        assert_eq!(Keycode::F13.to_string(), "F13");
        assert_eq!(Keycode::F14.to_string(), "F14");
        assert_eq!(Keycode::F15.to_string(), "F15");
        assert_eq!(Keycode::F16.to_string(), "F16");
        assert_eq!(Keycode::F17.to_string(), "F17");
        assert_eq!(Keycode::F18.to_string(), "F18");
        assert_eq!(Keycode::F19.to_string(), "F19");
        assert_eq!(Keycode::F20.to_string(), "F20");
        assert_eq!(Keycode::F21.to_string(), "F21");
        assert_eq!(Keycode::F22.to_string(), "F22");
        assert_eq!(Keycode::F23.to_string(), "F23");
        assert_eq!(Keycode::F24.to_string(), "F24");
    }
}
