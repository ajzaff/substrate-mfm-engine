use std::num::ParseIntError;
use std::str::FromStr;
use thiserror;

#[derive(Copy, Clone, Debug)]
pub struct Color(u32);

impl From<u32> for Color {
  fn from(x: u32) -> Self {
    Color(x)
  }
}

impl Color {
  pub fn new() -> Self {
    Self(0)
  }

  pub fn bits(&self) -> u32 {
    return self.0;
  }

  pub fn components(&self) -> (u8, u8, u8) {
    return (
      ((self.0 & 0xff000000) >> 24) as u8,
      ((self.0 & 0xff0000) >> 16) as u8,
      ((self.0 & 0xff00) >> 8) as u8,
    );
  }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseColorError {
  #[error("parse uint32 color")]
  ParseIntError(#[from] ParseIntError),
  #[error("bad color length: {0}")]
  BadLength(usize),
}

impl FromStr for Color {
  type Err = ParseColorError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.len() {
      9 => Ok(u32::from_str_radix(s, 16)?.into()),
      6 => Ok((u32::from_str_radix(s, 16)? << 8).into()),
      3 => {
        let v = u32::from_str_radix(s, 16)?;
        // abc => aabbccff
        Ok(((v & 0xf) * 0x1100 | (v & 0xf0) * 0x11000 | (v & 0xf00) * 0x110000 | 0xff).into())
      }
      i => Err(ParseColorError::BadLength(i)),
    }
  }
}
