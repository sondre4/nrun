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
    pub package: Color,
    pub primary: Color,
    pub secondary: Color,
    pub selected_fg: Color,
    pub selected_bg: Color,
    pub selected_package_bg: Color,
    pub border: Color
}

impl ColorPalette {
    pub fn from_capability(capability: ColorCapability) -> Self {
        match capability {
            ColorCapability::Ansi16 => Self::ansi16(),
            ColorCapability::Ansi256 => Self::ansi256(),
            ColorCapability::TrueColor => Self::truecolor()
        }
    }

    fn ansi16() -> Self {
        Self {
            text: Color::Gray,
            muted: Color::DarkGray,
            package: Color::White,
            primary: Color::Cyan,
            secondary: Color::LightCyan,
            selected_fg: Color::White,
            selected_bg: Color::DarkGray,
            selected_package_bg: Color::DarkGray,
            border: Color::DarkGray,
        }
    }

    fn ansi256() -> Self {
        Self {
            text: Color::Indexed(250),
            muted: Color::Indexed(242),
            package: Color::Indexed(255),
            primary: Color::Indexed(43),
            secondary: Color::Indexed(123),
            selected_fg: Color::Indexed(255),
            selected_bg: Color::Indexed(238),
            selected_package_bg: Color::Indexed(236),
            border: Color::Indexed(240)
        }
    }

    fn truecolor() -> Self {
        Self {
            text: Color::Rgb(190, 194, 198),
            muted: Color::Rgb(110, 115, 120),
            package: Color::Rgb(225, 228, 231),
            primary: Color::Rgb(70, 190, 180),
            secondary: Color::Rgb(145, 220, 218),
            selected_fg: Color::Rgb(235, 237, 239),
            selected_bg: Color::Rgb(55, 59, 63),
            selected_package_bg: Color::Rgb(43, 48, 51),
            border: Color::Rgb(75, 80, 84)
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
        return ColorCapability::TrueColor;
    } else if supports_ansi256() {
        return ColorCapability::Ansi256;
    } else {
        return ColorCapability::Ansi16;
    }
}
