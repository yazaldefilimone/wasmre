#![allow(dead_code)]

use super::ast;
use crate::{diagnostics::Diagnostic, lexer::Lexer};

type Result<T> = std::result::Result<T, Diagnostic>;

pub struct Parser<'a> {
  lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
  pub fn new(lexer: Lexer<'a>) -> Self {
    Self { lexer }
  }

  pub fn parse(&mut self) -> Result<ast::Program> {
    self.parse_program()
  }

  pub fn parse_program(&mut self) -> Result<ast::Program> {
    let mut program = ast::Program::default();
    while !self.lexer.is_end() {
      let module = self.parse_module()?;
      program.body.push(module);
    }
    Ok(program)
  }

  pub fn parse_module(&mut self) -> Result<ast::Module> {
    todo!()
  }
}
