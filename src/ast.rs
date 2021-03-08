use crate::base;

#[derive(Clone, Debug)]
pub enum Node {
    Label(String),
    Metadata(Metadata),
    Instruction(Instruction),
}

#[repr(u8)]
#[derive(Clone, Debug)]
pub enum Metadata {
    Name(String),
    Symbol(String),
    Desc(String),
    Author(String),
    License(String),
    Radius(u8),
    BgColor(String),
    FgColor(String),
    Symmetries(base::Symmetries),
    Field(String, base::FieldSelector),
    Parameter(String, base::Const),
}

impl Metadata {
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

#[repr(u8)]
#[derive(Clone, Debug)]
pub enum Instruction {
    Nop,
    Exit,
    SwapSites,
    SetSite,
    SetField(String),
    SetSiteField(String),
    GetSite,
    GetField(String),
    GetSiteField(String),
    GetType(String),
    Scan,
    PushSymmetries(base::Symmetries),
    PopSymmetries,
    Push(base::Const),
    Pop,
    Call(String),
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
    Jump(String),
    JumpRelativeOffset(String),
    JumpZero(String),
    JumpNonZero(String),
}

impl Instruction {
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
            Self::Scan => 10,
            Self::PushSymmetries(_) => 11,
            Self::PopSymmetries => 12,
            Self::Push(_) => 13,
            Self::Pop => 14,
            Self::Call(_) => 15,
            Self::Ret => 16,
            Self::Checksum => 17,
            Self::Add => 18,
            Self::Sub => 19,
            Self::Neg => 20,
            Self::Mod => 21,
            Self::Mul => 22,
            Self::Div => 23,
            Self::Less => 24,
            Self::LessEqual => 25,
            Self::Or => 26,
            Self::And => 27,
            Self::Xor => 28,
            Self::Equal => 29,
            Self::BitCount => 30,
            Self::BitScanForward => 31,
            Self::BitScanReverse => 32,
            Self::LShift => 33,
            Self::RShift => 34,
            Self::Jump(_) => 35,
            Self::JumpRelativeOffset(_) => 36,
            Self::JumpZero(_) => 37,
            Self::JumpNonZero(_) => 38,
        }
    }
}

#[derive(Debug)]
pub struct File {
    pub header: Vec<Node>,
    pub body: Vec<Node>,
}
