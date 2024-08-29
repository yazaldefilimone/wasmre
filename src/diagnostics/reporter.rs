#![allow(dead_code)]
use crate::utils::highlight_cyan;
use crate::utils::highlight_red;
use crate::utils::highlight_yellow;
use crate::utils::range::Range;

use super::Diagnostic;
use super::Severity;

pub fn report_lexer_diagnostics(message: &str, raw: &str, range: Range, file_name: &str) -> ! {
  println!("");
  println!("{}", highlight_red(&format!("ERROR: {}", message)));
  let text_file_highlighted = highlight_cyan(file_name);
  println!("{}", text_file_highlighted);
  println!("{}", code_highlighter::highlight_error(range.start, range.end, raw));
  std::process::exit(1);
}

pub fn report_diagnostic(diagnostics: &Diagnostic, raw: &str, file_name: &str) {
  match diagnostics.severity {
    Severity::Error => {
      report_error(&diagnostics.message, &diagnostics.range, file_name, raw);
    }
    Severity::Warning => {
      report_warning(&diagnostics.message, &diagnostics.range, file_name, raw);
    }
  }
}

pub fn report_warning(message: &str, range: &Option<Range>, file_name: &str, raw: &str) {
  println!("");
  println!("{}", highlight_yellow(&format!("WARNING: {}", message)));
  let text_file_highlighted = highlight_cyan(file_name);
  println!("{}", text_file_highlighted);
  if let Some(range) = range {
    let highlight = code_highlighter::highlight_error(range.start, range.end, raw);
    println!("{}", highlight);
  }
}

pub fn report_error(message: &str, range: &Option<Range>, file_name: &str, raw: &str) {
  println!("");
  println!("{}", highlight_red(&format!("ERROR: {}", message)));
  let text_file_highlighted = highlight_cyan(file_name);
  println!("{}", text_file_highlighted);
  if let Some(range) = range {
    let highlight = code_highlighter::highlight_error(range.start, range.end, raw);
    println!("{}", highlight);
  }
}
