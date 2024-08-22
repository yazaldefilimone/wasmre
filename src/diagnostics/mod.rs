#![allow(dead_code)]

mod reporter;
pub use reporter::*;

use crate::lexer::tokens::Range;

pub enum Severity {
  Error,
  Warning,
}

pub struct Diagnostic {
  pub severity: Severity,
  pub message: String,
  pub range: Range,
}

pub struct DiagnosticManager<'a> {
  diagnostics: Vec<Diagnostic>,
  pub file_name: &'a str,
  pub raw: &'a str,
}

impl<'a> DiagnosticManager<'a> {
  pub fn new(raw: &'a str, file_name: &'a str) -> Self {
    Self { diagnostics: vec![], file_name, raw }
  }

  pub fn add(&mut self, diagnostic: Diagnostic) {
    self.diagnostics.push(diagnostic);
  }

  pub fn add_sintax_error(&mut self, error: SintaxError) {
    self.add(error.into());
  }

  pub fn add_type_error(&mut self, error: TypeError) {
    self.add(error.into());
  }

  pub fn report(&self) {
    for diagnostic in &self.diagnostics {
      report_diagnostic(diagnostic, self.raw, self.file_name);
    }
  }
}

#[derive(Debug, Clone)]
pub enum SintaxError {
  UnxpectedToken {
    expected: String,
    found: String,
    range: Range,
  },
  UnknownToken {
    name: String,
    range: Range,
  },
}

impl From<SintaxError> for Diagnostic {
  fn from(error: SintaxError) -> Self {
    match error {
      SintaxError::UnxpectedToken { expected, found, range } => Diagnostic {
        severity: Severity::Error,
        message: format!(
          "syntax error: unexpected token, expected `{}` but found `{}`",
          expected, found
        ),
        range,
      },
      SintaxError::UnknownToken { name, range } => {
        Diagnostic { severity: Severity::Error, message: format!("ssyntax error: unknown token `{}`", name), range }
      }
    }
  }
}

//  type diagnostics
#[derive(Debug, Clone)]
pub enum TypeError {
  TypeMismatch {
    expected: String,
    found: String,
    range: Range,
  },
  UnknownType {
    name: String,
    range: Range,
  },
}
impl From<TypeError> for Diagnostic {
  fn from(error: TypeError) -> Self {
    match error {
      TypeError::TypeMismatch { expected, found, range } => Diagnostic {
        severity: Severity::Error,
        message: format!("type mismatch: expected `{}` but found `{}`", expected, found),
        range,
      },
      TypeError::UnknownType { name, range } => {
        Diagnostic { severity: Severity::Error, message: format!("unknown type `{}`", name), range }
      }
    }
  }
}
