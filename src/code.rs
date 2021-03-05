#[macro_use]
use lalrpop_util::lalrpop_mod;

use crate::ast::{Arg, Args, File, Node};
use crate::base::arith::{I96, U96};
use crate::base::op::{MetaOp, Op};
use crate::base::Const;
use byteorder::LittleEndian;
use byteorder::WriteBytesExt;
use lalrpop_util::lexer::Token;
use lalrpop_util::ParseError;
use std::collections::HashMap;
use std::fmt;
use std::io::BufWriter;
use std::io::Write;

lalrpop_mod!(pub substrate); // syntesized by LALRPOP

#[derive(Copy, Clone, Debug)]
pub enum Error {
    ParseError, // FIXME: include lalrpop ParseError
    UnexpectedNodeType,
    UnexpectedArgsCount,
    UnexpectedArgType,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ParseError => "parse error",
            Self::UnexpectedNodeType => "unexpected node type",
            Self::UnexpectedArgsCount => "unexpected args count",
            Self::UnexpectedArgType => "unexpected arg type",
        };
        write!(f, "{}", s)
    }
}

enum InlineOrPool {
    Inline(Const),
    Pool(u16),
}

const MAGIC_NUMBER: u32 = 0x41070302;

pub struct Compiler<'input> {
    src: &'input str,
    const_pool: Vec<Arg>,
    metadata: Vec<u64>,
    code: Vec<u64>,
    const_map: HashMap<&'input str, InlineOrPool>,
    field_map: HashMap<&'input str, u16>,
}

pub fn compile_to_bytes<'input>(src: &'input str) -> Result<Vec<u8>, Error> {
    let mut v = Vec::new();
    Compiler::new(src).compile_to_writer(&mut v).map(|_| v)
}

impl<'input> Compiler<'input> {
    const MINOR_VERSION: u16 = 1;
    const MAJOR_VERSION: u16 = 0;

    pub fn new(src: &'input str) -> Self {
        Self {
            src: src,
            const_pool: Vec::new(),
            metadata: Vec::new(),
            code: Vec::new(),
            const_map: HashMap::new(),
            field_map: HashMap::new(),
        }
    }

    fn process_metadata(&mut self, n: &Node) -> Result<(), Error> {
        let (op, args) = match n {
            Node::MetaInstruction(op, args) => (op, args),
            _ => return Err(Error::UnexpectedNodeType),
        };
        match op {
            MetaOp::Name
            | MetaOp::Symbol
            | MetaOp::Desc
            | MetaOp::Author
            | MetaOp::License
            | MetaOp::BgColor
            | MetaOp::FgColor
            | MetaOp::Radius
            | MetaOp::Symmetries
            | MetaOp::Field
            | MetaOp::Parameter => {
                match args {
                    Args::Unary(arg) => match arg {
                        Arg::String(_) => self.const_pool.push(arg.clone()), // TODO push instruction.
                        _ => return Err(Error::UnexpectedArgType),
                    },
                    _ => return Err(Error::UnexpectedArgsCount),
                }
            }
        }
        todo!()
    }

    fn process_instruction(&mut self, n: &Node) -> Result<(), Error> {
        todo!()
    }

    fn write_pool_entry<W: WriteBytesExt>(w: &mut W, a: Arg) {
        todo!()
    }

    pub fn compile_to_writer<W: WriteBytesExt>(&mut self, w: &mut W) -> Result<(), Error> {
        let p = substrate::FileParser::new()
            .parse(self.src)
            .map_err(|_| Error::ParseError); // FIXME: map useful error details
        if p.is_err() {
            return Err(p.unwrap_err());
        }
        let ast = p.unwrap();

        for e in ast.header {
            if let Err(v) = self.process_metadata(&e) {
                return Err(v);
            }
        }

        for e in &ast.body {
            if let Err(v) = self.process_instruction(e) {
                return Err(v);
            }
        }

        w.write_u32::<LittleEndian>(MAGIC_NUMBER);
        w.write_u16::<LittleEndian>(Self::MINOR_VERSION);
        w.write_u16::<LittleEndian>(Self::MAJOR_VERSION);

        w.write_u16::<LittleEndian>(self.const_pool.len() as u16);
        for a in &self.const_pool {
            Self::write_pool_entry(w, a.clone());
        }

        w.write_u8(self.metadata.len() as u8);
        for i in &self.metadata {
            w.write_u64::<LittleEndian>(*i);
        }

        w.write_u16::<LittleEndian>(self.code.len() as u16);
        for i in &self.code {
            w.write_u64::<LittleEndian>(*i);
        }

        Ok(())
    }
}
