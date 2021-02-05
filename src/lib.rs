use std::ops;

#[derive(Debug)]
pub struct Element<'a> {
  pub name: String,
  pub symbol: String,
  pub radius: u8,
  pub program: Program<'a>,
}

#[derive(Copy, Clone, Debug)]
pub enum Symmetries {
  NONE,
  ALL,
}

#[derive(Debug)]
pub struct Program<'a> {
  pub instructions: &'a [Instruction],
}

#[derive(Debug)]
pub enum Instruction {
  Nop,
  Label(String),
  Window {
    op: WindowOp,
    dst: u64,
    src: u64,
  },
  Unary {
    op: UnaryOp,
    dst: OpRef,
    src: OpRef,
  },
  Binary {
    op: BinaryOp,
    dst: OpRef,
    lhs: OpRef,
    rhs: OpRef,
  },
  Control {
    op: ControlOp,
    dst: String,
    src: OpRef,
  },
}

#[derive(Copy, Clone, Debug)]
pub enum WindowOp {
  Move,
  Copy,
  Swap,
}

#[derive(Copy, Clone, Debug)]
pub enum UnaryOp {
  Copy,
  Swap,
  RandOneIn,
}

#[derive(Copy, Clone, Debug)]
pub enum BinaryOp {
  Copy,
  Swap,
  Compare,
  Equal,
  LessThan,
  LessThanEqual,
  GreaterThan,
  GreaterThanEqual,
  RandBetween,
}

#[derive(Copy, Clone, Debug)]
pub enum ControlOp {
  Jump,
  CondLessThanZero,
  CondGreaterThanZero,
}

#[derive(Copy, Clone, Debug)]
pub enum OpRef {
  None,
  Site(u64),
  SiteAndMask(u64, u64),
  Const(u64),
  ConstAndShift(u64, u8),
  ConstAndMaskedShift(u64, u64, u8),
}

impl OpRef {
  pub fn is_const(self) -> bool {
    match self {
      OpRef::Const(_) => true,
      OpRef::ConstAndShift(_, _) => true,
      OpRef::ConstAndMaskedShift(_, _, _) => true,
      _ => false,
    }
  }
}
