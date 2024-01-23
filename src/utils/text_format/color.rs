use crate::strike;

#[repr(u8)]
pub enum Color {
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,
}


pub fn color(s: String, color_code: Color) -> String {
    format!("\x1b[0;{}m{}\x1b[0m", color_code as u8, s)
}


pub fn red(s: String) -> String {
    color(s, Color::Red)
}


pub fn green(s: String) -> String {
    color(s, Color::Green)
}


pub fn yellow(s: String) -> String {
    color(s, Color::Yellow)
}


pub fn green_strike(s: String) -> String {
    color(
        strike(s),
        Color::Green
    )
}
