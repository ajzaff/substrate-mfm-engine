mod base;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::base::Register;
use std::vec::Vec;

#[derive(Copy, Clone)]
pub struct Element<'a> {
  props: &'a ElementProperties<'a>,
  program: &'a Program<'a>,
}

impl<'a> Element<'a> {
  const EMPTY: &'a Element<'a> = &Self {
    props: ElementProperties::EMPTY,
    program: Program::EMPTY,
  };
}

#[derive(Copy, Clone)]
pub struct ElementProperties<'a> {
  name: &'a str,
  symbol: &'a str,
  desc: &'a str,
  authors: &'a [&'a str],
  license: &'a str,
  radius: usize,
  bg_color: &'a str,
  fg_color: &'a str,
  symmetries: Symmetries,
  fields: &'a [NamedField<'a>],
  params: &'a [NamedParameter<'a>],
}

impl<'a> ElementProperties<'a> {
  const EMPTY: &'a ElementProperties<'a> = &Self {
    name: "Empty",
    symbol: " ",
    desc: "Empty.",
    authors: &[],
    license: "",
    radius: 0,
    bg_color: "#000",
    fg_color: "#000",
    symmetries: Symmetries::None,
    fields: &[],
    params: &[],
  };
}

#[derive(Copy, Clone)]
pub struct NamedField<'a> {
  name: &'a str,
  field: Field,
}

#[derive(Copy, Clone)]
pub struct NamedParameter<'a> {
  name: &'a str,
  value: u128,
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Symmetries {
  None = 0,
  R000L = 1, // Normal.
  R090L = 2,
  R180L = 4, // Flip_XY.
  R270L = 8,
  R000R = 16, // Flip_Y.
  R090R = 32,
  R180R = 64, // Flip_X.
  R270R = 128,
  All = 255,
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
  pub const TYPE_MASK: u128 = 0xffff00000000000000000000;
  pub const ECC_MASK: u128 = 0xff800000000000000000;
  pub const HEADER_MASK: u128 = Self::TYPE_MASK | Self::ECC_MASK;
  pub const USER_MASK: u128 = 0xfffffffffffffffff;

  pub fn get_type(self) -> u16 {
    ((self.0 & Self::TYPE_MASK) >> 24) as u16
  }

  pub fn get_ecc(self) -> u16 {
    ((self.0 & Self::ECC_MASK) >> 15) as u16
  }

  pub fn get_header(self) -> u32 {
    ((self.0 & Self::HEADER_MASK) >> 15) as u32
  }

  pub fn get_state(self) -> u128 {
    (self.0 & Self::USER_MASK) as u128
  }
}

pub struct EventWindow<'a> {
  tile: &'a mut Tile<'a>,
  origin: usize,
  radius: usize,
}

impl<'a> EventWindow<'a> {
  pub fn new(tile: &'a mut Tile<'a>, origin: usize, radius: usize) -> EventWindow<'a> {
    EventWindow {
      tile: tile,
      origin: origin,
      radius: radius,
    }
  }

  fn set_origin(&mut self, i: usize) {
    self.origin = i
  }

  fn set_radius(&mut self, i: usize) {
    self.radius = i
  }

  const ys: [i32; 41] = [
    0, 0, -1, 1, 0, -1, 1, -1, 1, 0, -2, 2, 0, -1, 1, -2, 2, -2, 2, -1, 1, 0, -3, 3, 0, -2, 2, -2,
    2, -1, 1, -3, 3, -3, 3, -1, 1, 0, -4, 4, 0,
  ];
  const xs: [i32; 41] = [
    0, -1, 0, 0, 1, -1, -1, 1, 1, -2, 0, 0, 2, -2, -2, -1, -1, 1, 1, 2, 2, -3, 0, 0, 3, -2, -2, 2,
    2, -3, -3, -1, -1, 1, 1, 3, 3, -4, 0, 0, 4,
  ];

  fn add_sites_on_tile(&self, i: usize, delta: usize) -> Option<usize> {
    let dx = Self::xs[delta];
    let x = (i as u16) % self.tile.bounds.0;
    let new_x = x as i32 + dx;
    if new_x < 0 || new_x >= self.tile.bounds.0 as i32 {
      return None;
    }
    let dy = Self::ys[delta];
    let y = (i as u16) / self.tile.bounds.0;
    let new_y = y as i32 + dy;
    if new_y < 0 || new_y >= self.tile.bounds.1 as i32 {
      return None;
    }
    Some((new_y * self.tile.bounds.0 as i32 + new_x) as usize)
  }

  pub fn at(&self, i: usize) -> Option<&Atom> {
    self
      .add_sites_on_tile(self.origin, i)
      .and_then(|i| self.tile.get(i))
  }

  pub fn at_mut(&mut self, i: usize) -> Option<&mut Atom> {
    self
      .add_sites_on_tile(self.origin, i)
      .and_then(move |i| self.tile.get_mut(i))
  }
}

pub struct Runtime<'a> {
  ew: &'a mut EventWindow<'a>,
  registers: [u128; 16],
  labels: Vec<usize>,
  heap: Vec<u128>,
  default_symmetries: Symmetries,
  current_symmetries: Symmetries,
  ip: usize,
}

impl<'a> Runtime<'a> {
  pub fn new(ew: &'a mut EventWindow<'a>) -> Self {
    Self {
      ew: ew,
      registers: [0; 16],
      labels: Vec::new(),
      heap: Vec::new(),
      default_symmetries: Symmetries::R000L, // Normal
      current_symmetries: Symmetries::R000L,
      ip: 0,
    }
  }

  pub fn use_symmetries(&mut self, symmetries: Symmetries) {
    self.current_symmetries = symmetries
  }

  pub fn restore_symmetries(&mut self) {
    self.current_symmetries = self.default_symmetries
  }

  pub fn get_value_u128(&self, x: Value) -> Result<u128, &'static str> {
    match x.get_type() {
      Some(ValueType::Inline) => x.get_inline().map(|x| x as u128).ok_or("bad inline fetch"),
      Some(ValueType::Heap) => x.get_heap().map(|x| x as u128).ok_or("bad heap fetch"),
      Some(ValueType::Register) => x
        .get_register()
        .and_then(|v| match Register::from_usize(v as usize) {
          Some(Register::RRand) => Some(rand::random::<u128>() & Atom::MASK),
          Some(x) => Some(self.registers[x as usize]),
          None => None,
        })
        .ok_or("bad register"),
      Some(ValueType::Site) => x
        .get_site()
        .and_then(|v| match Site::from_usize(v as usize) {
          Some(x) => self.ew.at(x.0 as usize).map(|a| a.0),
          None => None,
        })
        .ok_or("bad site"),
      None => Err("bad value type"),
    }
  }

  pub fn store_const(&mut self, dst: Value, c: u128) -> Result<(), &'static str> {
    match dst.get_type() {
      Some(ValueType::Inline) => Err("inline value is immutable"),
      Some(ValueType::Heap) => Err("heap is immutable"),
      Some(ValueType::Register) => dst
        .get_register()
        .and_then(|v| match Register::from_usize(v as usize) {
          Some(Register::RRand) => None,
          Some(x) => Some(&mut self.registers[x as usize]),
          None => None,
        })
        .ok_or("bad register"),
      Some(ValueType::Site) => dst
        .get_site()
        .and_then(|v| match Site::from_usize(v as usize) {
          Some(x) => self.ew.at_mut(x.0 as usize).map(|a| &mut a.0),
          None => None,
        })
        .ok_or("bad site"),
      None => Err("bad destination type"),
    }
    .and_then(|result| {
      *result = c;
      Ok(())
    })
  }

  pub fn store_binary_op(
    self: &mut Runtime<'a>,
    dst: Value,
    lhs: Value,
    rhs: Value,
    op: fn(u128, u128) -> u128,
  ) -> Result<(), &'static str> {
    self.get_value_u128(lhs).and_then(|x| {
      self
        .get_value_u128(rhs)
        .and_then(|y| self.store_const(dst, op(x, y)))
    })
  }

  pub fn store_unary_op(
    self: &mut Runtime<'a>,
    dst: Value,
    src: Value,
    op: fn(u128) -> u128,
  ) -> Result<(), &'static str> {
    self
      .get_value_u128(src)
      .and_then(|x| self.store_const(dst, op(x)))
  }

  pub fn copy(r: &mut Runtime, dst: Value, src: Value) -> Result<(), &'static str> {
    r.get_value_u128(src).and_then(|c| r.store_const(dst, c))
  }

  pub fn swap(r: &mut Runtime, dst: Value, src: Value) -> Result<(), &'static str> {
    r.get_value_u128(dst).and_then(|t| {
      r.get_value_u128(src)
        .and_then(|y| r.store_const(dst, y).and_then(|_| r.store_const(src, t)))
    })
  }

  pub fn scan(r: &mut Runtime, dst: Value, src: Value) -> Result<(), &'static str> {
    Err("not implemented")
  }

  pub fn checksum(r: &mut Runtime, dst: Value, src: Value) -> Result<(), &'static str> {
    Err("not implemented")
  }

  pub fn add(r: &mut Runtime, dst: Value, lhs: Value, rhs: Value) -> Result<(), &'static str> {
    r.store_binary_op(dst, lhs, rhs, |x, y| x + y)
  }

  pub fn sub(r: &mut Runtime, dst: Value, lhs: Value, rhs: Value) -> Result<(), &'static str> {
    r.store_binary_op(dst, lhs, rhs, |x, y| x - y) // FIXME: perform proper signed math.
  }

  pub fn negate(r: &mut Runtime, dst: Value, src: Value) -> Result<(), &'static str> {
    r.store_unary_op(dst, src, |x| -(x as i128) as u128) // FIXME: perform proper signed math.
  }

  pub fn modulo(r: &mut Runtime, dst: Value, lhs: Value, rhs: Value) -> Result<(), &'static str> {
    r.store_binary_op(dst, lhs, rhs, |x, y| x % y)
  }

  pub fn mul(r: &mut Runtime, dst: Value, lhs: Value, rhs: Value) -> Result<(), &'static str> {
    r.store_binary_op(dst, lhs, rhs, |x, y| x * y)
  }

  pub fn div(r: &mut Runtime, dst: Value, lhs: Value, rhs: Value) -> Result<(), &'static str> {
    r.store_binary_op(dst, lhs, rhs, |x, y| x / y)
  }

  pub fn less_than(
    r: &mut Runtime,
    dst: Value,
    lhs: Value,
    rhs: Value,
  ) -> Result<(), &'static str> {
    r.store_binary_op(dst, lhs, rhs, |x, y| (x < y) as u128)
  }

  pub fn less_than_equal(
    r: &mut Runtime,
    dst: Value,
    lhs: Value,
    rhs: Value,
  ) -> Result<(), &'static str> {
    r.store_binary_op(dst, lhs, rhs, |x, y| (x <= y) as u128)
  }

  pub fn or(r: &mut Runtime, dst: Value, lhs: Value, rhs: Value) -> Result<(), &'static str> {
    r.store_binary_op(dst, lhs, rhs, |x, y| x | y)
  }

  pub fn and(r: &mut Runtime, dst: Value, lhs: Value, rhs: Value) -> Result<(), &'static str> {
    r.store_binary_op(dst, lhs, rhs, |x, y| x & y)
  }

  pub fn xor(r: &mut Runtime, dst: Value, lhs: Value, rhs: Value) -> Result<(), &'static str> {
    r.store_binary_op(dst, lhs, rhs, |x, y| x ^ y)
  }

  pub fn equal(r: &mut Runtime, dst: Value, lhs: Value, rhs: Value) -> Result<(), &'static str> {
    r.store_binary_op(dst, lhs, rhs, |x, y| (x == y) as u128)
  }

  pub fn bit_count(r: &mut Runtime, dst: Value, src: Value) -> Result<(), &'static str> {
    r.store_unary_op(dst, src, |x| x.count_ones() as u128)
  }

  pub fn bit_scan_forward(r: &mut Runtime, dst: Value, src: Value) -> Result<(), &'static str> {
    r.store_unary_op(dst, src, |x| x.trailing_zeros() as u128)
  }

  pub fn bit_scan_reverse(r: &mut Runtime, dst: Value, src: Value) -> Result<(), &'static str> {
    r.store_unary_op(dst, src, |x| x.leading_zeros() as u128)
  }

  pub fn lshift(r: &mut Runtime, dst: Value, lhs: Value, rhs: Value) -> Result<(), &'static str> {
    r.store_binary_op(dst, lhs, rhs, |x, y| x >> y)
  }

  pub fn rshift(r: &mut Runtime, dst: Value, lhs: Value, rhs: Value) -> Result<(), &'static str> {
    r.store_binary_op(dst, lhs, rhs, |x, y| x << y)
  }

  pub fn jump(r: &mut Runtime, label: Value) -> Result<(), &'static str> {
    Err("not implemented")
  }

  pub fn jump_relative_offset(r: &mut Runtime, dst: Value, lhs: Value) -> Result<(), &'static str> {
    Err("not implemented")
  }

  pub fn jump_zero(r: &mut Runtime, label: Value, src: Value) -> Result<(), &'static str> {
    Err("not implemented")
  }

  pub fn jump_non_zero(r: &mut Runtime, label: Value, src: Value) -> Result<(), &'static str> {
    Err("not implemented")
  }

  pub fn step(r: &mut Runtime) -> Result<(), &'static str> {
    let t: u16;
    {
      let a: Option<&mut Atom>;
      a = r.ew.at_mut(0);
      if a.is_none() {
        return Ok(());
      }
      t = a.unwrap().get_type();
    }

    let elem: Option<&Element>;
    {
      let physics = &mut r.ew.tile.physics;
      elem = physics.get(t as usize);
      if elem.is_none() {
        return Err("bad atom");
      }
    }

    let prog = elem.unwrap().program;
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

#[derive(Copy, Clone)]
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
  const EMPTY: &'a Program<'a> = &Self { code: &[] };

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
      dst: Value(((x & 0xffff00000000) >> 32) as u16),
      lhs: Value(((x & 0xffff0000) >> 16) as u16),
      rhs: Value((x & 0xffff) as u16),
    }
  }
}

#[derive(Copy, Clone)]
pub struct Value(u16);

impl Value {
  pub fn get_type(self) -> Option<ValueType> {
    ValueType::from_u8(((self.0 & 0xc000) >> 12) as u8)
  }

  pub fn get_inline(self) -> Option<u16> {
    match self.get_type() {
      Some(ValueType::Inline) => Some(self.0 & 0x7fff),
      _ => None,
    }
  }

  pub fn get_heap(self) -> Option<usize> {
    match self.get_type() {
      Some(ValueType::Heap) => Some((self.0 & 0x7fff) as usize),
      _ => None,
    }
  }

  pub fn get_register(self) -> Option<usize> {
    match self.get_type() {
      Some(ValueType::Register) => Some((self.0 & 0x7f00) as usize),
      _ => None,
    }
  }

  pub fn get_site(self) -> Option<usize> {
    match self.get_type() {
      Some(ValueType::Site) => Some((self.0 & 0x7f00) as usize),
      _ => None,
    }
  }

  pub fn get_field(self) -> Option<usize> {
    match self.get_type() {
      Some(ValueType::Register) | Some(ValueType::Site) => Some((self.0 & 0xff) as usize),
      _ => None,
    }
    .and_then(|x| if x > 0 { Some(x) } else { None })
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

#[derive(Copy, Clone)]
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
  physics: &'a Physics<'a>,
}

impl<'a> Tile<'a> {
  pub fn new(sites: &'a mut [Atom], bounds: (u16, u16), physics: &'a Physics<'a>) -> Tile<'a> {
    Tile {
      sites: sites,
      bounds: bounds,
      physics: physics,
    }
  }

  pub fn get(&self, i: usize) -> Option<&Atom> {
    self.sites.get(i)
  }

  pub fn get_mut(&mut self, i: usize) -> Option<&mut Atom> {
    self.sites.get_mut(i)
  }
}

#[derive(Copy, Clone)]
pub struct Physics<'a> {
  elements: &'a [Element<'a>],
}

impl<'a> Physics<'a> {
  pub fn get(&self, i: usize) -> Option<&Element<'a>> {
    self.elements.get(i)
  }
}
