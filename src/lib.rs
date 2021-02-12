use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Copy, Clone)]
pub struct Element<'a> {
  name: &'a str,
  symbol: &'a str,
  program: &'a Program<'a>,
  radius: u8,
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

pub type Atom = u128;

pub fn atom_type(a: Atom) -> u16 {
  ((a & 0xffff000000000000) >> 24) as u16
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
pub struct RuntimeState {
  ew: EventWindow,
  registers: [u128; 15],
  symmetries: Symmetries,
  ip: usize,
}

impl RuntimeState {
  pub fn new() -> RuntimeState {
    RuntimeState {
      ew: EventWindow::new(0, 0),
      registers: [0; 15],
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

pub type Instruction = u64;

#[repr(u8)]
#[derive(FromPrimitive)]
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

impl Op {
  fn from_instruction(instr: Instruction) -> Option<Op> {
    FromPrimitive::from_u64((instr & 0xff000000000000) >> 48)
  }
}

pub type Arg = u16;

#[repr(u8)]
pub enum ValueType {
  Inline,
  Heap,
  Register,
  Site,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum RegisterExpr {
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

pub struct Physics<'a> {
  elements: &'a [Element<'a>],
}

pub fn step(tile: &Tile, runtime: &RuntimeState) {
  let ew = runtime.ew;

  if ew.origin >= tile.sites.len() {
    return; // TODO: return an error.
  }

  let a = tile.sites[ew.origin];
  let t = atom_type(a);

  if (t as usize) >= tile.physics.elements.len() {
    return; // TODO: return an error.
  }

  let elem = tile.physics.elements[t as usize];
  let prog = elem.program;

  if runtime.ip >= prog.code.len() {
    return; // TODO: return an error.
  }

  let instr = prog.code[runtime.ip];
  let op = Op::from_instruction(instr);

  match op {
    Some(x) => match x {
      Nop => return,
      Exit => return, // TODO: return exit code.
      Copy => return,
      Swap => return,
      Scan => return,
      Checksum => return,
      UseSymmetries => return,
      RestoreSymmetries => return,
      Add => return,
      Sub => return,
      Negate => return,
      Mod => return,
      Mul => return,
      Div => return,
      LessThan => return,
      LessThanEqual => return,
      Or => return,
      And => return,
      Xor => return,
      Equal => return,
      BitCount => return,
      BitScanForward => return,
      BitScanReverse => return,
      LShift => return,
      RShift => return,
      Jump => return,
      JumpRelativeOffset => return,
      JumpZero => return,
      JumpNonZero => return,
    },
    None => return, // TODO return an error.
  }
}
