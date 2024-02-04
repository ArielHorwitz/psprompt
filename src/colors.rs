#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn from_hex(hex: &str) -> Self {
        let (r, gb) = hex.split_at(2);
        let (g, b) = gb.split_at(2);
        let r = u8::from_str_radix(r, 16).expect("parse color red hex");
        let g = u8::from_str_radix(g, 16).expect("parse color green hex");
        let b = u8::from_str_radix(b, 16).expect("parse color blue hex");
        Self(r, g, b)
    }
}

pub const RESET: &str = r"\[\e[0m\]";

#[derive(Debug, Clone)]
pub struct StyledText {
    text: String,
    fg: Color,
}

impl StyledText {
    pub fn new(text: &str, fg: Color) -> Self {
        Self {
            text: text.to_owned(),
            fg,
        }
    }
}

impl std::fmt::Display for StyledText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let codes = [38, 2, self.fg.0, self.fg.1, self.fg.2]
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(";");
        write!(f, r"\[\e[{codes}m\]{}{RESET}", self.text)
    }
}
