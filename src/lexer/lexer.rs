#![allow(dead_code)]
use crate::{diagnostics::report_lexer_diagnostics, utils::match_number};

use super::tokens::{Range, Token};
use crate::lexer::tokens::TokenKind;

pub struct Lexer<'a> {
  pub raw: &'a str,
  cursor: usize,
  pub file_name: &'a str,
  start_cursor: usize,
}

impl<'a> Lexer<'a> {
  pub fn new(raw: &'a str, file_name: &'a str) -> Self {
    Self { raw, cursor: 0, file_name, start_cursor: 0 }
  }
  pub fn next_token(&mut self) -> Token {
    self.start_cursor = self.cursor;
    self.skip_whitespace();
    if self.is_end() {
      let range = self.create_range();
      return Token::new_eof(range);
    }
    let next_char = self.peek_one();
    match next_char {
      '0'..='9' => self.read_number(),
      'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(),
      '"' => self.read_string(),
      '(' => self.create_simple_token(TokenKind::LParen),
      ')' => self.create_simple_token(TokenKind::RParen),
      '+' => self.create_simple_token(TokenKind::Plus),
      '*' => self.create_simple_token(TokenKind::Star),
      '/' => self.create_simple_token(TokenKind::Slash),
      ',' => self.create_simple_token(TokenKind::Comma),
      '{' => self.create_simple_token(TokenKind::LBrace),
      '}' => self.create_simple_token(TokenKind::RBrace),
      '[' => self.create_simple_token(TokenKind::LBracket),
      ']' => self.create_simple_token(TokenKind::RBracket),
      '=' => self.create_simple_token(TokenKind::Equal),
      ':' => self.create_simple_token(TokenKind::Colon),
      '.' => self.create_simple_token(TokenKind::Dot),
      '?' => self.create_simple_token(TokenKind::Question),
      '<' => self.create_complex_token("<=", TokenKind::LessThanEqual, TokenKind::LessThan),
      '>' => self.create_complex_token(">=", TokenKind::GreaterThanEqual, TokenKind::GreaterThan),
      '!' => self.create_complex_token("!=", TokenKind::NotEqual, TokenKind::Exclamation),
      '|' => self.create_complex_token("||", TokenKind::Bar, TokenKind::Or),
      '&' => self.create_complex_token("&&", TokenKind::Ampersand, TokenKind::And),
      '-' => self.create_complex_token("->", TokenKind::Arrow, TokenKind::Minus),
      ';' => self.read_comment_or_semicolon(),
      '@' => self.create_simple_token(TokenKind::At),
      '#' => self.create_simple_token(TokenKind::Hash),
      '\\' => self.create_simple_token(TokenKind::Backslash),
      '$' => self.create_simple_token(TokenKind::Dollar),
      _ => {
        let text = format!("unknown character `{}`", next_char);
        self.report_diagnostic(text, self.create_range())
      }
    }
  }

  fn read_comment_or_semicolon(&mut self) -> Token {
    if self.starts_with(";;") {
      return self.read_comment();
    }
    return self.create_simple_token(TokenKind::Semicolon);
  }

  fn read_comment(&mut self) -> Token {
    self.consume_expect(";;");
    let text = self.read_while(|c| c != '\n');
    let range = self.create_range();
    Token::new_comment(range, text)
  }

  fn create_simple_token(&mut self, token_kind: TokenKind) -> Token {
    let range = self.create_range();
    self.advance_one();
    return Token::new(token_kind, range);
  }

  fn create_complex_token(&mut self, text: &str, single_kind: TokenKind, double_kind: TokenKind) -> Token {
    let range = self.create_range();
    if self.starts_with(text) {
      self.advance_many(text.len());
      return Token::new(single_kind, range);
    } else {
      self.advance_one();
      return Token::new(double_kind, range);
    }
  }

  fn read_number(&mut self) -> Token {
    let value = self.read_while(|c| match_number(c));
    let range = self.create_range();
    Token::new_number(range, value)
  }

  fn read_string(&mut self) -> Token {
    self.consume_expect("\"");
    let text = self.read_while(|c| c != '"');
    self.consume_expect("\"");
    let range = self.create_range();
    Token::new_string(range, text)
  }

  fn read_identifier(&mut self) -> Token {
    let text = self.read_while(|c| c.is_ascii_alphabetic() || c == '_');
    let range = self.create_range();
    Token::new_identifier(range, text)
  }

  fn create_range(&self) -> Range {
    Range { start: self.start_cursor, end: self.cursor }
  }

  fn consume_expect(&mut self, expected: &str) {
    let atual = self.peek_many(expected.len());
    if atual != expected {
      let text = format!("expected `{}` but found `{}`", expected, atual);
      self.advance_many(expected.len());
      self.report_diagnostic(text, self.create_range());
    }
    self.advance_many(expected.len());
  }

  pub fn read_while(&mut self, predicate: fn(char) -> bool) -> String {
    let mut result = String::new();
    while !self.is_end() && predicate(self.peek_one()) {
      let character = self.consume_char();
      result.push(character);
    }
    result
  }

  fn skip_whitespace(&mut self) {
    while !self.is_end() && self.peek_one().is_whitespace() {
      self.advance_one();
    }
  }

  fn is_end(&self) -> bool {
    self.cursor >= self.raw.len()
  }

  fn advance_one(&mut self) {
    self.cursor += 1;
  }

  fn starts_with(&self, expected: &str) -> bool {
    self.peek_many(expected.len()) == expected
  }

  fn peek_one(&self) -> char {
    if self.is_end() {
      return '\0';
    }
    self.raw[self.cursor..].chars().next().unwrap()
  }

  fn consume_char(&mut self) -> char {
    let result = self.peek_one();
    self.advance_one();
    result
  }

  fn peek_many(&self, count: usize) -> &str {
    if self.is_end() || self.cursor + count > self.raw.len() {
      return &self.raw[self.cursor..].chars().as_str();
    }
    &self.raw[self.cursor..self.cursor + count].chars().as_str()
  }

  fn advance_many(&mut self, count: usize) {
    self.cursor += count;
  }

  fn report_diagnostic(&self, message: String, range: Range) -> ! {
    report_lexer_diagnostics(&message, &self.raw, range, &self.file_name)
  }
}
