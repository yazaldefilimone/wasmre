// const PREFIX: &str = "type: ";

pub fn format_type_mismatch(expected: &str, actual: &str) -> String {
  format!("expected `{}`, but found `{}`", expected, actual)
}

pub fn format_unknown_type(name: &str) -> String {
  format!("unknown type `{}`", name)
}
