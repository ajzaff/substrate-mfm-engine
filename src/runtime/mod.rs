pub mod mfm;

use crate::ast::{Arg, Instruction};
use crate::base::arith::Const;
use crate::base::{FieldSelector, Symmetries};
use crate::runtime::mfm::Metadata;
use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use std::collections::HashMap;
use std::io;
use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("IO error")]
  IOError(#[from] io::Error),
  #[error("UTF-8 error")]
  FromUtf8Error(#[from] std::string::FromUtf8Error),
  #[error("bad magic number: {0}")]
  BadMagicNumber(u32),
  #[error("wrong minor version")]
  BadMinorVersion(u16),
  #[error("wrong major version")]
  BadMajorVersion(u16),
  #[error("build tag mismatch: {got:?} but expected: {want:?}")]
  BuildTagMismatch { want: String, got: String },
  #[error("bad metadata op code: {0}")]
  BadMetadataOpCode(u8),
  #[error("bad instruction op code: {0}")]
  BadInstructionOpCode(u8),
  #[error("no element")]
  NoElement,
  #[error("running unknown element: {0}")]
  UnknownElement(u16),
  #[error("stack underflow")]
  StackUnderflow, // TODO: add context
}

pub fn load_from_bytes<'input>(bytes: &'input mut &[u8]) -> Result<Runtime<'input>, Error> {
  let mut r = Runtime::new();
  r.load_from_reader(bytes)?;
  Ok(r)
}

const MAGIC_NUMBER: u32 = 0x02030741;

#[derive(Debug)]
struct Cursor {
  ip: usize,
  symmetries: Symmetries,
  symmetries_stack: Vec<Symmetries>,
  call_stack: Vec<usize>,
  op_stack: Vec<Const>,
}

impl Cursor {
  fn new() -> Self {
    Self {
      ip: 0,
      symmetries: Symmetries::R000L,
      symmetries_stack: Vec::new(),
      call_stack: Vec::new(),
      op_stack: Vec::new(),
    }
  }

  fn reset(&mut self) {
    self.ip = 0;
    self.symmetries_stack.clear();
    self.call_stack.clear();
    self.op_stack.clear();
  }
}

pub struct Runtime<'input> {
  tag: Option<String>,
  pub code_map: HashMap<u16, Vec<Instruction<'input>>>,
  pub type_map: HashMap<u16, Metadata>,
}

impl<'input> Runtime<'input> {
  const MINOR_VERSION: u16 = 1;
  const MAJOR_VERSION: u16 = 0;

  pub fn new() -> Self {
    Self {
      tag: None,
      type_map: Self::new_type_map(),
      code_map: Self::new_code_map(),
    }
  }

  fn new_type_map() -> HashMap<u16, Metadata> {
    let mut m = HashMap::new();
    let mut empty = Metadata::new();
    empty.name = "Empty".to_owned();
    empty.fg_color = 0u32.into();
    m.insert(0, empty);
    m
  }

  fn new_code_map() -> HashMap<u16, Vec<Instruction<'input>>> {
    let mut m = HashMap::new();
    m.insert(0, vec![]);
    m
  }

  fn read_const<R: ReadBytesExt>(r: &mut R) -> Result<Const, Error> {
    todo!()
  }

  fn read_string<R: ReadBytesExt>(r: &mut R) -> Result<String, Error> {
    let n = r.read_u8()?;
    let mut b = vec![0u8; n as usize];
    r.read_exact(&mut b)?;
    Ok(String::from_utf8(b)?)
  }

  fn read_metadata<R: ReadBytesExt>(r: &mut R, elem: &mut Metadata) -> Result<(), Error> {
    let op = r.read_u8()?;
    match op {
      0 => elem.name = Self::read_string(r)?,         // Name
      1 => elem.symbol = Self::read_string(r)?,       // Symbol
      2 => elem.descs.push(Self::read_string(r)?),    // Desc
      3 => elem.authors.push(Self::read_string(r)?),  // Author
      4 => elem.licenses.push(Self::read_string(r)?), // License
      5 => elem.radius = r.read_u8()?,                // Radius
      6 => elem.bg_color = r.read_u32::<BigEndian>()?.into(), // BgColor
      7 => elem.fg_color = r.read_u32::<BigEndian>()?.into(), // FgColor
      8 => elem.symmetries = r.read_u8()?.into(),     // Symmetries
      9 => {
        // Field
        let i = Self::read_string(r)?;
        let f: FieldSelector = r.read_u16::<BigEndian>()?.into();
        elem.field_map.insert(i, f);
      }
      10 => {
        // Parameter
        let i = Self::read_string(r)?;
        let c = Self::read_const(r)?;
        elem.parameter_map.insert(i, c);
      }
      i => return Err(Error::BadMetadataOpCode(i)),
    }
    Ok(())
  }

  fn read_instruction<R: ReadBytesExt>(
    r: &mut R,
    code: &mut Vec<Instruction<'input>>,
  ) -> Result<(), Error> {
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
      83 => Instruction::JumpRelativeOffset,                               // JumpRelativeOffset
      84 => Instruction::JumpZero(Arg::Runtime(r.read_u16::<BigEndian>()?)), // JumpZero
      85 => Instruction::JumpNonZero(Arg::Runtime(r.read_u16::<BigEndian>()?)), // JumpNonZero
      86 => Instruction::SetPaint,
      87 => Instruction::GetPaint,
      i => return Err(Error::BadInstructionOpCode(i)),
    };
    code.push(instr);
    Ok(())
  }

  pub fn load_from_reader<R: ReadBytesExt>(&mut self, r: &mut R) -> Result<Const, Error> {
    if let v = r.read_u32::<BigEndian>()? {
      if v != MAGIC_NUMBER {
        return Err(Error::BadMagicNumber(v));
      }
    }
    if let v = r.read_u16::<BigEndian>()? {
      if v != Self::MINOR_VERSION {
        return Err(Error::BadMinorVersion(v));
      }
    }
    if let v = r.read_u16::<BigEndian>()? {
      if v != Self::MAJOR_VERSION {
        return Err(Error::BadMajorVersion(v));
      }
    }
    let tag = Self::read_string(r)?;
    if let Some(self_tag) = self.tag.as_ref() {
      if self_tag != &tag {
        return Err(Error::BuildTagMismatch {
          want: self_tag.to_owned(),
          got: tag.to_owned(),
        });
      }
    } else {
      self.tag = Some(tag);
    }

    let type_num = r.read_u16::<BigEndian>()?;
    let mut elem = Metadata::new();

    for _ in 0..r.read_u8()? {
      Self::read_metadata(r, &mut elem)?;
    }

    let mut code = Vec::new();

    r.read_u16::<BigEndian>()?; // Code index stub

    for _ in 0..r.read_u16::<BigEndian>()? {
      Self::read_instruction(r, &mut code)?;
    }

    self.type_map.insert(type_num, elem);
    self.code_map.insert(type_num, code);
    Ok(((type_num as u128) << 80).into())
  }

  pub fn execute(
    ew: &mut mfm::EventWindow,
    code_map: HashMap<u16, Vec<Instruction<'input>>>,
  ) -> Result<(), Error> {
    let my_atom = ew.get(0).ok_or(Error::NoElement)?;
    let my_type = my_atom.apply(FieldSelector::TYPE).as_u128() as u16;
    let code = code_map
      .get(&my_type)
      .ok_or(Error::UnknownElement(my_type))?;
    let mut cursor = Cursor::new();
    while (cursor.ip as usize) < code.len() {
      match code[cursor.ip as usize] {
        Instruction::Nop => {}
        Instruction::Exit => break,
        Instruction::SwapSites => todo!(),
        Instruction::SetSite => {
          let c = cursor.op_stack.pop().unwrap();
          let i = cursor.op_stack.pop().unwrap().as_u128() as usize;
          *ew.get_mut(i).unwrap() = c;
        }
        Instruction::SetField(_) => todo!(),
        Instruction::SetSiteField(_) => todo!(),
        Instruction::GetSite => {
          let v = *ew
            .get(cursor.op_stack.pop().unwrap().as_u128() as usize)
            .unwrap();
          cursor.op_stack.push(v);
        }
        Instruction::GetField(_) => todo!(),
        Instruction::GetSiteField(_) => todo!(),
        Instruction::GetType(_) => todo!(),
        Instruction::GetParameter(_) => todo!(),
        Instruction::Scan => todo!(),
        Instruction::SaveSymmetries => cursor.symmetries_stack.push(cursor.symmetries),
        Instruction::UseSymmetries(x) => cursor.symmetries = x,
        Instruction::RestoreSymmetries => {
          cursor.symmetries = cursor.symmetries_stack.pop().unwrap()
        }
        Instruction::Push0 => cursor.op_stack.push(0.into()),
        Instruction::Push1 => cursor.op_stack.push(1.into()),
        Instruction::Push2 => cursor.op_stack.push(2.into()),
        Instruction::Push3 => cursor.op_stack.push(3.into()),
        Instruction::Push4 => cursor.op_stack.push(4.into()),
        Instruction::Push5 => cursor.op_stack.push(5.into()),
        Instruction::Push6 => cursor.op_stack.push(6.into()),
        Instruction::Push7 => cursor.op_stack.push(7.into()),
        Instruction::Push8 => cursor.op_stack.push(8.into()),
        Instruction::Push9 => cursor.op_stack.push(9.into()),
        Instruction::Push10 => cursor.op_stack.push(10.into()),
        Instruction::Push11 => cursor.op_stack.push(11.into()),
        Instruction::Push12 => cursor.op_stack.push(12.into()),
        Instruction::Push13 => cursor.op_stack.push(13.into()),
        Instruction::Push14 => cursor.op_stack.push(14.into()),
        Instruction::Push15 => cursor.op_stack.push(15.into()),
        Instruction::Push16 => cursor.op_stack.push(16.into()),
        Instruction::Push17 => cursor.op_stack.push(17.into()),
        Instruction::Push18 => cursor.op_stack.push(18.into()),
        Instruction::Push19 => cursor.op_stack.push(19.into()),
        Instruction::Push20 => cursor.op_stack.push(20.into()),
        Instruction::Push21 => cursor.op_stack.push(21.into()),
        Instruction::Push22 => cursor.op_stack.push(22.into()),
        Instruction::Push23 => cursor.op_stack.push(23.into()),
        Instruction::Push24 => cursor.op_stack.push(24.into()),
        Instruction::Push25 => cursor.op_stack.push(25.into()),
        Instruction::Push26 => cursor.op_stack.push(26.into()),
        Instruction::Push27 => cursor.op_stack.push(27.into()),
        Instruction::Push28 => cursor.op_stack.push(28.into()),
        Instruction::Push29 => cursor.op_stack.push(29.into()),
        Instruction::Push30 => cursor.op_stack.push(30.into()),
        Instruction::Push31 => cursor.op_stack.push(31.into()),
        Instruction::Push32 => cursor.op_stack.push(32.into()),
        Instruction::Push33 => cursor.op_stack.push(33.into()),
        Instruction::Push34 => cursor.op_stack.push(34.into()),
        Instruction::Push35 => cursor.op_stack.push(35.into()),
        Instruction::Push36 => cursor.op_stack.push(36.into()),
        Instruction::Push37 => cursor.op_stack.push(37.into()),
        Instruction::Push38 => cursor.op_stack.push(38.into()),
        Instruction::Push39 => cursor.op_stack.push(39.into()),
        Instruction::Push40 => cursor.op_stack.push(40.into()),
        Instruction::Push(c) => cursor.op_stack.push(c),
        Instruction::Pop => {
          cursor.op_stack.pop().expect("stack underflow");
        }
        Instruction::Dup => {
          let t = cursor.op_stack.pop().unwrap();
          cursor.op_stack.push(t);
          cursor.op_stack.push(t);
        }
        Instruction::Over => {
          let ignore = cursor.op_stack.pop().unwrap();
          let t = cursor.op_stack.pop().unwrap();
          cursor.op_stack.push(t);
          cursor.op_stack.push(ignore);
          cursor.op_stack.push(t);
        }
        Instruction::Swap => {
          let n = cursor.op_stack.len();
          cursor.op_stack.swap(n - 2, n - 1);
        }
        Instruction::Rot => {
          let n = cursor.op_stack.len();
          cursor.op_stack.swap(n - 2, n - 1);
          cursor.op_stack.swap(n - 3, n - 2);
        }
        Instruction::Call(x) => {
          cursor.call_stack.push(cursor.ip);
          cursor.ip = *x.runtime() as usize;
        }
        Instruction::Ret => cursor.ip = cursor.call_stack.pop().unwrap(),
        Instruction::Checksum => todo!(),
        Instruction::Add => {
          let b = cursor.op_stack.pop().unwrap();
          let a = cursor.op_stack.pop().unwrap();
          cursor.op_stack.push(a + b);
        }
        Instruction::Sub => {
          let b = cursor.op_stack.pop().unwrap();
          let a = cursor.op_stack.pop().unwrap();
          cursor.op_stack.push(a - b);
        }
        Instruction::Neg => {
          let a = cursor.op_stack.pop().unwrap();
          cursor.op_stack.push(-a);
        }
        Instruction::Mod => {
          let b = cursor.op_stack.pop().unwrap();
          let a = cursor.op_stack.pop().unwrap();
          cursor.op_stack.push(a % b);
        }
        Instruction::Mul => {
          let b = cursor.op_stack.pop().unwrap();
          let a = cursor.op_stack.pop().unwrap();
          cursor.op_stack.push(a * b);
        }
        Instruction::Div => {
          let b = cursor.op_stack.pop().unwrap();
          let a = cursor.op_stack.pop().unwrap();
          cursor.op_stack.push(a / b);
        }
        Instruction::Less => {
          let b = cursor.op_stack.pop().unwrap();
          let a = cursor.op_stack.pop().unwrap();
          cursor.op_stack.push(if a < b { 1 } else { 0 }.into());
        }
        Instruction::LessEqual => {
          let b = cursor.op_stack.pop().unwrap();
          let a = cursor.op_stack.pop().unwrap();
          cursor.op_stack.push(if a <= b { 1 } else { 0 }.into());
        }
        Instruction::Or => {
          let b = cursor.op_stack.pop().unwrap();
          let a = cursor.op_stack.pop().unwrap();
          cursor.op_stack.push(a | b);
        }
        Instruction::And => {
          let b = cursor.op_stack.pop().unwrap();
          let a = cursor.op_stack.pop().unwrap();
          cursor.op_stack.push(a & b);
        }
        Instruction::Xor => {
          let b = cursor.op_stack.pop().unwrap();
          let a = cursor.op_stack.pop().unwrap();
          cursor.op_stack.push(a ^ b);
        }
        Instruction::Equal => {
          let b = cursor.op_stack.pop().unwrap();
          let a = cursor.op_stack.pop().unwrap();
          cursor.op_stack.push(if a == b { 1 } else { 0 }.into())
        }
        Instruction::BitCount => {
          let a = cursor.op_stack.pop().unwrap();
          cursor.op_stack.push(a.as_u128().count_ones().into());
        }
        Instruction::BitScanForward => todo!(),
        Instruction::BitScanReverse => todo!(),
        Instruction::LShift => todo!(),
        Instruction::RShift => todo!(),
        Instruction::Jump(x) => {
          cursor.ip = *x.runtime() as usize;
          continue;
        }
        Instruction::JumpRelativeOffset => todo!(),
        Instruction::JumpZero(x) => {
          if cursor.op_stack.pop().unwrap().is_zero() {
            cursor.ip = *x.runtime() as usize;
            continue;
          }
        }
        Instruction::JumpNonZero(x) => {
          if !cursor.op_stack.pop().unwrap().is_zero() {
            cursor.ip = *x.runtime() as usize;
            continue;
          }
        }
        Instruction::SetPaint => {
          let i = cursor.op_stack.pop().unwrap().as_u128() as usize;
          let v = cursor.op_stack.pop().unwrap().as_u128() as u32;
          *ew.get_paint_mut(i).unwrap() = v.into();
        }
        Instruction::GetPaint => {
          let i = cursor.op_stack.pop().unwrap().as_u128() as usize;
          let v = ew.get_paint(i).unwrap();
          cursor.op_stack.push(v.bits().into());
        }
      }
      cursor.ip += 1;
    }
    Ok(())
  }
}
