#![allow(dead_code)]

use crate::{diagnostics::Diagnostic, lexer::Lexer};

use super::ast::Program;

type Result<T> = std::result::Result<T, Diagnostic>;

pub struct Parser<'a> {
  lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
  pub fn new(lexer: Lexer<'a>) -> Self {
    Self { lexer }
  }

  pub fn parse_program(&mut self) -> Result<Program> {
    todo!()
  }
  pub fn parse_module(&mut self) -> Result<Program> {
    todo!()
  }
  pub fn parse_type(&mut self) -> Result<Program> {
    todo!()
  }
  pub fn parse_import(&mut self) -> Result<Program> {
    todo!()
  }
  pub fn parse_function(&mut self) -> Result<Program> {
    todo!()
  }
  pub fn parse_table(&mut self) -> Result<Program> {
    todo!()
  }
  pub fn parse_memory(&mut self) -> Result<Program> {
    todo!()
  }
  pub fn parse_global(&mut self) -> Result<Program> {
    todo!()
  }
  pub fn parse_export(&mut self) -> Result<Program> {
    todo!()
  }
  pub fn parse_start(&mut self) -> Result<Program> {
    todo!()
  }
  pub fn parse_element(&mut self) -> Result<Program> {
    todo!()
  }
  pub fn parse_code(&mut self) -> Result<Program> {
    todo!()
  }
  pub fn parse_data(&mut self) -> Result<Program> {
    todo!()
  }
}
