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
  None,
  All,
}

#[derive(Debug)]
pub struct Program<'a> {
  pub instructions: &'a [Instruction],
}

#[derive(Debug)]
pub enum Instruction {
  Nop,
  Label(String),
  Window { op: WindowOp, dst: Site },
  Logical { op: LogicalOp, dst: OpRef },
  Control { op: ControlOp, dst: String },
}

pub type Site = u64;

pub fn site_pos(i: Site) -> Option<(u64, u64)> {
  None
}

pub fn site_grid_pos(i: Site, bounds: (u64, u64)) -> Option<(u64, u64)> {
  None
}

pub fn site_from_pos(i: Site) -> Option<Site> {
  None
}

pub fn site_from_grid_pos(i: Site, bounds: (u64, u64)) -> Option<Site> {
  None
}

#[derive(Copy, Clone, Debug)]
pub enum WindowOp {
  Select { op: SelectOp },
  Move,
  Copy,
  Swap,
  WindowTransform(Site, SelectOp),
}

#[derive(Copy, Clone, Debug)]
pub enum SelectOp {
  None,
  All,
  Any,
  ByElementId(u64),
  Window(Site, SelectOp),
  Invert(SelectOp),
}

pub enum WindowRef {
  Site(Site),
  SiteConst(Site, ConstOp),
}

#[derive(Clone, Debug)]
pub enum LogicalOp {
  Copy,
  Swap,
  RandOneIn(OpRef),
  Compare { lhs: OpRef, rhs: OpRef },
  Equal { lhs: OpRef, rhs: OpRef },
  LessThan { lhs: OpRef, rhs: OpRef },
  LessThanEqual { lhs: OpRef, rhs: OpRef },
  GreaterThan { lhs: OpRef, rhs: OpRef },
  GreaterThanEqual { lhs: OpRef, rhs: OpRef },
  RandBetween { lhs: OpRef, rhs: OpRef },
}

#[derive(Clone, Debug)]
pub enum ControlOp {
  Jump,
  JumpOffset(OpRef),
  CondLessThanZero(OpRef),
  CondGreaterThanZero(OpRef),
}

#[derive(Clone, Debug)]
pub enum OpRef {
  Null,
  ElementConst(Site, ConstOp),
  Const(ConstOp),
}

#[derive(Clone, Debug)]
pub enum ConstOp {
  C(u64),
  LShift(u8),
}

impl OpRef {
  pub fn is_const(self) -> bool {
    match self {
      OpRef::Const(_) => true,
      _ => false,
    }
  }
}

#[derive(Copy, Clone, Debug)]
pub struct Record<'a> {
  elem: &'a Element<'a>,
  value: u64,
  ip: u64,
}

impl Record<'_> {
  pub fn new<'a>(elem: &'a Element<'a>) -> Record<'a> {
    Record {
      elem: elem,
      value: 0,
      ip: 0,
    }
  }
}

#[derive(Debug)]
pub struct Grid<'a> {
  records: &'a mut [Option<Record<'a>>],
  bounds: (u16, u16),
}

impl Grid<'_> {
  pub fn new<'a>(records: &'a mut [Option<Record<'a>>], bounds: (u16, u16)) -> Grid {
    Grid {
      records: records,
      bounds: bounds,
    }
  }

  pub fn set_pos(x: i8, y: i8, e: Element) {}

  pub fn set_site(i: Site, e: Element) {}

  pub fn step() {}
}
