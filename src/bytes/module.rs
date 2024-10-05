use crate::diagnostics::{ResultWithDiagnostics, RuntimeError};
use nom::{
  bytes::complete::{tag, take},
  number::complete::{le_u32, le_u8},
  sequence::pair,
  IResult,
};
use nom_leb128::leb128_u32;
use num_traits::FromPrimitive as _;

use super::{section::SectionCode, types::FuncType};

type TypeSection = Vec<FuncType>;

fn decode_type_section(_input: &[u8]) -> IResult<&[u8], TypeSection> {
  let func_types = vec![FuncType::default()];
  Ok((&[], func_types))
}
#[derive(Debug, PartialEq, Eq)]
pub struct Module {
  pub magic: String,
  pub version: u32,
  pub type_section: Option<TypeSection>,
}
// https://webassembly.github.io/spec/core/binary/modules.html#binary-module
impl Default for Module {
  fn default() -> Self {
    Self { magic: "\0asm".to_string(), version: 1, type_section: None }
  }
}

impl Module {
  pub fn new(input: &[u8]) -> ResultWithDiagnostics<Module> {
    let (_, module) = Module::decode(input).map_err(|error| {
      let cause = format!("{}", error);
      let runtime_error = RuntimeError::FailedToDecodeModule { range: None, cause };
      return runtime_error;
    })?;
    Ok(module)
  }

  fn decode(input: &[u8]) -> IResult<&[u8], Module> {
    let (input, _) = tag(b"\0asm")(input)?;
    let (input, version) = le_u32(input)?;
    let module = Module { magic: "\0asm".into(), version, ..Default::default() };
    Ok((input, module))
  }

  fn decode_section_header(&self, input: &[u8]) -> IResult<&[u8], (SectionCode, u32)> {
    let (input, (code, size)) = pair(le_u8, leb128_u32)(input)?;
    let section_code = SectionCode::from_u8(code);

    if section_code.is_none() {
      return Err(nom::Err::Error(nom::error::make_error(
        input,
        nom::error::ErrorKind::Tag,
      )));
    }
    let mut remaining = input;

    while !remaining.is_empty() {
      match self.decode_section_header(remaining) {
        Ok((input, (code, size))) => {
          let (rest, section_contents) = take(size)(input)?;

          match code {
            SectionCode::Type => {
              let (rest, types) = decode_type_section(section_contents)?;
            }
            _ => todo!(),
          };
          remaining = rest;
        }
        Err(err) => return Err(err),
      }
    }
    Ok((input, section))
  }
}
