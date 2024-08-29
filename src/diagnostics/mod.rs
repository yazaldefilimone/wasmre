#![allow(dead_code)]

mod format_syntax_error;
mod format_type_error;
mod reporter;
pub use reporter::*;

use crate::utils::range::Range;

pub type ResultWithDiagnostics<T> = std::result::Result<T, Diagnostic>;

pub enum Severity {
  Error,
  Warning,
}

pub struct Diagnostic {
  pub severity: Severity,
  pub message: String,
  pub hint: Option<String>,
  pub range: Option<Range>,
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
  UnknownCharacter {
    name: String,
    range: Range,
  },
}

impl From<SintaxError> for Diagnostic {
  fn from(error: SintaxError) -> Self {
    match error {
      SintaxError::UnxpectedToken { expected, found, range } => {
        let message = format_syntax_error::format_unexpected_token(&expected, &found);
        Diagnostic { severity: Severity::Error, message, range: Some(range), hint: None }
      }
      SintaxError::UnknownCharacter { name, range } => {
        let message = format_syntax_error::format_unkon_character(&name);
        Diagnostic { severity: Severity::Error, message, range: Some(range), hint: None }
      }
    }
  }
}

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
      TypeError::TypeMismatch { expected, found, range } => {
        let message = format_type_error::format_type_mismatch(&expected, &found);
        Diagnostic { severity: Severity::Error, message, range: Some(range), hint: None }
      }
      TypeError::UnknownType { name, range } => {
        let message = format_type_error::format_unknown_type(&name);
        Diagnostic { severity: Severity::Error, message, range: Some(range), hint: None }
      }
    }
  }
}

pub enum RuntimeError {
  UnknownFunction {
    name: String,
    range: Option<Range>,
  },
  UnknownTable {
    name: String,
    range: Option<Range>,
  },
  UnknownMemory {
    name: String,
    range: Option<Range>,
  },
  TableOutOfBounds {
    index: u32,
    range: Option<Range>,
  },
  MemoryOutOfBounds {
    offset: u32,
    range: Option<Range>,
  },
  TypeMismatch {
    expected: String,
    found: String,
    range: Option<Range>,
  },
  StackOverflow {
    range: Option<Range>,
  },
  CallIndirect {
    type_idx: u32,
    range: Option<Range>,
  },
  FailedToDecodeModule {
    range: Option<Range>,
    cause: String,
  },
}

impl From<RuntimeError> for Diagnostic {
  fn from(error: RuntimeError) -> Self {
    match error {
      RuntimeError::UnknownFunction { name, range } => {
        let message = format!("unknown function `{}`", name);
        Diagnostic { severity: Severity::Error, message, range, hint: None }
      }
      RuntimeError::UnknownTable { name, range } => {
        let message = format!("unknown table `{}`", name);
        Diagnostic { severity: Severity::Error, message, range, hint: None }
      }
      RuntimeError::UnknownMemory { name, range } => {
        let message = format!("unknown memory `{}`", name);
        Diagnostic { severity: Severity::Error, message, range, hint: None }
      }
      RuntimeError::TableOutOfBounds { index, range } => {
        let message = format!("table out of bounds, index = {}", index);
        Diagnostic { severity: Severity::Error, message, range, hint: None }
      }
      RuntimeError::MemoryOutOfBounds { offset, range } => {
        let message = format!("memory out of bounds, offset = {}", offset);
        Diagnostic { severity: Severity::Error, message, range, hint: None }
      }
      RuntimeError::TypeMismatch { expected, found, range } => {
        let message = format!("type mismatch: expected `{}` but found `{}`", expected, found);
        Diagnostic { severity: Severity::Error, message, range, hint: None }
      }
      RuntimeError::StackOverflow { range } => {
        let message = "stack overflow".to_string();
        Diagnostic { severity: Severity::Error, message, range, hint: None }
      }
      RuntimeError::CallIndirect { type_idx, range } => {
        let message = format!("call indirect: type index = {}", type_idx);
        Diagnostic { severity: Severity::Error, message, range, hint: None }
      }
      RuntimeError::FailedToDecodeModule { range, cause } => {
        let message = format!("failed to decode module: {}", cause);
        Diagnostic { severity: Severity::Error, message, range, hint: None }
      }
    }
  }
}
