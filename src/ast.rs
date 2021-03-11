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

impl Metadata<'_> {
    pub const MAX: u8 = 10;

    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Name(_) => 0,
            Self::Symbol(_) => 1,
            Self::Desc(_) => 2,
            Self::Author(_) => 3,
            Self::License(_) => 4,
            Self::Radius(_) => 5,
            Self::BgColor(_) => 6,
            Self::FgColor(_) => 7,
            Self::Symmetries(_) => 8,
            Self::Field(_, _) => 9,
            Self::Parameter(_, _) => 10,
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
}

impl Instruction<'_> {
    pub const MAX: u8 = 85;

    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Nop => 0,
            Self::Exit => 1,
            Self::SwapSites => 2,
            Self::SetSite => 3,
            Self::SetField(_) => 4,
            Self::SetSiteField(_) => 5,
            Self::GetSite => 6,
            Self::GetField(_) => 7,
            Self::GetSiteField(_) => 8,
            Self::GetType(_) => 9,
            Self::GetParameter(_) => 10,
            Self::Scan => 11,
            Self::SaveSymmetries => 12,
            Self::UseSymmetries(_) => 13,
            Self::RestoreSymmetries => 14,
            Self::Push0 => 15,
            Self::Push1 => 16,
            Self::Push2 => 17,
            Self::Push3 => 18,
            Self::Push4 => 19,
            Self::Push5 => 20,
            Self::Push6 => 21,
            Self::Push7 => 22,
            Self::Push8 => 23,
            Self::Push9 => 24,
            Self::Push10 => 25,
            Self::Push11 => 26,
            Self::Push12 => 27,
            Self::Push13 => 28,
            Self::Push14 => 29,
            Self::Push15 => 30,
            Self::Push16 => 31,
            Self::Push17 => 32,
            Self::Push18 => 33,
            Self::Push19 => 34,
            Self::Push20 => 35,
            Self::Push21 => 36,
            Self::Push22 => 37,
            Self::Push23 => 38,
            Self::Push24 => 39,
            Self::Push25 => 40,
            Self::Push26 => 41,
            Self::Push27 => 42,
            Self::Push28 => 43,
            Self::Push29 => 44,
            Self::Push30 => 45,
            Self::Push31 => 46,
            Self::Push32 => 47,
            Self::Push33 => 48,
            Self::Push34 => 49,
            Self::Push35 => 50,
            Self::Push36 => 51,
            Self::Push37 => 52,
            Self::Push38 => 53,
            Self::Push39 => 54,
            Self::Push40 => 55,
            Self::Push(_) => 56,
            Self::Pop => 57,
            Self::Dup => 58,
            Self::Over => 59,
            Self::Swap => 60,
            Self::Rot => 61,
            Self::Call(_) => 62,
            Self::Ret => 63,
            Self::Checksum => 64,
            Self::Add => 65,
            Self::Sub => 66,
            Self::Neg => 67,
            Self::Mod => 68,
            Self::Mul => 69,
            Self::Div => 70,
            Self::Less => 71,
            Self::LessEqual => 72,
            Self::Or => 73,
            Self::And => 74,
            Self::Xor => 75,
            Self::Equal => 76,
            Self::BitCount => 77,
            Self::BitScanForward => 78,
            Self::BitScanReverse => 79,
            Self::LShift => 80,
            Self::RShift => 81,
            Self::Jump(_) => 82,
            Self::JumpRelativeOffset => 83,
            Self::JumpZero(_) => 84,
            Self::JumpNonZero(_) => 85,
        }
    }
}

#[derive(Debug)]
pub struct File<'input> {
    pub header: Vec<Node<'input>>,
    pub body: Vec<Node<'input>>,
}
