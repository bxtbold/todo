pub fn italic(s: String) -> String {
    format!("\x1b[3m{}\x1b[0m", s)
}

pub fn bold(s: String) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

pub fn strike(s: String) -> String {
    format!("\x1b[9m{}\x1b[0m", s)
}

pub fn underline(s: String) -> String {
    format!("\x1b[4m{}\x1b[0m", s)
}
