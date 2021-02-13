use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::vec::Vec;

#[derive(Copy, Clone)]
pub struct Element<'a> {
  name: &'a str,
  symbol: &'a str,
  program: Program<'a>,
  radius: u8,
}

impl<'a> Element<'a> {
  const EMPTY: &'a Element<'a> = &Self {
    name: "Empty",
    symbol: " ",
    program: Program::EMPTY,
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

#[derive(Copy, Clone, Debug, FromPrimitive)]
pub struct Site(u8);

impl Site {
  pub const LIMIT: usize = 41;

  pub fn from_usize(x: usize) -> Option<Site> {
    if x < Self::LIMIT {
      FromPrimitive::from_usize(x)
    } else {
      None
    }
  }
}

#[derive(Copy, Clone)]
pub struct Atom(u128);

impl Atom {
  pub const MASK: u128 = 0xffffffffffffffffffffffff;

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

pub struct Runtime<'a> {
  ew: EventWindow,
  elem: &'a Element<'a>,
  tile: &'a mut Tile<'a>,
  physics: Physics<'a>,
  registers: [u128; 16],
  heap: Vec<u128>,
  default_symmetries: Symmetries,
  current_symmetries: Symmetries,
  ip: usize,
}

impl<'a> Runtime<'a> {
  pub fn new(tile: &'a mut Tile<'a>, physics: Physics<'a>) -> Self {
    Self {
      ew: EventWindow::new(0, 0),
      elem: Element::EMPTY,
      tile: tile,
      physics: physics,
      registers: [0; 16],
      heap: Vec::new(),
      default_symmetries: Symmetries::R000L, // Normal
      current_symmetries: Symmetries::R000L,
      ip: 0,
    }
  }

  pub fn reset(&mut self) {
    self.ew.reset();
    for i in self.registers.iter_mut() {
      *i = 0;
    }
    self.default_symmetries = Symmetries::R000L;
    self.current_symmetries = Symmetries::R000L;
    self.ip = 0;
  }

  pub fn get_value_mut(&mut self, x: Value) -> Result<&mut u128, &'static str> {
    match x.vtype {
      Some(ValueType::Inline) => Err("inline value is immutable"),
      Some(ValueType::Heap) => Err("heap value is immutable"),
      Some(ValueType::Register) => match Register::from_usize(x.data as usize) {
        Some(Register::RUniformRandom) => Err("random register is immutable"),
        Some(x) => Ok(&mut self.registers[x as usize]),
        None => Err("bad register argument"),
      },
      Some(ValueType::Site) => match Site::from_usize(x.data as usize) {
        Some(x) => Ok(&mut self.tile.sites[x.0 as usize].0),
        None => Err("bad site number"),
      },
      None => Err("bad argument type"),
    }
  }

  pub fn get_value(&self, x: Value) -> Result<u128, &'static str> {
    match x.vtype {
      Some(ValueType::Inline) => Ok(x.data as u128),
      Some(ValueType::Heap) => Ok(self.heap[x.data as usize]),
      Some(ValueType::Register) => match Register::from_usize(x.data as usize) {
        Some(Register::RUniformRandom) => Ok(rand::random::<u128>() & Atom::MASK),
        Some(x) => Ok(self.registers[x as usize]),
        None => Err("bad register argument"),
      },
      Some(ValueType::Site) => match Site::from_usize(x.data as usize) {
        Some(x) => Ok(self.tile.sites[x.0 as usize].0),
        None => Err("bad site number"),
      },
      None => Err("bad argument type"),
    }
  }

  pub fn use_symmetries(&mut self, symmetries: Symmetries) {
    self.current_symmetries = symmetries
  }

  pub fn restore_symmetries(&mut self) {
    self.current_symmetries = self.default_symmetries
  }

  pub fn copy(r: &mut Runtime, dst: Value, src: Value) -> Result<(), &'static str> {
    let x = r.get_value_mut(dst);
    if x.is_err() {
      return Err(x.unwrap_err());
    }
    let y = r.get_value_mut(src); // TODO: Fix me
    if y.is_err() {
      return Err(y.unwrap_err());
    }
    *x.unwrap() = *y.unwrap();
    Ok(())
  }

  pub fn swap(r: &mut Runtime, dst: Value, src: Value) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn scan(r: &mut Runtime, dst: Value, src: Value) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn checksum(r: &mut Runtime, dst: Value, src: Value) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn add(r: &mut Runtime, dst: Value, lhs: Value, rhs: Value) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn sub(r: &mut Runtime, dst: Value, lhs: Value, rhs: Value) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn negate(r: &mut Runtime, dst: Value, src: Value) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn modulo(r: &mut Runtime, dst: Value, lhs: Value, rhs: Value) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn mul(r: &mut Runtime, dst: Value, lhs: Value, rhs: Value) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn div(r: &mut Runtime, dst: Value, lhs: Value, rhs: Value) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn less_than(
    r: &mut Runtime,
    dst: Value,
    lhs: Value,
    rhs: Value,
  ) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn less_than_equal(
    r: &mut Runtime,
    dst: Value,
    lhs: Value,
    rhs: Value,
  ) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn or(r: &mut Runtime, dst: Value, lhs: Value, rhs: Value) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn and(r: &mut Runtime, dst: Value, lhs: Value, rhs: Value) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn xor(r: &mut Runtime, dst: Value, lhs: Value, rhs: Value) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn equal(r: &mut Runtime, dst: Value, lhs: Value, rhs: Value) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn bit_count(r: &mut Runtime, dst: Value, src: Value) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn bit_scan_forward(r: &mut Runtime, dst: Value, src: Value) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn bit_scan_reverse(r: &mut Runtime, dst: Value, src: Value) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn lshift(r: &mut Runtime, dst: Value, lhs: Value, rhs: Value) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn rshift(r: &mut Runtime, dst: Value, lhs: Value, rhs: Value) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn jump(r: &mut Runtime, label: Value) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn jump_relative_offset(r: &mut Runtime, dst: Value, lhs: Value) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn jump_zero(r: &mut Runtime, label: Value, src: Value) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn jump_non_zero(r: &mut Runtime, label: Value, src: Value) -> Result<(), &'static str> {
    Ok(())
  }

  pub fn step(r: &mut Runtime) -> Result<(), &'static str> {
    let ew = r.ew;

    if ew.origin >= r.tile.sites.len() {
      return Err("bad origin site");
    }

    let a = r.tile.sites[ew.origin];
    let t = a.get_type();

    if (t as usize) >= r.tile.physics.elements.len() {
      return Err("bad origin atom type");
    }

    let elem = r.tile.physics.elements[t as usize];
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
      Some(Op::Add) => Runtime::add(r, instr.dst, instr.lhs, instr.rhs),
      Some(Op::Sub) => Runtime::sub(r, instr.dst, instr.lhs, instr.rhs),
      Some(Op::Negate) => Runtime::negate(r, instr.dst, instr.lhs),
      Some(Op::Mod) => Runtime::modulo(r, instr.dst, instr.lhs, instr.rhs),
      Some(Op::Mul) => Runtime::mul(r, instr.dst, instr.lhs, instr.rhs),
      Some(Op::Div) => Runtime::div(r, instr.dst, instr.lhs, instr.rhs),
      Some(Op::LessThan) => Runtime::less_than(r, instr.dst, instr.lhs, instr.rhs),
      Some(Op::LessThanEqual) => Runtime::less_than_equal(r, instr.dst, instr.lhs, instr.rhs),
      Some(Op::Or) => Runtime::or(r, instr.dst, instr.lhs, instr.rhs),
      Some(Op::And) => Runtime::and(r, instr.dst, instr.lhs, instr.rhs),
      Some(Op::Xor) => Runtime::xor(r, instr.dst, instr.lhs, instr.rhs),
      Some(Op::Equal) => Runtime::equal(r, instr.dst, instr.lhs, instr.rhs),
      Some(Op::BitCount) => Runtime::bit_count(r, instr.dst, instr.lhs),
      Some(Op::BitScanForward) => Runtime::bit_scan_forward(r, instr.dst, instr.lhs),
      Some(Op::BitScanReverse) => Runtime::bit_scan_reverse(r, instr.dst, instr.lhs),
      Some(Op::LShift) => Runtime::lshift(r, instr.dst, instr.lhs, instr.rhs),
      Some(Op::RShift) => Runtime::rshift(r, instr.dst, instr.lhs, instr.rhs),
      Some(Op::Jump) => Runtime::jump(r, instr.dst),
      Some(Op::JumpRelativeOffset) => Runtime::jump_relative_offset(r, instr.dst, instr.lhs),
      Some(Op::JumpZero) => Runtime::jump_zero(r, instr.dst, instr.lhs),
      Some(Op::JumpNonZero) => Runtime::jump_non_zero(r, instr.dst, instr.lhs),
      None => Err("bad op"),
    }
  }
}

#[repr(u8)]
pub enum DataType {
  Unsigned,
  Signed,
}

#[derive(Copy, Clone)]
pub struct Program<'a> {
  code: &'a [Instruction],
}

impl<'a> Program<'a> {
  const EMPTY: Program<'a> = Self { code: &[] };

  pub fn new() -> Self {
    Self { code: &[] }
  }
}

#[derive(Copy, Clone)]
pub struct Instruction {
  pub op: Option<Op>,
  pub dst: Value,
  pub lhs: Value,
  pub rhs: Value,
}

impl Instruction {
  pub fn from_u64(x: u64) -> Self {
    Self {
      op: FromPrimitive::from_u64((x & 0xff000000000000) >> 48),
      dst: Value::from_u16(((x & 0xffff00000000) >> 32) as u16),
      lhs: Value::from_u16(((x & 0xffff0000) >> 16) as u16),
      rhs: Value::from_u16((x & 0xffff) as u16),
    }
  }
}

#[derive(Copy, Clone)]
pub struct Value {
  vtype: Option<ValueType>,
  data: u16,
}

impl Value {
  pub fn from_u16(x: u16) -> Self {
    Self {
      vtype: ValueType::from_u8(((x & 0xc000) >> 12) as u8),
      data: x & 0xfff,
    }
  }

  pub fn get_value(self) -> Option<u16> {
    match self.vtype {
      Some(ValueType::Inline) => Some(self.data),
      _ => None,
    }
  }

  pub fn get_reference(self) -> Option<usize> {
    match self.vtype {
      Some(ValueType::Heap) => Some(self.data as usize),
      Some(ValueType::Register) => Some(((self.data & 0x3f80) >> 7) as usize),
      Some(ValueType::Site) => Some(((self.data & 0x3f80) >> 7) as usize),
      _ => None,
    }
  }

  pub fn get_field(self) -> Option<usize> {
    match self.vtype {
      Some(ValueType::Register) => Some((self.data & 0x7f) as usize),
      Some(ValueType::Site) => Some((self.data & 0x7f) as usize),
      _ => None,
    }
  }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum ValueType {
  Inline,
  Heap,
  Register,
  Site,
}

impl ValueType {
  pub fn from_u8(x: u8) -> Option<ValueType> {
    FromPrimitive::from_u8(x)
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

#[derive(Copy, Clone, PartialEq, PartialOrd, FromPrimitive)]
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
  sites: &'a mut [Atom],
  bounds: (u16, u16),
  physics: Physics<'a>,
}

impl<'a> Tile<'a> {
  pub fn new(sites: &'a mut [Atom], bounds: (u16, u16), physics: Physics<'a>) -> Tile<'a> {
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
