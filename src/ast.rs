use crate::base::arith::Const;
use crate::base::{FieldSelector, Symmetries};

#[derive(Copy, Clone, Debug)]
pub enum Node<'input> {
    Label(&'input str),
    Metadata(Metadata<'input>),
    Instruction(Instruction<'input>),
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Metadata<'input> {
    Name(&'input str),
    Symbol(&'input str),
    Desc(&'input str),
    Author(&'input str),
    License(&'input str),
    Radius(u8),
    BgColor(&'input str),
    FgColor(&'input str),
    Symmetries(Symmetries),
    Field(&'input str, FieldSelector),
    Parameter(&'input str, Const),
}

impl From<Metadata<'_>> for u8 {
    fn from(x: Metadata<'_>) -> u8 {
        match x {
            Metadata::Name(_) => 0,
            Metadata::Symbol(_) => 1,
            Metadata::Desc(_) => 2,
            Metadata::Author(_) => 3,
            Metadata::License(_) => 4,
            Metadata::Radius(_) => 5,
            Metadata::BgColor(_) => 6,
            Metadata::FgColor(_) => 7,
            Metadata::Symmetries(_) => 8,
            Metadata::Field(_, _) => 9,
            Metadata::Parameter(_, _) => 10,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Arg<T, U> {
    Ast(T),
    Runtime(U),
}

impl<T, U> Arg<T, U> {
    pub fn get_ast(&self) -> Option<&T> {
        if let Self::Ast(x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn get_runtime(&self) -> Option<&U> {
        if let Self::Runtime(x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn ast(&self) -> &T {
        self.get_ast().unwrap()
    }

    pub fn runtime(&self) -> &U {
        self.get_runtime().unwrap()
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Instruction<'input> {
    Nop,
    Exit,
    SwapSites,
    SetSite,
    SetField(Arg<&'input str, FieldSelector>),
    SetSiteField(Arg<&'input str, FieldSelector>),
    GetSite,
    GetField(Arg<&'input str, FieldSelector>),
    GetSiteField(Arg<&'input str, FieldSelector>),
    GetType(Arg<&'input str, u16>),
    GetParameter(Arg<&'input str, Const>),
    Scan,
    SaveSymmetries,
    UseSymmetries(Symmetries),
    RestoreSymmetries,
    Push0,
    Push1,
    Push2,
    Push3,
    Push4,
    Push5,
    Push6,
    Push7,
    Push8,
    Push9,
    Push10,
    Push11,
    Push12,
    Push13,
    Push14,
    Push15,
    Push16,
    Push17,
    Push18,
    Push19,
    Push20,
    Push21,
    Push22,
    Push23,
    Push24,
    Push25,
    Push26,
    Push27,
    Push28,
    Push29,
    Push30,
    Push31,
    Push32,
    Push33,
    Push34,
    Push35,
    Push36,
    Push37,
    Push38,
    Push39,
    Push40,
    Push(Const),
    Pop,
    Dup,
    Over,
    Swap,
    Rot,
    Call(Arg<&'input str, u16>),
    Ret,
    Checksum,
    Add,
    Sub,
    Neg,
    Mod,
    Mul,
    Div,
    Less,
    LessEqual,
    Or,
    And,
    Xor,
    Equal,
    BitCount,
    BitScanForward,
    BitScanReverse,
    LShift,
    RShift,
    Jump(Arg<&'input str, u16>),
    JumpRelativeOffset,
    JumpZero(Arg<&'input str, u16>),
    JumpNonZero(Arg<&'input str, u16>),
    SetPaint,
    GetPaint,
    Rand,
}

impl From<Instruction<'_>> for u8 {
    fn from(x: Instruction<'_>) -> u8 {
        match x {
            Instruction::Nop => 0,
            Instruction::Exit => 1,
            Instruction::SwapSites => 2,
            Instruction::SetSite => 3,
            Instruction::SetField(_) => 4,
            Instruction::SetSiteField(_) => 5,
            Instruction::GetSite => 6,
            Instruction::GetField(_) => 7,
            Instruction::GetSiteField(_) => 8,
            Instruction::GetType(_) => 9,
            Instruction::GetParameter(_) => 10,
            Instruction::Scan => 11,
            Instruction::SaveSymmetries => 12,
            Instruction::UseSymmetries(_) => 13,
            Instruction::RestoreSymmetries => 14,
            Instruction::Push0 => 15,
            Instruction::Push1 => 16,
            Instruction::Push2 => 17,
            Instruction::Push3 => 18,
            Instruction::Push4 => 19,
            Instruction::Push5 => 20,
            Instruction::Push6 => 21,
            Instruction::Push7 => 22,
            Instruction::Push8 => 23,
            Instruction::Push9 => 24,
            Instruction::Push10 => 25,
            Instruction::Push11 => 26,
            Instruction::Push12 => 27,
            Instruction::Push13 => 28,
            Instruction::Push14 => 29,
            Instruction::Push15 => 30,
            Instruction::Push16 => 31,
            Instruction::Push17 => 32,
            Instruction::Push18 => 33,
            Instruction::Push19 => 34,
            Instruction::Push20 => 35,
            Instruction::Push21 => 36,
            Instruction::Push22 => 37,
            Instruction::Push23 => 38,
            Instruction::Push24 => 39,
            Instruction::Push25 => 40,
            Instruction::Push26 => 41,
            Instruction::Push27 => 42,
            Instruction::Push28 => 43,
            Instruction::Push29 => 44,
            Instruction::Push30 => 45,
            Instruction::Push31 => 46,
            Instruction::Push32 => 47,
            Instruction::Push33 => 48,
            Instruction::Push34 => 49,
            Instruction::Push35 => 50,
            Instruction::Push36 => 51,
            Instruction::Push37 => 52,
            Instruction::Push38 => 53,
            Instruction::Push39 => 54,
            Instruction::Push40 => 55,
            Instruction::Push(_) => 56,
            Instruction::Pop => 57,
            Instruction::Dup => 58,
            Instruction::Over => 59,
            Instruction::Swap => 60,
            Instruction::Rot => 61,
            Instruction::Call(_) => 62,
            Instruction::Ret => 63,
            Instruction::Checksum => 64,
            Instruction::Add => 65,
            Instruction::Sub => 66,
            Instruction::Neg => 67,
            Instruction::Mod => 68,
            Instruction::Mul => 69,
            Instruction::Div => 70,
            Instruction::Less => 71,
            Instruction::LessEqual => 72,
            Instruction::Or => 73,
            Instruction::And => 74,
            Instruction::Xor => 75,
            Instruction::Equal => 76,
            Instruction::BitCount => 77,
            Instruction::BitScanForward => 78,
            Instruction::BitScanReverse => 79,
            Instruction::LShift => 80,
            Instruction::RShift => 81,
            Instruction::Jump(_) => 82,
            Instruction::JumpRelativeOffset => 83,
            Instruction::JumpZero(_) => 84,
            Instruction::JumpNonZero(_) => 85,
            Instruction::SetPaint => 86,
            Instruction::GetPaint => 87,
            Instruction::Rand => 88,
        }
    }
}

#[derive(Debug)]
pub struct File<'input> {
    pub header: Vec<Node<'input>>,
    pub body: Vec<Node<'input>>,
}
