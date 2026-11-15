use crate::{Button, Keycode, UnknownKeyCode};

impl From<rdev::Key> for Keycode {
    fn from(value: rdev::Key) -> Self {
        match value {
            // numbers
            rdev::Key::Num1 => Keycode::Num1,
            rdev::Key::Num2 => Keycode::Num2,
            rdev::Key::Num3 => Keycode::Num3,
            rdev::Key::Num4 => Keycode::Num4,
            rdev::Key::Num5 => Keycode::Num5,
            rdev::Key::Num6 => Keycode::Num6,
            rdev::Key::Num7 => Keycode::Num7,
            rdev::Key::Num8 => Keycode::Num8,
            rdev::Key::Num9 => Keycode::Num9,
            rdev::Key::Num0 => Keycode::Num0,

            // Keypad
            rdev::Key::Kp0 => Keycode::Kp0,
            rdev::Key::Kp1 => Keycode::Kp1,
            rdev::Key::Kp2 => Keycode::Kp2,
            rdev::Key::Kp3 => Keycode::Kp3,
            rdev::Key::Kp4 => Keycode::Kp4,
            rdev::Key::Kp5 => Keycode::Kp5,
            rdev::Key::Kp6 => Keycode::Kp6,
            rdev::Key::Kp7 => Keycode::Kp7,
            rdev::Key::Kp8 => Keycode::Kp8,
            rdev::Key::Kp9 => Keycode::Kp9,
            rdev::Key::KpReturn => Keycode::KpReturn,
            rdev::Key::KpMinus => Keycode::KpMinus,
            rdev::Key::KpPlus => Keycode::KpPlus,
            rdev::Key::KpMultiply => Keycode::KpMultiply,
            rdev::Key::KpDivide => Keycode::KpDivide,
            rdev::Key::KpDecimal => Keycode::KpDecimal,
            rdev::Key::KpEqual => Keycode::KpEqual,
            rdev::Key::KpComma => Keycode::KpComma,

            // letters
            rdev::Key::KeyA => Keycode::A,
            rdev::Key::KeyB => Keycode::B,
            rdev::Key::KeyC => Keycode::C,
            rdev::Key::KeyD => Keycode::D,
            rdev::Key::KeyE => Keycode::E,
            rdev::Key::KeyF => Keycode::F,
            rdev::Key::KeyG => Keycode::G,
            rdev::Key::KeyH => Keycode::H,
            rdev::Key::KeyI => Keycode::I,
            rdev::Key::KeyJ => Keycode::J,
            rdev::Key::KeyK => Keycode::K,
            rdev::Key::KeyL => Keycode::L,
            rdev::Key::KeyM => Keycode::M,
            rdev::Key::KeyN => Keycode::N,
            rdev::Key::KeyO => Keycode::O,
            rdev::Key::KeyP => Keycode::P,
            rdev::Key::KeyQ => Keycode::Q,
            rdev::Key::KeyR => Keycode::R,
            rdev::Key::KeyS => Keycode::S,
            rdev::Key::KeyT => Keycode::T,
            rdev::Key::KeyU => Keycode::U,
            rdev::Key::KeyV => Keycode::V,
            rdev::Key::KeyW => Keycode::W,
            rdev::Key::KeyX => Keycode::X,
            rdev::Key::KeyY => Keycode::Y,
            rdev::Key::KeyZ => Keycode::Z,
            // F keys
            rdev::Key::F1 => Keycode::F1,
            rdev::Key::F2 => Keycode::F2,
            rdev::Key::F3 => Keycode::F3,
            rdev::Key::F4 => Keycode::F4,
            rdev::Key::F5 => Keycode::F5,
            rdev::Key::F6 => Keycode::F6,
            rdev::Key::F7 => Keycode::F7,
            rdev::Key::F8 => Keycode::F8,
            rdev::Key::F9 => Keycode::F9,
            rdev::Key::F10 => Keycode::F10,
            rdev::Key::F11 => Keycode::F11,
            rdev::Key::F12 => Keycode::F12,
            rdev::Key::F13 => Keycode::F13,
            rdev::Key::F14 => Keycode::F14,
            rdev::Key::F15 => Keycode::F15,
            rdev::Key::F16 => Keycode::F16,
            rdev::Key::F17 => Keycode::F17,
            rdev::Key::F18 => Keycode::F18,
            rdev::Key::F19 => Keycode::F19,
            rdev::Key::F20 => Keycode::F20,
            rdev::Key::F21 => Keycode::F21,
            rdev::Key::F22 => Keycode::F22,
            rdev::Key::F23 => Keycode::F23,
            rdev::Key::F24 => Keycode::F24,

            // Arrow keys
            rdev::Key::UpArrow => Keycode::UpArrow,
            rdev::Key::DownArrow => Keycode::DownArrow,
            rdev::Key::LeftArrow => Keycode::LeftArrow,
            rdev::Key::RightArrow => Keycode::RightArrow,

            // Symbols
            rdev::Key::LeftBracket => Keycode::LeftBracket,
            rdev::Key::RightBracket => Keycode::RightBracket,
            rdev::Key::BackQuote => Keycode::BackQuote,
            rdev::Key::SemiColon => Keycode::SemiColon,
            rdev::Key::Quote => Keycode::Quote,
            rdev::Key::BackSlash => Keycode::BackSlash,
            rdev::Key::IntlBackslash => Keycode::IntlBackslash,
            rdev::Key::Slash => Keycode::Slash,
            rdev::Key::Comma => Keycode::Comma,
            rdev::Key::Dot => Keycode::Period,
            rdev::Key::Minus => Keycode::Minus,
            rdev::Key::Equal => Keycode::Equal,

            // Volume keys
            rdev::Key::VolumeUp => Keycode::VolumeUp,
            rdev::Key::VolumeDown => Keycode::VolumeDown,
            rdev::Key::VolumeMute => Keycode::VolumeMute,

            // Modifier keys
            rdev::Key::ShiftLeft => Keycode::LeftShift,
            rdev::Key::ShiftRight => Keycode::RightShift,
            rdev::Key::ControlLeft => Keycode::LeftControl,
            rdev::Key::ControlRight => Keycode::RightControl,
            rdev::Key::Alt => Keycode::LeftAlt,
            rdev::Key::AltGr => Keycode::RightAlt,
            rdev::Key::MetaLeft => Keycode::LeftMeta,
            rdev::Key::MetaRight => Keycode::RightMeta,
            rdev::Key::CapsLock => Keycode::CapsLock,

            // Other keys
            rdev::Key::Escape => Keycode::Escape,
            rdev::Key::Delete => Keycode::Delete,
            rdev::Key::Backspace => Keycode::Backspace,
            rdev::Key::Space => Keycode::Space,
            rdev::Key::Return => Keycode::Enter,
            rdev::Key::Tab => Keycode::Tab,
            rdev::Key::PageUp => Keycode::PageUp,
            rdev::Key::PageDown => Keycode::PageDown,
            rdev::Key::Home => Keycode::Home,
            rdev::Key::End => Keycode::End,
            rdev::Key::Insert => Keycode::Insert,
            rdev::Key::PrintScreen => Keycode::PrintScreen,
            rdev::Key::ScrollLock => Keycode::ScrollLock,
            rdev::Key::Pause => Keycode::Pause,
            rdev::Key::NumLock => Keycode::NumLock,
            rdev::Key::Function => Keycode::Function,

            // Unknown keys
            rdev::Key::IntlRo => Keycode::Unknown(UnknownKeyCode::Str("IntlRo".to_string())),
            rdev::Key::IntlYen => Keycode::Unknown(UnknownKeyCode::Str("IntlYen".to_string())),
            rdev::Key::KanaMode => Keycode::Unknown(UnknownKeyCode::Str("KanaMode".to_string())),

            rdev::Key::Lang1 => Keycode::Unknown(UnknownKeyCode::Str("Lang1".to_string())),
            rdev::Key::Lang2 => Keycode::Unknown(UnknownKeyCode::Str("Lang2".to_string())),
            rdev::Key::Lang3 => Keycode::Unknown(UnknownKeyCode::Str("Lang3".to_string())),
            rdev::Key::Lang4 => Keycode::Unknown(UnknownKeyCode::Str("Lang4".to_string())),
            rdev::Key::Lang5 => Keycode::Unknown(UnknownKeyCode::Str("Lang5".to_string())),
            rdev::Key::Apps => Keycode::Unknown(UnknownKeyCode::Str("Apps".to_string())),
            rdev::Key::Cancel => Keycode::Unknown(UnknownKeyCode::Str("Cancel".to_string())),
            rdev::Key::Clear => Keycode::Unknown(UnknownKeyCode::Str("Clear".to_string())),
            rdev::Key::Kana => Keycode::Unknown(UnknownKeyCode::Str("Kana".to_string())),
            rdev::Key::Hangul => Keycode::Unknown(UnknownKeyCode::Str("Hangul".to_string())),
            rdev::Key::Junja => Keycode::Unknown(UnknownKeyCode::Str("Junja".to_string())),
            rdev::Key::Final => Keycode::Unknown(UnknownKeyCode::Str("Final".to_string())),
            rdev::Key::Hanja => Keycode::Unknown(UnknownKeyCode::Str("Hanja".to_string())),
            rdev::Key::Hanji => Keycode::Unknown(UnknownKeyCode::Str("Hanji".to_string())),
            rdev::Key::Print => Keycode::Unknown(UnknownKeyCode::Str("Print".to_string())),
            rdev::Key::Select => Keycode::Unknown(UnknownKeyCode::Str("Select".to_string())),
            rdev::Key::Execute => Keycode::Unknown(UnknownKeyCode::Str("Execute".to_string())),
            rdev::Key::Help => Keycode::Unknown(UnknownKeyCode::Str("Help".to_string())),
            rdev::Key::Sleep => Keycode::Unknown(UnknownKeyCode::Str("Sleep".to_string())),
            rdev::Key::Separator => Keycode::Unknown(UnknownKeyCode::Str("Separator".to_string())),
            rdev::Key::Unknown(id) => Keycode::Unknown(UnknownKeyCode::Int(id)),
            rdev::Key::RawKey(raw_key) => match raw_key {
                rdev::RawKey::ScanCode(code) => Keycode::Unknown(UnknownKeyCode::Int(code)),
                rdev::RawKey::WinVirtualKeycode(code) =>
                {
                    #[allow(clippy::unnecessary_cast)]
                    Keycode::Unknown(UnknownKeyCode::Int(code as u32))
                }
                rdev::RawKey::LinuxXorgKeycode(code) =>
                {
                    #[allow(clippy::unnecessary_cast)]
                    Keycode::Unknown(UnknownKeyCode::Int(code as u32))
                }
                rdev::RawKey::LinuxConsoleKeycode(code) =>
                {
                    #[allow(clippy::unnecessary_cast)]
                    Keycode::Unknown(UnknownKeyCode::Int(code as u32))
                }
                rdev::RawKey::MacVirtualKeycode(code) =>
                {
                    #[allow(clippy::unnecessary_cast)]
                    Keycode::Unknown(UnknownKeyCode::Int(code as u32))
                }
            },
        }
    }
}

impl TryInto<rdev::Key> for Keycode {
    type Error = UnknownKeyCode;

    fn try_into(self) -> Result<rdev::Key, Self::Error> {
        match self {
            // numbers
            Keycode::Num1 => Ok(rdev::Key::Num1),
            Keycode::Num2 => Ok(rdev::Key::Num2),
            Keycode::Num3 => Ok(rdev::Key::Num3),
            Keycode::Num4 => Ok(rdev::Key::Num4),
            Keycode::Num5 => Ok(rdev::Key::Num5),
            Keycode::Num6 => Ok(rdev::Key::Num6),
            Keycode::Num7 => Ok(rdev::Key::Num7),
            Keycode::Num8 => Ok(rdev::Key::Num8),
            Keycode::Num9 => Ok(rdev::Key::Num9),
            Keycode::Num0 => Ok(rdev::Key::Num0),

            // Keypad
            Keycode::Kp0 => Ok(rdev::Key::Kp0),
            Keycode::Kp1 => Ok(rdev::Key::Kp1),
            Keycode::Kp2 => Ok(rdev::Key::Kp2),
            Keycode::Kp3 => Ok(rdev::Key::Kp3),
            Keycode::Kp4 => Ok(rdev::Key::Kp4),
            Keycode::Kp5 => Ok(rdev::Key::Kp5),
            Keycode::Kp6 => Ok(rdev::Key::Kp6),
            Keycode::Kp7 => Ok(rdev::Key::Kp7),
            Keycode::Kp8 => Ok(rdev::Key::Kp8),
            Keycode::Kp9 => Ok(rdev::Key::Kp9),
            Keycode::KpReturn => Ok(rdev::Key::KpReturn),
            Keycode::KpMinus => Ok(rdev::Key::KpMinus),
            Keycode::KpPlus => Ok(rdev::Key::KpPlus),
            Keycode::KpMultiply => Ok(rdev::Key::KpMultiply),
            Keycode::KpDivide => Ok(rdev::Key::KpDivide),
            Keycode::KpDecimal => Ok(rdev::Key::KpDecimal),
            Keycode::KpEqual => Ok(rdev::Key::KpEqual),
            Keycode::KpComma => Ok(rdev::Key::KpComma),
            Keycode::KpEnter => Ok(rdev::Key::KpReturn),

            // letters
            Keycode::A => Ok(rdev::Key::KeyA),
            Keycode::B => Ok(rdev::Key::KeyB),
            Keycode::C => Ok(rdev::Key::KeyC),
            Keycode::D => Ok(rdev::Key::KeyD),
            Keycode::E => Ok(rdev::Key::KeyE),
            Keycode::F => Ok(rdev::Key::KeyF),
            Keycode::G => Ok(rdev::Key::KeyG),
            Keycode::H => Ok(rdev::Key::KeyH),
            Keycode::I => Ok(rdev::Key::KeyI),
            Keycode::J => Ok(rdev::Key::KeyJ),
            Keycode::K => Ok(rdev::Key::KeyK),
            Keycode::L => Ok(rdev::Key::KeyL),
            Keycode::M => Ok(rdev::Key::KeyM),
            Keycode::N => Ok(rdev::Key::KeyN),
            Keycode::O => Ok(rdev::Key::KeyO),
            Keycode::P => Ok(rdev::Key::KeyP),
            Keycode::Q => Ok(rdev::Key::KeyQ),
            Keycode::R => Ok(rdev::Key::KeyR),
            Keycode::S => Ok(rdev::Key::KeyS),
            Keycode::T => Ok(rdev::Key::KeyT),
            Keycode::U => Ok(rdev::Key::KeyU),
            Keycode::V => Ok(rdev::Key::KeyV),
            Keycode::W => Ok(rdev::Key::KeyW),
            Keycode::X => Ok(rdev::Key::KeyX),
            Keycode::Y => Ok(rdev::Key::KeyY),
            Keycode::Z => Ok(rdev::Key::KeyZ),
            // F keys
            Keycode::F1 => Ok(rdev::Key::F1),
            Keycode::F2 => Ok(rdev::Key::F2),
            Keycode::F3 => Ok(rdev::Key::F3),
            Keycode::F4 => Ok(rdev::Key::F4),
            Keycode::F5 => Ok(rdev::Key::F5),
            Keycode::F6 => Ok(rdev::Key::F6),
            Keycode::F7 => Ok(rdev::Key::F7),
            Keycode::F8 => Ok(rdev::Key::F8),
            Keycode::F9 => Ok(rdev::Key::F9),
            Keycode::F10 => Ok(rdev::Key::F10),
            Keycode::F11 => Ok(rdev::Key::F11),
            Keycode::F12 => Ok(rdev::Key::F12),
            Keycode::F13 => Ok(rdev::Key::F13),
            Keycode::F14 => Ok(rdev::Key::F14),
            Keycode::F15 => Ok(rdev::Key::F15),
            Keycode::F16 => Ok(rdev::Key::F16),
            Keycode::F17 => Ok(rdev::Key::F17),
            Keycode::F18 => Ok(rdev::Key::F18),
            Keycode::F19 => Ok(rdev::Key::F19),
            Keycode::F20 => Ok(rdev::Key::F20),
            Keycode::F21 => Ok(rdev::Key::F21),
            Keycode::F22 => Ok(rdev::Key::F22),
            Keycode::F23 => Ok(rdev::Key::F23),
            Keycode::F24 => Ok(rdev::Key::F24),

            // Arrow keys
            Keycode::UpArrow => Ok(rdev::Key::UpArrow),
            Keycode::DownArrow => Ok(rdev::Key::DownArrow),
            Keycode::LeftArrow => Ok(rdev::Key::LeftArrow),
            Keycode::RightArrow => Ok(rdev::Key::RightArrow),

            // Symbols
            Keycode::LeftBracket => Ok(rdev::Key::LeftBracket),
            Keycode::RightBracket => Ok(rdev::Key::RightBracket),
            Keycode::BackQuote => Ok(rdev::Key::BackQuote),
            Keycode::SemiColon => Ok(rdev::Key::SemiColon),
            Keycode::Quote => Ok(rdev::Key::Quote),
            Keycode::BackSlash => Ok(rdev::Key::BackSlash),
            Keycode::IntlBackslash => Ok(rdev::Key::IntlBackslash),
            Keycode::Slash => Ok(rdev::Key::Slash),
            Keycode::Comma => Ok(rdev::Key::Comma),
            Keycode::Period => Ok(rdev::Key::Dot),
            Keycode::Minus => Ok(rdev::Key::Minus),
            Keycode::Equal => Ok(rdev::Key::Equal),

            // Volume keys
            Keycode::VolumeUp => Ok(rdev::Key::VolumeUp),
            Keycode::VolumeDown => Ok(rdev::Key::VolumeDown),
            Keycode::VolumeMute => Ok(rdev::Key::VolumeMute),

            // Modifier keys
            Keycode::LeftShift => Ok(rdev::Key::ShiftLeft),
            Keycode::RightShift => Ok(rdev::Key::ShiftRight),
            Keycode::LeftControl => Ok(rdev::Key::ControlLeft),
            Keycode::RightControl => Ok(rdev::Key::ControlRight),
            Keycode::LeftAlt => Ok(rdev::Key::Alt),
            Keycode::RightAlt => Ok(rdev::Key::AltGr),
            Keycode::LeftMeta => Ok(rdev::Key::MetaLeft),
            Keycode::RightMeta => Ok(rdev::Key::MetaRight),
            Keycode::CapsLock => Ok(rdev::Key::CapsLock),

            // Other keys
            Keycode::Escape => Ok(rdev::Key::Escape),
            Keycode::Delete => Ok(rdev::Key::Delete),
            Keycode::Backspace => Ok(rdev::Key::Backspace),
            Keycode::Space => Ok(rdev::Key::Space),
            Keycode::Enter => Ok(rdev::Key::Return),
            Keycode::Tab => Ok(rdev::Key::Tab),
            Keycode::PageUp => Ok(rdev::Key::PageUp),
            Keycode::PageDown => Ok(rdev::Key::PageDown),
            Keycode::Home => Ok(rdev::Key::Home),
            Keycode::End => Ok(rdev::Key::End),
            Keycode::Insert => Ok(rdev::Key::Insert),
            Keycode::PrintScreen => Ok(rdev::Key::PrintScreen),
            Keycode::ScrollLock => Ok(rdev::Key::ScrollLock),
            Keycode::Pause => Ok(rdev::Key::Pause),
            Keycode::NumLock => Ok(rdev::Key::NumLock),
            Keycode::Function => Ok(rdev::Key::Function),

            Keycode::Unknown(UnknownKeyCode::Str(s)) => match s.as_str() {
                "IntlRo" => Ok(rdev::Key::IntlRo),
                "IntlYen" => Ok(rdev::Key::IntlYen),
                "KanaMode" => Ok(rdev::Key::KanaMode),
                "Lang1" => Ok(rdev::Key::Lang1),
                "Lang2" => Ok(rdev::Key::Lang2),
                "Lang3" => Ok(rdev::Key::Lang3),
                "Lang4" => Ok(rdev::Key::Lang4),
                "Lang5" => Ok(rdev::Key::Lang5),
                "Apps" => Ok(rdev::Key::Apps),
                "Cancel" => Ok(rdev::Key::Cancel),
                "Clear" => Ok(rdev::Key::Clear),
                "Kana" => Ok(rdev::Key::Kana),
                "Hangul" => Ok(rdev::Key::Hangul),
                "Junja" => Ok(rdev::Key::Junja),
                "Final" => Ok(rdev::Key::Final),
                "Hanja" => Ok(rdev::Key::Hanja),
                "Hanji" => Ok(rdev::Key::Hanji),
                "Print" => Ok(rdev::Key::Print),
                "Select" => Ok(rdev::Key::Select),
                "Execute" => Ok(rdev::Key::Execute),
                "Help" => Ok(rdev::Key::Help),
                "Sleep" => Ok(rdev::Key::Sleep),
                "Separator" => Ok(rdev::Key::Separator),
                _ => Err(UnknownKeyCode::Str(s)),
            },
            Keycode::Unknown(UnknownKeyCode::Int(id)) => Ok(rdev::Key::Unknown(id)),
        }
    }
}

impl From<rdev::Button> for Button {
    fn from(value: rdev::Button) -> Self {
        match value {
            rdev::Button::Left => Button::Left,
            rdev::Button::Middle => Button::Middle,
            rdev::Button::Right => Button::Right,
            rdev::Button::Unknown(id) => Button::Unknown(id),
        }
    }
}

impl From<Button> for rdev::Button {
    fn from(value: Button) -> Self {
        match value {
            Button::Left => rdev::Button::Left,
            Button::Middle => rdev::Button::Middle,
            Button::Right => rdev::Button::Right,
            Button::Unknown(id) => rdev::Button::Unknown(id),
        }
    }
}
