#![allow(dead_code)]
use crate::lexer::tokens::Range;
use crate::utils::highlight_cyan;
use crate::utils::highlight_red;

pub fn report_lexer_diagnostics(message: &str, raw: &str, range: Range, file_name: &str) -> ! {
  println!("");
  println!("{}", highlight_red(&format!("ERROR: {}", message)));
  let text_file_highlighted = highlight_cyan(file_name);
  println!("{}", text_file_highlighted);
  println!("{}", code_highlighter::highlight_error(range.start, range.end, raw));
  std::process::exit(1);
}
