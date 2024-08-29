use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Range {
  pub start: usize,
  pub end: usize,
}

impl Range {
  pub fn new(start: usize, end: usize) -> Self {
    Self { start, end }
  }
}
