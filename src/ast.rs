use crate::base;
use crate::base::op::{MetaOp, Op};
use crate::base::{Const, SiteNumber, Symmetries};

#[derive(Debug)]
pub enum Node {
    Label(String),
    MetaInstruction(MetaOp, Args),
    Instruction(Op, Args),
}

#[derive(Debug)]
pub enum Args {
    Null,
    Unary(Arg),
    Binary(Arg, Arg),
    Ternary(Arg, Arg, Arg),
}

#[derive(Debug)]
pub enum Arg {
    U8(u8),
    String(String),
    Label(String),
    SiteNumber(SiteNumber, Field),
    Symmetries(Symmetries),
    Register(Register, Field),
    Const(Const, Field),
    ConstRef(String, Field),
    Type(String),
}

#[derive(Debug)]
pub enum Field {
    Ref(String),
    Field(base::Field),
    Selector(base::FieldSelector),
}

#[derive(Debug)]
pub enum Register {
    Random,
    R(usize),
}

#[derive(Debug)]
pub struct File {
    pub header: Vec<Node>,
    pub body: Vec<Node>,
}
