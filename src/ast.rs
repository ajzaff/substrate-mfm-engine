use crate::base;
use std::vec;

#[derive(Clone, Debug)]
pub enum Node {
    Label(String),
    MetaInstruction(base::op::MetaOp, MetaArg),
    Instruction(base::op::Op, Args),
}

#[derive(Clone, Debug)]
pub enum Args {
    Null,
    Unary(Arg),
    Binary(Arg, Arg),
    Ternary(Arg, Arg, Arg),
}

#[repr(u8)]
#[derive(Clone, Debug)]
pub enum Arg {
    Label(String),
    SiteNumber(base::SiteNumber, Field),
    Symmetries(base::Symmetries),
    Register(Register, Field),
    ConstRef(String, Field),
    Type(String),
}

#[repr(u8)]
#[derive(Clone, Debug)]
pub enum MetaArg {
    Radius(u8),
    String(String),
    Symmetries(base::Symmetries),
    Parameter(String, base::Const),
    Field(String, base::FieldSelector),
}

#[derive(Clone, Debug)]
pub enum Field {
    Ref(String),
    Selector(base::FieldSelector),
}

#[derive(Copy, Clone, Debug)]
pub enum Register {
    Random,
    R(usize),
}

#[derive(Debug)]
pub struct File {
    pub header: Vec<Node>,
    pub body: Vec<Node>,
}
