const PREFIX: &str = "syntax error: ";

pub fn format_unexpected_token(expected: &str, actual: &str) -> String {
  format!("{} expected `{}`, but found `{}`", PREFIX, expected, actual)
}

pub fn format_unkon_character(character: &str) -> String {
  format!("{} unknown character `{}`", PREFIX, character)
}
