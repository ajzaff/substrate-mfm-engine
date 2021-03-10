pub mod mfm;

use crate::ast::{Arg, Instruction};
use crate::base;
use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use std::collections::HashMap;
use std::fmt;
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
  IOError,
  WrongMagicNumber,
  WrongMinorVersion,
  WrongMajorVersion,
  TagMismatch,
  BadMetadataOpCode,
  BadInstructionOpCode,
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
      Self::BadMetadataOpCode => "bad metadata opcode",
      Self::BadInstructionOpCode => "bad instruction opcode",
    };
    write!(f, "{}", s)
  }
}

pub fn load_from_bytes<'input>(bytes: &'input mut &[u8]) -> Result<Runtime<'input>, Error> {
  let mut r = Runtime::new();
  r.load_from_reader(bytes)?;
  Ok(r)
}

const MAGIC_NUMBER: u32 = 0x02030741;

struct Element<'input> {
  metadata: mfm::Metadata,
  code: Vec<Instruction<'input>>,
}

impl Element<'_> {
  fn new() -> Self {
    Self {
      metadata: mfm::Metadata::new(),
      code: Vec::new(),
    }
  }
}

pub struct Runtime<'input> {
  tag: Option<u64>,
  element_map: HashMap<String, Element<'input>>,
  ip: Option<u16>,
  symmetries_stack: Vec<base::Symmetries>,
  call_stack: Vec<u16>,
  op_stack: Vec<base::Const>,
}

impl<'input> Runtime<'input> {
  const MINOR_VERSION: u16 = 1;
  const MAJOR_VERSION: u16 = 0;

  pub fn new() -> Self {
    Self {
      tag: None,
      element_map: Self::new_element_map(),
      ip: None,
      symmetries_stack: Vec::new(),
      call_stack: Vec::new(),
      op_stack: Vec::new(),
    }
  }

  fn new_element_map() -> HashMap<String, Element<'input>> {
    let mut m = HashMap::new();
    let mut empty = Element::new();
    empty.metadata.name = "Empty".to_string();
    m.insert("Empty".to_string(), empty);
    m
  }

  fn read_const<R: ReadBytesExt>(r: &mut R) -> Result<base::Const, Error> {
    todo!()
  }

  fn read_string<R: ReadBytesExt>(r: &mut R) -> Result<String, Error> {
    todo!()
  }

  fn read_metadata<R: ReadBytesExt>(r: &mut R, elem: &mut Element) -> Result<(), Error> {
    let op = r.read_u8()?;
    match op {
      0 => elem.metadata.name = Self::read_string(r)?, // Name
      1 => elem.metadata.symbol = Self::read_string(r)?, // Symbol
      2 => elem.metadata.descs.push(Self::read_string(r)?), // Desc
      3 => elem.metadata.authors.push(Self::read_string(r)?), // Author
      4 => elem.metadata.licenses.push(Self::read_string(r)?), // License
      5 => elem.metadata.radius = r.read_u8()?,        // Radius
      6 => elem.metadata.bg_color = Self::read_string(r)?, // BgColor
      7 => elem.metadata.fg_color = Self::read_string(r)?, // FgColor
      8 => elem.metadata.symmetries = r.read_u8()?.into(), // Symmetries
      9 => {
        // Field
        let i = Self::read_string(r)?;
        let f: base::FieldSelector = r.read_u16::<BigEndian>()?.into();
        elem.metadata.field_map.insert(i, f);
      }
      10 => {
        // Parameter
        let i = Self::read_string(r)?;
        let c = Self::read_const(r)?;
        elem.metadata.parameter_map.insert(i, c);
      }
      _ => return Err(Error::BadMetadataOpCode),
    }
    Ok(())
  }

  fn read_instruction<R: ReadBytesExt>(r: &mut R, elem: &mut Element) -> Result<(), Error> {
    let op = r.read_u8()?;
    let instr = match op {
      0 => Instruction::Nop,       // Nop
      1 => Instruction::Exit,      // Exit
      2 => Instruction::SwapSites, // SwapSites
      3 => Instruction::SetSite,   // SetSite
      4 => Instruction::SetSite,   // SetField
      5 => Instruction::SetSiteField(Arg::Runtime(r.read_u16::<BigEndian>()?.into())), // SetSiteField
      6 => Instruction::GetSite,                                                       // GetSite
      7 => Instruction::GetField(Arg::Runtime(r.read_u16::<BigEndian>()?.into())),     // GetField
      8 => Instruction::GetSiteField(Arg::Runtime(r.read_u16::<BigEndian>()?.into())), // GetSiteField
      9 => Instruction::GetType(Arg::Runtime(r.read_u16::<BigEndian>()?)),             // GetType
      10 => Instruction::GetParameter(Arg::Runtime(Self::read_const(r)?)), // GetParamter
      11 => Instruction::Scan,                                             // Scan
      12 => Instruction::SaveSymmetries,                                   // SaveSymmetries
      13 => Instruction::UseSymmetries(r.read_u8()?.into()),               // UseSymmetries
      14 => Instruction::RestoreSymmetries,                                // RestoreSymmetries
      15 => Instruction::Push0,                                            // Push0
      16 => Instruction::Push1,                                            // Push1
      17 => Instruction::Push2,                                            // Push2
      18 => Instruction::Push3,                                            // Push3
      19 => Instruction::Push4,                                            // Push4
      20 => Instruction::Push5,                                            // Push5
      21 => Instruction::Push6,                                            // Push6
      22 => Instruction::Push7,                                            // Push7
      23 => Instruction::Push8,                                            // Push8
      24 => Instruction::Push9,                                            // Push9
      25 => Instruction::Push10,                                           // Push10
      26 => Instruction::Push11,                                           // Push11
      27 => Instruction::Push12,                                           // Push12
      28 => Instruction::Push13,                                           // Push13
      29 => Instruction::Push14,                                           // Push14
      30 => Instruction::Push15,                                           // Push15
      31 => Instruction::Push16,                                           // Push16
      32 => Instruction::Push17,                                           // Push17
      33 => Instruction::Push18,                                           // Push18
      34 => Instruction::Push19,                                           // Push19
      35 => Instruction::Push20,                                           // Push20
      36 => Instruction::Push21,                                           // Push21
      37 => Instruction::Push22,                                           // Push22
      38 => Instruction::Push23,                                           // Push23
      39 => Instruction::Push24,                                           // Push24
      40 => Instruction::Push25,                                           // Push25
      41 => Instruction::Push26,                                           // Push26
      42 => Instruction::Push27,                                           // Push27
      43 => Instruction::Push28,                                           // Push28
      44 => Instruction::Push29,                                           // Push29
      45 => Instruction::Push30,                                           // Push30
      46 => Instruction::Push31,                                           // Push31
      47 => Instruction::Push32,                                           // Push32
      48 => Instruction::Push33,                                           // Push33
      49 => Instruction::Push34,                                           // Push34
      50 => Instruction::Push35,                                           // Push35
      51 => Instruction::Push36,                                           // Push36
      52 => Instruction::Push37,                                           // Push37
      53 => Instruction::Push38,                                           // Push38
      54 => Instruction::Push39,                                           // Push39
      55 => Instruction::Push40,                                           // Push40
      56 => Instruction::Push(Self::read_const(r)?),                       // Push
      57 => Instruction::Pop,                                              // Pop
      58 => Instruction::Dup,                                              // Dup
      59 => Instruction::Over,                                             // Over
      60 => Instruction::Swap,                                             // Swap
      61 => Instruction::Rot,                                              // Rot
      62 => Instruction::Call(Arg::Runtime(r.read_u16::<BigEndian>()?)),   // Call
      63 => Instruction::Ret,                                              // Ret
      64 => Instruction::Checksum,                                         // Checksum
      65 => Instruction::Add,                                              // Add
      66 => Instruction::Sub,                                              // Sub
      67 => Instruction::Neg,                                              // Neg
      68 => Instruction::Mod,                                              // Mod
      69 => Instruction::Mul,                                              // Mul
      70 => Instruction::Div,                                              // Div
      71 => Instruction::Less,                                             // Less
      72 => Instruction::LessEqual,                                        // LessEqual
      73 => Instruction::Or,                                               // Or
      74 => Instruction::And,                                              // And
      75 => Instruction::Xor,                                              // Xor
      76 => Instruction::Equal,                                            // Equal
      77 => Instruction::BitCount,                                         // BitCount
      78 => Instruction::BitScanForward,                                   // BitScanForward
      79 => Instruction::BitScanReverse,                                   // BitScanReverse
      80 => Instruction::LShift,                                           // LShift
      81 => Instruction::RShift,                                           // RShift
      82 => Instruction::Jump(Arg::Runtime(r.read_u16::<BigEndian>()?)),   // Jump
      83 => Instruction::JumpRelativeOffset(Arg::Runtime(r.read_u16::<BigEndian>()?)), // JumpRelativeOffset
      84 => Instruction::JumpZero(Arg::Runtime(r.read_u16::<BigEndian>()?)),           // JumpZero
      85 => Instruction::JumpNonZero(Arg::Runtime(r.read_u16::<BigEndian>()?)), // JumpNonZero
      _ => return Err(Error::BadInstructionOpCode),
    };
    elem.code.push(instr);
    Ok(())
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
      Self::read_metadata(r, &mut elem)?;
    }

    for _ in 0..r.read_u16::<BigEndian>()? {
      Self::read_instruction(r, &mut elem)?;
    }

    Ok(())
  }

  pub fn reset(&mut self) {
    self.symmetries_stack.truncate(0);
    self.call_stack.truncate(0);
    self.op_stack.truncate(0);
    self.ip = None;
  }

  pub fn execute(&mut self, ew: &mfm::EventWindow) -> Result<(), Error> {
    self.reset();
    self.ip = Some(0);
    let elem = ew.get(0);
    todo!()
  }
}
