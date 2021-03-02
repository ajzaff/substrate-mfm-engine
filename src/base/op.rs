use std::str::FromStr;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum MetaOp {
    Name,
    Symbol,
    Desc,
    Author,
    License,
    Radius,
    BgColor,
    FgColor,
    Symmetries,
    Field,
    Parameter,
}

impl FromStr for MetaOp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ".name" => Ok(Self::Name),
            ".symbol" => Ok(Self::Symbol),
            ".desc" => Ok(Self::Desc),
            ".author" => Ok(Self::Author),
            ".license" => Ok(Self::License),
            ".radius" => Ok(Self::Radius),
            ".bgcolor" => Ok(Self::BgColor),
            ".fgcolor" => Ok(Self::FgColor),
            ".symmetries" => Ok(Self::Symmetries),
            ".field" => Ok(Self::Field),
            ".parameter" => Ok(Self::Parameter),
            _ => Err(()),
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Op {
    Nop,
    Exit,
    Copy,
    Swap,
    Scan,
    UseSymmetries,
    RestoreSymmetries,
    Push,
    Pop,
    Call,
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
    Jump,
    JumpRelativeOffset,
    JumpZero,
    JumpNonZero,
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nop" => Ok(Self::Nop),
            "exit" => Ok(Self::Exit),
            "copy" => Ok(Self::Copy),
            "swap" => Ok(Self::Swap),
            "scan" => Ok(Self::Scan),
            "usesymmetries" => Ok(Self::UseSymmetries),
            "restoresymmetries" => Ok(Self::RestoreSymmetries),
            "push" => Ok(Self::Push),
            "pop" => Ok(Self::Push),
            "call" => Ok(Self::Push),
            "ret" => Ok(Self::Push),
            "checksum" => Ok(Self::Checksum),
            "add" => Ok(Self::Add),
            "sub" => Ok(Self::Sub),
            "neg" => Ok(Self::Neg),
            "mod" => Ok(Self::Mod),
            "mul" => Ok(Self::Mul),
            "div" => Ok(Self::Div),
            "less" => Ok(Self::Less),
            "lessequal" => Ok(Self::LessEqual),
            "or" => Ok(Self::Or),
            "and" => Ok(Self::And),
            "xor" => Ok(Self::Xor),
            "equal" => Ok(Self::Equal),
            "bitcount" => Ok(Self::BitCount),
            "bitscanforward" => Ok(Self::BitScanForward),
            "bitscanreverse" => Ok(Self::BitScanReverse),
            "lshift" => Ok(Self::LShift),
            "rshift" => Ok(Self::RShift),
            "jump" => Ok(Self::Jump),
            "jumprelativeoffset" => Ok(Self::JumpRelativeOffset),
            "jumpzero" => Ok(Self::JumpZero),
            "jumpnonzero" => Ok(Self::JumpNonZero),
            _ => Err(()),
        }
    }
}
