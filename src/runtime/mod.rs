pub mod mfm;

use crate::ast::{Arg, Instruction};
use crate::base::arith::Const;
use crate::base::{FieldSelector, Symmetries};
use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use log::trace;
use mfm::{EventWindow, Metadata};
use rand::RngCore;
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
  #[error("bad constant type: {0}")]
  BadConstantType(u8),
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
    empty.symbol = ".".to_owned();
    m.insert(0, empty);
    m
  }

  fn new_code_map() -> HashMap<u16, Vec<Instruction<'input>>> {
    let mut m = HashMap::new();
    m.insert(0, vec![]);
    m
  }

  fn read_const<R: ReadBytesExt>(r: &mut R) -> Result<Const, Error> {
    match r.read_u8()? {
      0 => {
        let mut n: u128 = r.read_u32::<BigEndian>()? as u128;
        n <<= 64;
        n |= r.read_u64::<BigEndian>()? as u128;
        Ok(n.into())
      }
      1 => {
        let mut n: i128 = r.read_i32::<BigEndian>()? as i128;
        n <<= 64;
        n |= r.read_i64::<BigEndian>()? as i128;
        Ok(n.into())
      }
      i => Err(Error::BadConstantType(i)),
    }
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
      4 => Instruction::SetField(Arg::Runtime(r.read_u16::<BigEndian>()?.into())), // SetField
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
      88 => Instruction::Rand,
      i => return Err(Error::BadInstructionOpCode(i)),
    };
    code.push(instr);
    Ok(())
  }

  pub fn load_from_reader<R: ReadBytesExt>(&mut self, r: &mut R) -> Result<Const, Error> {
    {
      let v = r.read_u32::<BigEndian>()?;
      if v != MAGIC_NUMBER {
        return Err(Error::BadMagicNumber(v));
      }
    }
    {
      let v = r.read_u16::<BigEndian>()?;
      if v != Self::MINOR_VERSION {
        return Err(Error::BadMinorVersion(v));
      }
    }
    {
      let v = r.read_u16::<BigEndian>()?;
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

    trace!("{:?}", elem);

    let mut code = Vec::new();

    for _ in 0..r.read_u16::<BigEndian>()? {
      Self::read_instruction(r, &mut code)?;
    }

    trace!("{:?}", code);

    self.type_map.insert(type_num, elem);
    self.code_map.insert(type_num, code);
    Ok(((type_num as u128) << 80).into())
  }

  pub fn execute<T: mfm::EventWindow + mfm::Rand>(
    ew: &mut T,
    code_map: &HashMap<u16, Vec<Instruction<'input>>>,
  ) -> Result<(), Error> {
    let my_atom = ew.get(0);
    let my_type: u16 = my_atom.apply(&FieldSelector::TYPE).into();
    let code = code_map
      .get(&my_type)
      .ok_or(Error::UnknownElement(my_type))?;
    let mut cursor = Cursor::new();
    loop {
      if cursor.ip >= code.len() {
        // Handle implicit Ret:
        while let Some(mut ip) = cursor.call_stack.pop() {
          if ip == u16::MAX as usize {
            continue;
          }
          ip += 1;
          if ip >= code.len() {
            continue;
          }
          cursor.ip = ip;
          break;
        }
        if cursor.ip >= code.len() {
          break;
        }
      }
      let op = code[cursor.ip];
      trace!("{:?} => {:?}", cursor, op);
      match op {
        Instruction::Nop => {}
        Instruction::Exit => break,
        Instruction::SwapSites => {
          let j: usize = cursor.op_stack.pop().unwrap().into();
          let i: usize = cursor.op_stack.pop().unwrap().into();
          ew.swap(i, j);
        }
        Instruction::SetSite => {
          let c = cursor.op_stack.pop().unwrap();
          let i: usize = cursor.op_stack.pop().unwrap().into();
          *ew.get_mut(i).unwrap() = c;
        }
        Instruction::SetField(f) => {
          let mut b = cursor.op_stack.pop().unwrap();
          let a = cursor.op_stack.pop().unwrap();
          let fi = f.runtime();
          b.truncate(fi.length);
          cursor.op_stack.push(a | (b << fi.offset));
        }
        Instruction::SetSiteField(f) => {
          let mut c = cursor.op_stack.pop().unwrap();
          let i: usize = cursor.op_stack.pop().unwrap().into();
          let fi = f.runtime();
          c.truncate(fi.length);
          if let Some(a) = ew.get_mut(i) {
            *a = *a | (c << fi.offset);
          }
        }
        Instruction::GetSite => {
          let v = ew.get(cursor.op_stack.pop().unwrap().into());
          cursor.op_stack.push(v);
        }
        Instruction::GetField(f) => {
          let a = cursor.op_stack.pop().unwrap();
          cursor.op_stack.push(a.apply(f.runtime()));
        }
        Instruction::GetSiteField(f) => {
          let i: usize = cursor.op_stack.pop().unwrap().into();
          cursor.op_stack.push(ew.get(i).apply(f.runtime()));
        }
        Instruction::GetType(x) => cursor.op_stack.push((*x.runtime()).into()),
        Instruction::GetParameter(c) => {
          cursor.op_stack.push(*c.runtime());
        }
        Instruction::Scan => todo!(),
        Instruction::SaveSymmetries => cursor.symmetries_stack.push(cursor.symmetries),
        Instruction::UseSymmetries(x) => cursor.symmetries = x,
        Instruction::RestoreSymmetries => {
          cursor.symmetries = cursor.symmetries_stack.pop().unwrap()
        }
        Instruction::Push0 => cursor.op_stack.push(0u8.into()),
        Instruction::Push1 => cursor.op_stack.push(1u8.into()),
        Instruction::Push2 => cursor.op_stack.push(2u8.into()),
        Instruction::Push3 => cursor.op_stack.push(3u8.into()),
        Instruction::Push4 => cursor.op_stack.push(4u8.into()),
        Instruction::Push5 => cursor.op_stack.push(5u8.into()),
        Instruction::Push6 => cursor.op_stack.push(6u8.into()),
        Instruction::Push7 => cursor.op_stack.push(7u8.into()),
        Instruction::Push8 => cursor.op_stack.push(8u8.into()),
        Instruction::Push9 => cursor.op_stack.push(9u8.into()),
        Instruction::Push10 => cursor.op_stack.push(10u8.into()),
        Instruction::Push11 => cursor.op_stack.push(11u8.into()),
        Instruction::Push12 => cursor.op_stack.push(12u8.into()),
        Instruction::Push13 => cursor.op_stack.push(13u8.into()),
        Instruction::Push14 => cursor.op_stack.push(14u8.into()),
        Instruction::Push15 => cursor.op_stack.push(15u8.into()),
        Instruction::Push16 => cursor.op_stack.push(16u8.into()),
        Instruction::Push17 => cursor.op_stack.push(17u8.into()),
        Instruction::Push18 => cursor.op_stack.push(18u8.into()),
        Instruction::Push19 => cursor.op_stack.push(19u8.into()),
        Instruction::Push20 => cursor.op_stack.push(20u8.into()),
        Instruction::Push21 => cursor.op_stack.push(21u8.into()),
        Instruction::Push22 => cursor.op_stack.push(22u8.into()),
        Instruction::Push23 => cursor.op_stack.push(23u8.into()),
        Instruction::Push24 => cursor.op_stack.push(24u8.into()),
        Instruction::Push25 => cursor.op_stack.push(25u8.into()),
        Instruction::Push26 => cursor.op_stack.push(26u8.into()),
        Instruction::Push27 => cursor.op_stack.push(27u8.into()),
        Instruction::Push28 => cursor.op_stack.push(28u8.into()),
        Instruction::Push29 => cursor.op_stack.push(29u8.into()),
        Instruction::Push30 => cursor.op_stack.push(30u8.into()),
        Instruction::Push31 => cursor.op_stack.push(31u8.into()),
        Instruction::Push32 => cursor.op_stack.push(32u8.into()),
        Instruction::Push33 => cursor.op_stack.push(33u8.into()),
        Instruction::Push34 => cursor.op_stack.push(34u8.into()),
        Instruction::Push35 => cursor.op_stack.push(35u8.into()),
        Instruction::Push36 => cursor.op_stack.push(36u8.into()),
        Instruction::Push37 => cursor.op_stack.push(37u8.into()),
        Instruction::Push38 => cursor.op_stack.push(38u8.into()),
        Instruction::Push39 => cursor.op_stack.push(39u8.into()),
        Instruction::Push40 => cursor.op_stack.push(40u8.into()),
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
          continue;
        }
        Instruction::Ret => {
          cursor.ip = cursor.call_stack.pop().unwrap();
          if cursor.ip == u16::MAX as usize {
            break;
          }
          cursor.ip += 1;
          continue;
        }
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
          cursor.op_stack.push(a.count_ones().into());
        }
        Instruction::BitScanForward => {
          let a = cursor.op_stack.pop().unwrap();
          cursor.op_stack.push(a.bitscanforward().into());
        }
        Instruction::BitScanReverse => {
          let a = cursor.op_stack.pop().unwrap();
          cursor.op_stack.push(a.bitscanreverse().into());
        }
        Instruction::LShift => {
          let b = cursor.op_stack.pop().unwrap();
          let a = cursor.op_stack.pop().unwrap();
          cursor.op_stack.push(a >> b.into()) // TODO handle b overflow
        }
        Instruction::RShift => {
          let b = cursor.op_stack.pop().unwrap();
          let a = cursor.op_stack.pop().unwrap();
          cursor.op_stack.push(a << b.into()) // TODO handle b overflow
        }
        Instruction::Jump(x) => {
          cursor.ip = *x.runtime() as usize;
          continue;
        }
        Instruction::JumpRelativeOffset => {
          let a = cursor.op_stack.pop().unwrap();
          match a {
            Const::Unsigned(x) => cursor.ip += x as usize,
            Const::Signed(_) => {
              let amount: usize = a.abs_saturating().into();
              cursor.ip -= amount;
            }
          }
          break;
        }
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
          let c: u32 = cursor.op_stack.pop().unwrap().into();
          *ew.get_paint_mut() = c.into();
        }
        Instruction::GetPaint => {
          cursor.op_stack.push(ew.get_paint().bits().into());
        }
        Instruction::Rand => {
          cursor.op_stack.push(ew.rand());
        }
      }
      cursor.ip += 1;
    }
    Ok(())
  }
}
