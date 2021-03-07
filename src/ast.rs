use crate::base;

#[derive(Clone, Debug)]
pub enum Node {
    Label(String),
    Metadata(Metadata),
    Instruction(Instruction),
}

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

#[derive(Clone, Debug)]
pub enum Instruction {
    Nop,
    Exit,
    SetSite,
    SetRegister,
    GetSite,
    GetRegister,
    GetField(String),
    SetField(String),
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

#[derive(Debug)]
pub struct File {
    pub header: Vec<Node>,
    pub body: Vec<Node>,
}
