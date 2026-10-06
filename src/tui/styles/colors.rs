use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorCapability {
    Ansi16,
    Ansi256,
    TrueColor
}

pub struct ColorPalette {
    pub text: Color,
    pub muted: Color,
    pub group: Color,
    pub accent: Color,
    pub selected_fg: Color,
    pub selected_bg: Color,
    pub border: Color,
}

impl ColorPalette {
    pub fn from_capability(capability: ColorCapability) -> Self {
        match capability {
            ColorCapability::Ansi16 => Self::ansi16(),
            ColorCapability::Ansi256 => Self::ansi256(),
            ColorCapability::TrueColor => Self::truecolor(),
        }
    }

    fn ansi16() -> Self {
        Self {
            text: Color::White,
            muted: Color::DarkGray,
            group: Color::Gray,
            accent: Color::Cyan,
            selected_fg: Color::Black,
            selected_bg: Color::Cyan,
            border: Color::DarkGray,
        }
    }

    fn ansi256() -> Self {
        Self {
            text: Color::Indexed(253),
            muted: Color::Indexed(243),
            group: Color::Indexed(250),
            accent: Color::Indexed(117),
            selected_fg: Color::Indexed(16),
            selected_bg: Color::Indexed(75),
            border: Color::Indexed(239),
        }
    }

    fn truecolor() -> Self {
        Self {
            text: Color::Rgb(205, 214, 244),
            muted: Color::Rgb(127, 132, 156),
            group: Color::Rgb(166, 173, 200),
            accent: Color::Rgb(137, 180, 250),
            selected_fg: Color::Rgb(20, 20, 30),
            selected_bg: Color::Rgb(69, 71, 90),
            border: Color::Rgb(88, 91, 112),
        }
    }
}

fn is_dumb_terminal() -> bool {
    matches!(
        std::env::var("TERM").as_deref(),
        Ok("dumb") | Ok("")
    )
}

fn supports_truecolor() -> bool {
    matches!(
        std::env::var("COLORTERM").as_deref(),
        Ok("truecolor") | Ok("24bit")
    )
}

fn supports_ansi256() -> bool {
    std::env::var("TERM")
        .map(|term| term.contains("256color"))
        .unwrap_or(false)
}

pub fn detect_color_capability() -> ColorCapability {
    if is_dumb_terminal() {
        return ColorCapability::Ansi16;
    }

    if supports_truecolor() {
        ColorCapability::TrueColor
    } else if supports_ansi256() {
        ColorCapability::Ansi256
    } else {
        ColorCapability::Ansi16
    }
}
