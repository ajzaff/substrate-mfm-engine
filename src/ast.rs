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
    GetSignedField(Arg<&'input str, FieldSelector>),
    GetSignedSiteField(Arg<&'input str, FieldSelector>),
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
            Instruction::GetSignedField(_) => 9,
            Instruction::GetSignedSiteField(_) => 10,
            Instruction::GetType(_) => 11,
            Instruction::GetParameter(_) => 12,
            Instruction::Scan => 13,
            Instruction::SaveSymmetries => 14,
            Instruction::UseSymmetries(_) => 15,
            Instruction::RestoreSymmetries => 16,
            Instruction::Push0 => 17,
            Instruction::Push1 => 18,
            Instruction::Push2 => 19,
            Instruction::Push3 => 20,
            Instruction::Push4 => 21,
            Instruction::Push5 => 22,
            Instruction::Push6 => 23,
            Instruction::Push7 => 24,
            Instruction::Push8 => 25,
            Instruction::Push9 => 26,
            Instruction::Push10 => 27,
            Instruction::Push11 => 28,
            Instruction::Push12 => 29,
            Instruction::Push13 => 30,
            Instruction::Push14 => 31,
            Instruction::Push15 => 32,
            Instruction::Push16 => 33,
            Instruction::Push17 => 34,
            Instruction::Push18 => 35,
            Instruction::Push19 => 36,
            Instruction::Push20 => 37,
            Instruction::Push21 => 38,
            Instruction::Push22 => 39,
            Instruction::Push23 => 40,
            Instruction::Push24 => 41,
            Instruction::Push25 => 42,
            Instruction::Push26 => 43,
            Instruction::Push27 => 44,
            Instruction::Push28 => 45,
            Instruction::Push29 => 46,
            Instruction::Push30 => 47,
            Instruction::Push31 => 48,
            Instruction::Push32 => 49,
            Instruction::Push33 => 50,
            Instruction::Push34 => 51,
            Instruction::Push35 => 52,
            Instruction::Push36 => 53,
            Instruction::Push37 => 54,
            Instruction::Push38 => 55,
            Instruction::Push39 => 56,
            Instruction::Push40 => 57,
            Instruction::Push(_) => 58,
            Instruction::Pop => 59,
            Instruction::Dup => 60,
            Instruction::Over => 61,
            Instruction::Swap => 62,
            Instruction::Rot => 63,
            Instruction::Call(_) => 64,
            Instruction::Ret => 65,
            Instruction::Checksum => 66,
            Instruction::Add => 67,
            Instruction::Sub => 68,
            Instruction::Neg => 69,
            Instruction::Mod => 70,
            Instruction::Mul => 71,
            Instruction::Div => 72,
            Instruction::Less => 73,
            Instruction::LessEqual => 74,
            Instruction::Or => 75,
            Instruction::And => 76,
            Instruction::Xor => 77,
            Instruction::Equal => 78,
            Instruction::BitCount => 79,
            Instruction::BitScanForward => 80,
            Instruction::BitScanReverse => 81,
            Instruction::LShift => 82,
            Instruction::RShift => 83,
            Instruction::Jump(_) => 84,
            Instruction::JumpRelativeOffset => 85,
            Instruction::JumpZero(_) => 86,
            Instruction::JumpNonZero(_) => 87,
            Instruction::SetPaint => 88,
            Instruction::GetPaint => 89,
            Instruction::Rand => 90,
        }
    }
}

#[derive(Debug)]
pub struct File<'input> {
    pub header: Vec<Node<'input>>,
    pub body: Vec<Node<'input>>,
}
