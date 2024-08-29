use num_derive::FromPrimitive;
#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum SectionCode {
  Type = 0x01,
  Import = 0x02,
  Function = 0x03,
  Memory = 0x05,
  Export = 0x07,
  Code = 0x0a,
  Data = 0x0b,
  Custom = 0xff, // custom section
}

impl From<u8> for SectionCode {
  fn from(code: u8) -> Option<Self> {
    match code {
      0x00 => Some(Self::Custom),
      0x01 => Some(Self::Type),
      _ => ,
    }
  }
}
