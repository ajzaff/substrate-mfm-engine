use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Copy, Clone)]
pub struct Element<'a> {
  name: &'a str,
  symbol: &'a str,
  program: &'a Program<'a>,
  radius: u8,
}

impl<'a> Element<'a> {
  const Empty: &'a Element<'a> = &Self {
    name: "Empty",
    symbol: " ",
    program: &Program::new(),
    radius: 0,
  };
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Symmetries {
  R000L = 1, // Normal.
  R090L = 2,
  R180L = 4, // Flip_XY.
  R270L = 8,
  R000R = 16, // Flip_Y.
  R090R = 32,
  R180R = 64, // Flip_X.
  R270R = 128,
}

#[derive(Copy, Clone)]
struct Atom(u128);

impl Atom {
  const Mask: u128 = 0xffffffffffffffffffffffff;

  pub fn get_type(self) -> u16 {
    ((self.0 & 0xffff000000000000) >> 24) as u16
  }

  pub fn get_ecc(self) -> u16 {
    ((self.0 & 0xff8000000000) >> 15) as u16
  }

  pub fn get_header(self) -> u32 {
    ((self.0 & 0xffffff8000000000) >> 15) as u32
  }

  pub fn get_state(self) -> u128 {
    (self.0 & 0x7fffffffffffffffff) as u128
  }
}

#[derive(Copy, Clone)]
pub struct EventWindow {
  origin: usize,
  radius: u8,
}

impl EventWindow {
  pub fn new(origin: usize, radius: u8) -> EventWindow {
    EventWindow {
      origin: origin,
      radius: radius,
    }
  }

  pub fn reset(&mut self) {
    self.origin = 0;
    self.radius = 0;
  }
}

#[derive(Copy, Clone)]
pub struct Runtime<'a> {
  ew: EventWindow,
  elem: &'a Element<'a>,
  tile: &'a Tile<'a>,
  physics: Physics<'a>,
  registers: [u128; 16],
  symmetries: Symmetries,
  ip: usize,
}

impl<'a> Runtime<'a> {
  pub fn new(tile: &'a Tile<'a>, physics: Physics<'a>) -> Self {
    Self {
      ew: EventWindow::new(0, 0),
      elem: Element::Empty,
      tile: tile,
      physics: physics,
      registers: [0; 16],
      symmetries: Symmetries::R000L, // Normal
      ip: 0,
    }
  }

  pub fn reset(&mut self) {
    self.ew.reset();
    for i in self.registers.iter_mut() {
      *i = 0;
    }
    self.symmetries = Symmetries::R000L;
    self.ip = 0;
  }

  pub fn heap_lookup(&self, i: usize) -> Option<u128> {
    return Some(0);
  }

  pub fn register_lookup(&self, i: usize) -> Option<u128> {
    let r = Register::from_usize(i);
    if r.is_none() {
      return None;
    }
    if r.unwrap() == Register::RUniformRandom {
      self.registers[Register::RUniformRandom as usize] = rand::random::<u128>() & Atom::Mask;
    }
    return Some(self.registers[r.unwrap() as usize]);
  }

  pub fn use_symmetries(&mut self, symmetries: Symmetries) {
    self.symmetries = symmetries
  }

  pub fn copy(r: &mut Runtime, dst: Arg, src: Arg) -> Result<(), &'static str> {
    match dst.vtype {
      Some(ValueType::Inline) => Err("inline value is not writeable"),
      Some(ValueType::Heap) => Err("heap value is not writeable"),
      Some(ValueType::Register) => {
        let lookup = ValueType::value_to_usize_field(dst.data);
        match Register::from_usize(lookup.0) {
          Some(Register::RUniformRandom) => return Err("random register is not writeable"),
          Some(_) => (),
          None => return Err("bad destination register"),
        }
        let val: Option<u128> = match src.vtype {
          Some(ValueType::Inline) => Some(src.data as u128),
          Some(ValueType::Heap) => self.heap_lookup(src.data as usize),
          Some(ValueType::Register) => Some(0),
          Some(ValueType::Site) => Some(0),
          None => return Err("bad source value type"),
        };
        if val.is_none() {
          return Err("bad source value");
        }
        match lookup.1 {
          Some(_) => {}
          None => r.registers[lookup.0] = val.unwrap(),
        }
      }
      Some(ValueType::Site) => {
        let lookup = ValueType::value_to_usize_field(dst.1);
      }
      None => Err("bad destination value type"),
    }
  }

  pub fn swap(r: &mut Runtime, dst: Arg, src: Arg) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn scan(r: &mut Runtime, dst: Arg, src: Arg) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn checksum(r: &mut Runtime, dst: Arg, src: Arg) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn step(r: &mut Runtime) -> Result<(), &'static str> {
    let ew = r.ew;

    if ew.origin >= tile.sites.len() {
      return Err("bad origin site");
    }

    let a = tile.sites[ew.origin];
    let t = atom_type(a);

    if (t as usize) >= tile.physics.elements.len() {
      return Err("bad origin atom type");
    }

    let elem = tile.physics.elements[t as usize];
    let prog = elem.program;

    if r.ip >= prog.code.len() {
      return Ok(());
    }

    let instr = prog.code[r.ip];
    r.ip += 1;

    match instr.op {
      Some(Op::Nop) => Ok(()),
      Some(Op::Exit) => Ok(()),
      Some(Op::Copy) => Runtime::copy(r, instr.dst, instr.lhs),
      Some(Op::Swap) => Runtime::swap(r, instr.dst, instr.lhs),
      Some(Op::Scan) => Runtime::scan(r, instr.dst, instr.lhs),
      Some(Op::Checksum) => Runtime::checksum(r, instr.dst, instr.lhs),
      Some(Op::UseSymmetries) => r.use_symmetries(),
      Some(Op::RestoreSymmetries) => return,
      Some(Op::Add) => return,
      Some(Op::Sub) => return,
      Some(Op::Negate) => return,
      Some(Op::Mod) => return,
      Some(Op::Mul) => return,
      Some(Op::Div) => return,
      Some(Op::LessThan) => return,
      Some(Op::LessThanEqual) => return,
      Some(Op::Or) => return,
      Some(Op::And) => return,
      Some(Op::Xor) => return,
      Some(Op::Equal) => return,
      Some(Op::BitCount) => return,
      Some(Op::BitScanForward) => return,
      Some(Op::BitScanReverse) => return,
      Some(Op::LShift) => return,
      Some(Op::RShift) => return,
      Some(Op::Jump) => return,
      Some(Op::JumpRelativeOffset) => return,
      Some(Op::JumpZero) => return,
      Some(Op::JumpNonZero) => return,
      None => return, // TODO return an error.
    }
  }
}

#[repr(u8)]
pub enum DataType {
  Unsigned,
  Signed,
}

pub struct Program<'a> {
  fields: &'a [Field],
  consts: &'a [Const],
  code: &'a [Instruction],
}

impl<'a> Program<'a> {
  pub fn new() -> Self {
    Self {
      fields: &[],
      consts: &[],
      code: &[],
    }
  }
}

#[derive(Copy, Clone)]
pub struct Instruction {
  pub op: Option<Op>,
  pub dst: Arg,
  pub lhs: Arg,
  pub rhs: Arg,
}

impl Instruction {
  pub fn from_u64(x: u64) -> Self {
    Self {
      op: FromPrimitive::from_u64((x & 0xff000000000000) >> 48),
      dst: Arg::from_u16(((x & 0xffff00000000) >> 32) as u16),
      lhs: Arg::from_u16(((x & 0xffff0000) >> 16) as u16),
      rhs: Arg::from_u16((x & 0xffff) as u16),
    }
  }
}

#[derive(Copy, Clone)]
pub struct Arg {
  pub vtype: Option<ValueType>,
  pub data: u16,
}

impl Arg {
  pub fn from_u16(x: u16) -> Self {
    Self {
      vtype: ValueType::from_u8(((x & 0xc000) >> 14) as u8),
      data: x & 0x3fff,
    }
  }
}

#[repr(u8)]
#[derive(Copy, Clone, FromPrimitive)]
pub enum Op {
  Nop,
  Exit,
  Copy,
  Swap,
  Scan,
  Checksum,
  UseSymmetries,
  RestoreSymmetries,
  Add,
  Sub,
  Negate,
  Mod,
  Mul,
  Div,
  LessThan,
  LessThanEqual,
  Or,
  And,
  Xor,
  Equal,
  BitCount,
  BitScanForward,
  BitScanReverse,
  LShift,
  RShift,
  Jump,
  JumpRelativeOffset,
  JumpZero,
  JumpNonZero,
}

#[repr(u8)]
#[derive(Copy, Clone, FromPrimitive)]
pub enum ValueType {
  Inline,
  Heap,
  Register,
  Site,
}

impl ValueType {
  fn from_u8(x: u8) -> Option<Self> {
    FromPrimitive::from_u8(x)
  }

  fn value_to_usize_field(x: u16) -> (usize, Option<usize>) {
    let i = ((x & 0x3f80) >> 7) as usize;
    let f = (x & 0x7f) as usize;
    if f == 0 {
      return (i, None);
    }
    (i, Some(f))
  }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Register {
  R0,
  R1,
  R2,
  R3,
  R4,
  R5,
  R6,
  R7,
  R8,
  R9,
  R10,
  R11,
  R12,
  R13,
  R14,
  RUniformRandom,
}

impl Register {
  fn from_usize(x: usize) -> Option<Self> {
    FromPrimitive::from_usize(x)
  }
}

pub struct Field {
  dtype: DataType,
  length: u8,
  offset: u8,
}

pub struct Const {
  dtype: DataType,
  value: u128,
}

pub struct Tile<'a> {
  sites: &'a [Atom],
  bounds: (u16, u16),
  physics: Physics<'a>,
}

impl<'a> Tile<'a> {
  pub fn new(sites: &'a [Atom], bounds: (u16, u16), physics: Physics<'a>) -> Tile<'a> {
    Tile {
      sites: sites,
      bounds: bounds,
      physics: physics,
    }
  }
}

#[derive(Copy, Clone)]
pub struct Physics<'a> {
  elements: &'a [Element<'a>],
}
