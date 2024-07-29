#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TokenKind {
  LParen,             // '('
  RParen,             // ')'
  LBrace,             // '{'
  RBrace,             // '}'
  LBracket,           // '['
  RBracket,           // ']'
  Comma,              // ','
  Dot,                // '.'
  Plus,               // '+'
  Minus,              // '-'
  Star,               // '*'
  Slash,              // '/'
  Percent,            // '%'
  Caret,              // '^'
  Ampersand,          // '&'
  Bar,                // '|'
  Tilde,              // '~'
  Equal,              // '='
  Exclamation,        // '!'
  LessThan,           // '<'
  GreaterThan,        // '>'
  LessThanEqual,      // '<='
  GreaterThanEqual,   // '>='
  NotEqual,           // '!='
  And,                // '&&'
  Or,                 // '||'
  Question,           // '?'
  Colon,              // ':'
  Semicolon,          // ';'
  Arrow,              // '->'
  At,                 // '@'
  Hash,               // '#'
  Backslash,          // '\\'
  Dollar,             // '$'
  String(String),     // string
  Identifier(String), // identifier
  Number(String),     // number
  Comment(String),    // comment
  EOF,                // end of file
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Range {
  pub start: usize,
  pub end: usize,
}

impl Range {
  pub fn new(start: usize, end: usize) -> Self {
    Self { start, end }
  }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
  pub kind: TokenKind,
  pub range: Range,
}

impl Token {
  pub fn new(kind: TokenKind, range: Range) -> Self {
    Self { kind, range }
  }

  pub fn new_string(range: Range, string: String) -> Self {
    Self { kind: TokenKind::String(string), range }
  }

  pub fn new_identifier(range: Range, identifier: String) -> Self {
    Self { kind: TokenKind::Identifier(identifier), range }
  }

  pub fn new_number(range: Range, number: String) -> Self {
    Self { kind: TokenKind::Number(number), range }
  }

  pub fn new_eof(range: Range) -> Self {
    Self { kind: TokenKind::EOF, range }
  }

  pub fn new_comment(range: Range, comment: String) -> Self {
    Self { kind: TokenKind::Comment(comment), range }
  }
}
