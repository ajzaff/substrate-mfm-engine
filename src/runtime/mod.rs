pub mod mfm;

use crate::ast;
use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use std::collections::HashMap;
use std::fmt;
use std::io;

pub enum Error {
  IOError,
  WrongMagicNumber,
  WrongMinorVersion,
  WrongMajorVersion,
  TagMismatch,
}

impl From<io::Error> for Error {
  fn from(_: io::Error) -> Self {
    Self::IOError
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let s = match self {
      Self::IOError => "IO error",
      Self::WrongMagicNumber => "wrong magic number",
      Self::WrongMinorVersion => "wrong minor version",
      Self::WrongMajorVersion => "wrong major version",
      Self::TagMismatch => "tag mismatch",
    };
    write!(f, "{}", s)
  }
}

pub fn load_from_bytes(bytes: &mut &[u8]) -> Result<Runtime, Error> {
  let mut r = Runtime::new();
  r.load_from_reader(bytes)?;
  Ok(r)
}

const MAGIC_NUMBER: u32 = 0x02030741;

struct Element {
  metadata: Vec<ast::Metadata>,
  code: Vec<ast::Node>,
}

impl Element {
  fn new() -> Self {
    Self {
      metadata: vec![],
      code: vec![],
    }
  }
}

pub struct Runtime {
  tag: Option<u64>,
  elements: HashMap<String, Element>,
}

impl Runtime {
  const MINOR_VERSION: u16 = 1;
  const MAJOR_VERSION: u16 = 0;

  pub fn new() -> Self {
    Self {
      tag: None,
      elements: Self::new_elements_map(),
    }
  }

  fn new_elements_map() -> HashMap<String, Element> {
    let mut m = HashMap::new();
    let mut empty = Element::new();
    empty
      .metadata
      .push(ast::Metadata::Name("Empty".to_string()));
    m.insert("Empty".to_string(), empty);
    m
  }

  fn read_metadata<R: ReadBytesExt>(&mut self, r: &mut R, elem: &mut Element) -> Result<(), Error> {
    todo!()
  }

  fn read_instruction<R: ReadBytesExt>(
    &mut self,
    r: &mut R,
    elem: &mut Element,
  ) -> Result<(), Error> {
    todo!()
  }

  pub fn load_from_reader<R: ReadBytesExt>(&mut self, r: &mut R) -> Result<(), Error> {
    if r.read_u32::<BigEndian>()? != MAGIC_NUMBER {
      return Err(Error::WrongMagicNumber);
    }
    if r.read_u16::<BigEndian>()? != Self::MINOR_VERSION {
      return Err(Error::WrongMinorVersion);
    }
    if r.read_u16::<BigEndian>()? != Self::MAJOR_VERSION {
      return Err(Error::WrongMajorVersion);
    }
    let tag = r.read_u64::<BigEndian>()?;
    if let Some(t) = self.tag {
      if tag != t {
        return Err(Error::TagMismatch);
      }
    } else {
      self.tag = Some(tag);
    }

    let mut elem = Element::new();

    for _ in 0..r.read_u8()? {
      self.read_metadata(r, &mut elem)?;
    }

    for _ in 0..r.read_u16::<BigEndian>()? {
      self.read_instruction(r, &mut elem)?;
    }

    Ok(())
  }

  pub fn execute(&mut self, ew: &mfm::EventWindow) -> Result<(), Error> {
    todo!()
  }
}
