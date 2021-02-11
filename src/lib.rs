#[derive(Debug)]
pub struct Element<'a> {
  pub id: u64,
  pub name: &'a str,
  pub symbol: &'a str,
  pub radius: u8,
  pub program: Program<'a>,
}

#[derive(Copy, Clone, Debug)]
pub enum Symmetries {
  R000L = 1, // Normal.
  R090L = 2,
  R180L = 4, // Flip_XY.
  R270L = 8,
  R000R = 16, // Flip_Y.
  R090R = 32,
  R180R = 64, // Flip_X.
  R270R = 128,
  ReflectX = 1 | 4,  // Normal | Flip_X.
  ReflectY = 1 | 16, // Normal | Flip_Y.
  All = 255,         // All rotations.
}

#[derive(Debug)]
pub struct Program<'a> {
  pub instructions: &'a [Instruction],
}

pub type Instruction = u64;

#[repr(u8)]
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

#[derive(Clone, Debug)]
pub enum Expr {
  Unsigned(i16),
  Signed(u16),
  RegisterField(RegisterExpr, FieldExpr),
  SiteField(u8, FieldExpr),
}

impl Expr {
  pub fn is_const(self) -> bool {
    match self {
      Self::Unsigned(_) => true,
      Self::Signed(_) => true,
      _ => false,
    }
  }
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
  R15,
  RUniformRandom,
}

#[derive(Copy, Clone, Debug)]
pub enum FieldExpr {
  Default,
  Named(u8),
  Inline(u8, u8),
}

impl FieldExpr {
  pub fn apply_option(self, x: u128) -> Option<u128> {
    match self {
      Self::Named(x) => None, // TODO: Implement field lookup.
      Self::Inline(n, p) => Some((x >> p) & ((1 << n) - 1)),
    }
  }
}

#[derive(Copy, Clone, Debug)]
pub struct Record<'a> {
  ev: &'a EventWindow<'a>,
  elem: &'a Element<'a>,
  registers: [u128; 16],
  symmetries: Symmetries,
  data: u128,
  ip: u64,
}

impl Record<'_> {
  pub fn new<'a>(ev: &'a EventWindow, elem: &'a Element<'a>) -> Record<'a> {
    Record {
      ev: ev,
      elem: elem,
      registers: [0; 16],
      symmetries: Symmetries::R000L, // Normal
      data: 0,
      ip: 0,
    }
  }

  pub fn deref_register_option(&self, op: RegisterExpr) -> Option<u128> {
    if op < RegisterExpr::RUniformRandom {
      return Some(self.registers[op as usize]);
    }
    if op == RegisterExpr::RUniformRandom {
      return Some(rand::random::<u128>());
    }
    None
  }

  pub fn deref_expr_option(&self, expr: Expr) -> Option<u128> {
    match expr {
      Zero => Some(0),
      One => Some(1),
      Expr::I96(x) => Some(x as u128),
      Expr::U96(x) => Some(x),
      Expr::Register(op) => self.deref_register_option(op),
      Expr::RegisterField(op, field) => {
        let x = self.deref_register_option(op);
        if x.is_some() {
          return field.apply_option(x.unwrap());
        }
        None
      }
      Expr::Site(x) => Some(x as u128),
      Expr::SiteField(x, field) => {
        let site = self.ev.get_site_option(x);
        if site.is_some() {
          return field.apply_option(site.unwrap().data);
        }
        None
      }
    }
  }
}

#[derive(Debug)]
pub struct EventWindow<'a> {
  radius: u8,
  origin: u64,
  grid: &'a Grid<'a>,
}

impl<'a> EventWindow<'a> {
  pub fn new_radius(radius: u8, origin: u64, grid: &'a Grid<'a>) -> EventWindow<'a> {
    EventWindow {
      radius: radius,
      origin: origin,
      grid: grid,
    }
  }
  pub fn new(origin: u64, grid: &'a Grid<'a>) -> EventWindow<'a> {
    Self::new_radius(4, origin, grid)
  }

  pub fn get_site_option(&self, x: Site) -> Option<&'a Record<'a>> {
    if x < 41 {
      // TODO: use self.radius
      return self.grid.records[x as usize];
    }
    None
  }
}

#[derive(Debug)]
pub struct Grid<'a> {
  records: &'a [Option<&'a Record<'a>>],
  bounds: (u16, u16),
}

impl<'a> Grid<'a> {
  pub fn new(records: &'a [Option<&'a Record<'a>>], bounds: (u16, u16)) -> Grid<'a> {
    Grid {
      records: records,
      bounds: bounds,
    }
  }

  pub fn is_valid(&self, i: usize) -> bool {
    i < self.records.len()
  }

  pub fn get_site_option(&self, i: usize, x: Site) -> Option<usize> {
    if self.is_valid(i) {}
    None
  }

  pub fn get_record_option(&self, i: usize) -> Option<&'a Record<'a>> {
    if self.is_valid(i) {
      return self.records[i];
    }
    None
  }
}
