// https://en.wikipedia.org/wiki/ANSI_escape_code

pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack, // Gray
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

pub struct ColorString<'a>(pub &'a str, pub Color);

impl ColorString<'_> {
    pub fn fmt(&self, ansi_support: bool) -> String {
        if !ansi_support {
            self.0.to_string()
        } else {
            let fg = match self.1 {
                Color::Black => "30",
                Color::Red => "31",
                Color::Green => "32",
                Color::Yellow => "33",
                Color::Blue => "34",
                Color::Magenta => "35",
                Color::Cyan => "36",
                Color::White => "37",
                Color::BrightBlack => "90",
                Color::BrightRed => "91",
                Color::BrightGreen => "92",
                Color::BrightYellow => "93",
                Color::BrightBlue => "94",
                Color::BrightMagenta => "95",
                Color::BrightCyan => "96",
                Color::BrightWhite => "97",
            };

            format!("\u{001B}[{}m{}\u{001B}[0m", fg, self.0)
        }
    }
}
