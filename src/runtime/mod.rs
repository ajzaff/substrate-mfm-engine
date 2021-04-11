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

pub trait RuntimeImpl {
  fn pop() -> Option<Const>;
  fn pop_site() -> Option<usize>;
  fn push(x: Const);
  fn save_symmetries();
  fn use_symmetries(x: Symmetries);
  fn restore_symmetries();
  fn get(i: usize) -> Option<Const>;
}

const MAGIC_NUMBER: u32 = 0x02030741;

#[derive(Debug)]
pub struct Cursor {
  ip: usize,
  symmetry: Symmetries,
  symmetries_stack: Vec<Symmetries>,
  call_stack: Vec<usize>,
  op_stack: Vec<Const>,
}

impl Cursor {
  pub fn new() -> Self {
    Self::with_symmetry(Symmetries::R000L)
  }

  pub fn with_symmetry(s: Symmetries) -> Self {
    Self {
      ip: 0,
      symmetry: s,
      symmetries_stack: Vec::new(),
      call_stack: Vec::new(),
      op_stack: Vec::new(),
    }
  }

  pub fn reset(&mut self, s: Symmetries) {
    self.ip = 0;
    self.symmetry = s;
    self.symmetries_stack.clear();
    self.call_stack.clear();
    self.op_stack.clear();
  }

  fn pop(&mut self) -> Const {
    self.op_stack.pop().unwrap()
  }

  fn pop_site(&mut self) -> usize {
    let i: u8 = self.pop().into();
    mfm::map_site(i, self.symmetry) as usize
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
      9 => Instruction::GetSignedField(Arg::Runtime(r.read_u16::<BigEndian>()?.into())), // GetSignedField
      10 => Instruction::GetSignedSiteField(Arg::Runtime(r.read_u16::<BigEndian>()?.into())), // GetSignedSiteField
      11 => Instruction::GetType(Arg::Runtime(r.read_u16::<BigEndian>()?)), // GetType
      12 => Instruction::GetParameter(Arg::Runtime(Self::read_const(r)?)),  // GetParamter
      13 => Instruction::Scan,                                              // Scan
      14 => Instruction::SaveSymmetries,                                    // SaveSymmetries
      15 => Instruction::UseSymmetries(r.read_u8()?.into()),                // UseSymmetries
      16 => Instruction::RestoreSymmetries,                                 // RestoreSymmetries
      17 => Instruction::Push0,                                             // Push0
      18 => Instruction::Push1,                                             // Push1
      19 => Instruction::Push2,                                             // Push2
      20 => Instruction::Push3,                                             // Push3
      21 => Instruction::Push4,                                             // Push4
      22 => Instruction::Push5,                                             // Push5
      23 => Instruction::Push6,                                             // Push6
      24 => Instruction::Push7,                                             // Push7
      25 => Instruction::Push8,                                             // Push8
      26 => Instruction::Push9,                                             // Push9
      27 => Instruction::Push10,                                            // Push10
      28 => Instruction::Push11,                                            // Push11
      29 => Instruction::Push12,                                            // Push12
      30 => Instruction::Push13,                                            // Push13
      31 => Instruction::Push14,                                            // Push14
      32 => Instruction::Push15,                                            // Push15
      33 => Instruction::Push16,                                            // Push16
      34 => Instruction::Push17,                                            // Push17
      35 => Instruction::Push18,                                            // Push18
      36 => Instruction::Push19,                                            // Push19
      37 => Instruction::Push20,                                            // Push20
      38 => Instruction::Push21,                                            // Push21
      39 => Instruction::Push22,                                            // Push22
      40 => Instruction::Push23,                                            // Push23
      41 => Instruction::Push24,                                            // Push24
      42 => Instruction::Push25,                                            // Push25
      43 => Instruction::Push26,                                            // Push26
      44 => Instruction::Push27,                                            // Push27
      45 => Instruction::Push28,                                            // Push28
      46 => Instruction::Push29,                                            // Push29
      47 => Instruction::Push30,                                            // Push30
      48 => Instruction::Push31,                                            // Push31
      49 => Instruction::Push32,                                            // Push32
      50 => Instruction::Push33,                                            // Push33
      51 => Instruction::Push34,                                            // Push34
      52 => Instruction::Push35,                                            // Push35
      53 => Instruction::Push36,                                            // Push36
      54 => Instruction::Push37,                                            // Push37
      55 => Instruction::Push38,                                            // Push38
      56 => Instruction::Push39,                                            // Push39
      57 => Instruction::Push40,                                            // Push40
      58 => Instruction::Push(Self::read_const(r)?),                        // Push
      59 => Instruction::Pop,                                               // Pop
      60 => Instruction::Dup,                                               // Dup
      61 => Instruction::Over,                                              // Over
      62 => Instruction::Swap,                                              // Swap
      63 => Instruction::Rot,                                               // Rot
      64 => Instruction::Call(Arg::Runtime(r.read_u16::<BigEndian>()?)),    // Call
      65 => Instruction::Ret,                                               // Ret
      66 => Instruction::Checksum,                                          // Checksum
      67 => Instruction::Add,                                               // Add
      68 => Instruction::Sub,                                               // Sub
      69 => Instruction::Neg,                                               // Neg
      70 => Instruction::Mod,                                               // Mod
      71 => Instruction::Mul,                                               // Mul
      72 => Instruction::Div,                                               // Div
      73 => Instruction::Less,                                              // Less
      74 => Instruction::LessEqual,                                         // LessEqual
      75 => Instruction::Or,                                                // Or
      76 => Instruction::And,                                               // And
      77 => Instruction::Xor,                                               // Xor
      78 => Instruction::Equal,                                             // Equal
      79 => Instruction::BitCount,                                          // BitCount
      80 => Instruction::BitScanForward,                                    // BitScanForward
      81 => Instruction::BitScanReverse,                                    // BitScanReverse
      82 => Instruction::LShift,                                            // LShift
      83 => Instruction::RShift,                                            // RShift
      84 => Instruction::Jump(Arg::Runtime(r.read_u16::<BigEndian>()?)),    // Jump
      85 => Instruction::JumpRelativeOffset,                                // JumpRelativeOffset
      86 => Instruction::JumpZero(Arg::Runtime(r.read_u16::<BigEndian>()?)), // JumpZero
      87 => Instruction::JumpNonZero(Arg::Runtime(r.read_u16::<BigEndian>()?)), // JumpNonZero
      88 => Instruction::SetPaint,
      89 => Instruction::GetPaint,
      90 => Instruction::Rand,
      i => return Err(Error::BadInstructionOpCode(i)),
    };
    code.push(instr);
    Ok(())
  }

  pub fn load_from_reader<R: ReadBytesExt>(&mut self, r: &mut R) -> Result<mfm::Metadata, Error> {
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
    elem.type_num = type_num;

    for _ in 0..r.read_u8()? {
      Self::read_metadata(r, &mut elem)?;
    }

    trace!("{:?}", elem);

    let mut code = Vec::new();

    for _ in 0..r.read_u16::<BigEndian>()? {
      Self::read_instruction(r, &mut code)?;
    }

    trace!("{:?}", code);

    self.type_map.insert(type_num, elem.clone());
    self.code_map.insert(type_num, code);
    Ok(elem)
  }

  pub fn execute<T: mfm::EventWindow + mfm::Rand>(
    ew: &mut T,
    cursor: &mut Cursor,
    code_map: &HashMap<u16, Vec<Instruction<'input>>>,
  ) -> Result<(), Error> {
    let my_atom = ew.get(0);
    let my_type: u16 = my_atom.apply(&FieldSelector::TYPE).into();
    let code = code_map
      .get(&my_type)
      .ok_or(Error::UnknownElement(my_type))?;
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
          let j: usize = cursor.pop_site();
          let i: usize = cursor.pop_site();
          ew.swap(i, j);
        }
        Instruction::SetSite => {
          let c = cursor.pop();
          let i: usize = cursor.pop_site();
          ew.set(i, c);
        }
        Instruction::SetField(f) => {
          let c = cursor.pop();
          let mut a = cursor.pop();
          let fi = f.runtime();
          a.store(c, fi);
          cursor.op_stack.push(a);
        }
        Instruction::SetSiteField(f) => {
          let c = cursor.pop();
          let i: usize = cursor.pop_site();
          let fi = f.runtime();
          let mut a = ew.get(i);
          a.store(c, fi);
          ew.set(i, a);
        }
        Instruction::GetSite => {
          let v = ew.get(cursor.pop_site());
          cursor.op_stack.push(v);
        }
        Instruction::GetField(f) => {
          let a = cursor.pop();
          cursor.op_stack.push(a.apply(f.runtime()));
        }
        Instruction::GetSiteField(f) => {
          let i: usize = cursor.pop_site();
          cursor.op_stack.push(ew.get(i).apply(f.runtime()));
        }
        Instruction::GetSignedField(f) => {
          let i: i128 = cursor.pop().apply(f.runtime()).into();
          cursor.op_stack.push(i.into());
        }
        Instruction::GetSignedSiteField(f) => {
          let i: usize = cursor.pop_site();
          let i: i128 = ew.get(i).apply(f.runtime()).into();
          cursor.op_stack.push(i.into());
        }
        Instruction::GetType(x) => cursor.op_stack.push((*x.runtime()).into()),
        Instruction::GetParameter(c) => {
          cursor.op_stack.push(*c.runtime());
        }
        Instruction::Scan => todo!(),
        Instruction::SaveSymmetries => cursor.symmetries_stack.push(cursor.symmetry),
        Instruction::UseSymmetries(x) => cursor.symmetry = mfm::select_symmetries(ew.rand_u32(), x),
        Instruction::RestoreSymmetries => cursor.symmetry = cursor.symmetries_stack.pop().unwrap(),
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
          let t = cursor.pop();
          cursor.op_stack.push(t);
          cursor.op_stack.push(t);
        }
        Instruction::Over => {
          let n = cursor.op_stack.len();
          let a = cursor.op_stack[n - 2];
          cursor.op_stack.push(a);
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
          let b = cursor.pop();
          let a = cursor.pop();
          cursor.op_stack.push(a + b);
        }
        Instruction::Sub => {
          let b = cursor.pop();
          let a = cursor.pop();
          cursor.op_stack.push(a - b);
        }
        Instruction::Neg => {
          let a = cursor.pop();
          cursor.op_stack.push(-a);
        }
        Instruction::Mod => {
          let b = cursor.pop();
          let a = cursor.pop();
          cursor.op_stack.push(a % b);
        }
        Instruction::Mul => {
          let b = cursor.pop();
          let a = cursor.pop();
          cursor.op_stack.push(a * b);
        }
        Instruction::Div => {
          let b = cursor.pop();
          let a = cursor.pop();
          cursor.op_stack.push(a / b);
        }
        Instruction::Less => {
          let b = cursor.pop();
          let a = cursor.pop();
          cursor.op_stack.push(if a < b { 1 } else { 0 }.into());
        }
        Instruction::LessEqual => {
          let b = cursor.pop();
          let a = cursor.pop();
          cursor.op_stack.push(if a <= b { 1 } else { 0 }.into());
        }
        Instruction::Or => {
          let b = cursor.pop();
          let a = cursor.pop();
          cursor.op_stack.push(a | b);
        }
        Instruction::And => {
          let b = cursor.pop();
          let a = cursor.pop();
          cursor.op_stack.push(a & b);
        }
        Instruction::Xor => {
          let b = cursor.pop();
          let a = cursor.pop();
          cursor.op_stack.push(a ^ b);
        }
        Instruction::Equal => {
          let b = cursor.pop();
          let a = cursor.pop();
          cursor.op_stack.push(if a == b { 1 } else { 0 }.into())
        }
        Instruction::BitCount => {
          let a = cursor.pop();
          cursor.op_stack.push(a.count_ones().into());
        }
        Instruction::BitScanForward => {
          let a = cursor.pop();
          cursor.op_stack.push(a.bitscanforward().into());
        }
        Instruction::BitScanReverse => {
          let a = cursor.pop();
          cursor.op_stack.push(a.bitscanreverse().into());
        }
        Instruction::LShift => {
          let b = cursor.pop();
          let a = cursor.pop();
          cursor.op_stack.push(a >> b.into()) // TODO handle b overflow
        }
        Instruction::RShift => {
          let b = cursor.pop();
          let a = cursor.pop();
          cursor.op_stack.push(a << b.into()) // TODO handle b overflow
        }
        Instruction::Jump(x) => {
          cursor.ip = *x.runtime() as usize;
          continue;
        }
        Instruction::JumpRelativeOffset => {
          let a = cursor.pop();
          assert!(!a.is_zero());
          match a {
            Const::Unsigned(x) => cursor.ip += x as usize,
            Const::Signed(_) => {
              let amount = a.abs();
              if amount.is_neg() {
                if let Some(ip) = cursor.ip.checked_sub(amount.into()) {
                  cursor.ip = ip;
                } else {
                  cursor.ip = u16::MAX as usize;
                  continue;
                }
              } else {
                cursor.ip = cursor.ip.saturating_add(amount.into());
              }
            }
          }
          continue;
        }
        Instruction::JumpZero(x) => {
          if cursor.pop().is_zero() {
            cursor.ip = *x.runtime() as usize;
            continue;
          }
        }
        Instruction::JumpNonZero(x) => {
          if !cursor.pop().is_zero() {
            cursor.ip = *x.runtime() as usize;
            continue;
          }
        }
        Instruction::SetPaint => {
          let c: u32 = cursor.pop().into();
          ew.set_paint(c.into());
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
