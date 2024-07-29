#![allow(dead_code)]

pub fn match_number(character: char) -> bool {
  "1234567890.".contains(character)
}
pub fn highlight_red(text: &str) -> String {
  format!("\x1b[31m{}\x1b[0m", text)
}

pub fn highlight_yellow(text: &str) -> String {
  format!("\x1b[33m{}\x1b[0m", text)
}

pub fn highlight_green(text: &str) -> String {
  format!("\x1b[32m{}\x1b[0m", text)
}

pub fn highlight_blue(text: &str) -> String {
  format!("\x1b[34m{}\x1b[0m", text)
}

pub fn highlight_magenta(text: &str) -> String {
  format!("\x1b[35m{}\x1b[0m", text)
}

pub fn highlight_cyan(text: &str) -> String {
  format!("\x1b[36m{}\x1b[0m", text)
}

pub fn highlight_white(text: &str) -> String {
  format!("\x1b[37m{}\x1b[0m", text)
}

pub fn highlight_gray(text: &str) -> String {
  format!("\x1b[90m{}\x1b[0m", text)
}
