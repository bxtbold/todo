pub fn bold(s: String) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

pub fn color(s: String, color_code: u8) -> String {
    format!("\x1b[0;{}m{}\x1b[0m", color_code, s)
}

pub fn strike(s: String) -> String {
    format!("\x1b[9m{}\x1b[0m", s)
}

pub fn underline(s: String) -> String {
    format!("\x1b[4m{}\x1b[0m", s)
}
