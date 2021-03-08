#[macro_use]
use lalrpop_util::lalrpop_mod;

use crate::ast::{Instruction, Metadata, Node};
use crate::base;
use crate::base::Const;
use byteorder::BigEndian;
use byteorder::WriteBytesExt;
use lalrpop_util;
use std::collections::HashMap;
use std::fmt;
use std::io;

lalrpop_mod!(pub substrate); // syntesized by LALRPOP

#[derive(Clone, Debug)]
pub enum Error<'input> {
    IOError,
    ParseError(lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'input>, &'input str>),
    InternalError,
    InternalUnexpectedNodeType,
    InternalUnexpectedArgsCount,
    InternalUnexpectedArgType,
}

impl From<io::Error> for Error<'_> {
    fn from(x: io::Error) -> Self {
        Error::IOError
    }
}

impl<'input> From<lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'input>, &'input str>>
    for Error<'input>
{
    fn from(
        x: lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'input>, &'input str>,
    ) -> Self {
        Error::ParseError(x)
    }
}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::IOError => "IO error",
            Self::ParseError(x) => return (*x).fmt(f),
            Self::InternalError => "internal error",
            Self::InternalUnexpectedNodeType => "internal: unexpected node type",
            Self::InternalUnexpectedArgsCount => "internal: unexpected args count",
            Self::InternalUnexpectedArgType => "internal: unexpected arg type",
        };
        write!(f, "{}", s)
    }
}

#[repr(u8)]
#[derive(Clone, Debug)]
enum PoolEntry {
    String(String),
}

const MAGIC_NUMBER: u32 = 0x02030741;

pub struct Compiler<'input> {
    src: &'input str,
    code: Vec<u64>,
    const_map: HashMap<String, Const>,
    field_map: HashMap<String, base::FieldSelector>,
    label_map: HashMap<String, usize>,
}

pub fn compile_to_bytes<'input>(src: &'input str) -> Result<Vec<u8>, Error<'input>> {
    let mut v = Vec::new();
    Compiler::new(src).compile_to_writer(&mut v).map(|_| v)
}

impl<'input> Compiler<'input> {
    const MINOR_VERSION: u16 = 1;
    const MAJOR_VERSION: u16 = 0;

    pub fn new(src: &'input str) -> Self {
        Self {
            src: src,
            code: Vec::new(),
            const_map: HashMap::new(),
            field_map: HashMap::new(),
            label_map: HashMap::new(),
        }
    }

    fn process_labels(&mut self, ns: &Vec<Node>) -> Result<(), Error<'input>> {
        let mut ln = 0;
        for n in ns {
            match n {
                Node::Label(x) => {
                    self.label_map.insert(x.to_owned(), ln + 1);
                }
                Node::Instruction(_) => ln += 1,
                _ => return Err(Error::InternalUnexpectedNodeType),
            };
        }
        Ok(())
    }

    fn process_instruction(&mut self, n: &Node) -> Result<(), Error<'input>> {
        let i = match n {
            Node::Instruction(i) => i,
            _ => return Err(Error::InternalUnexpectedNodeType),
        };
        match i {
            Instruction::Nop => todo!(),
            Instruction::Exit => todo!(),
            Instruction::SwapSites => todo!(),
            Instruction::SetSite => todo!(),
            Instruction::SetField(_) => todo!(),
            Instruction::SetSiteField(_) => todo!(),
            Instruction::GetSite => todo!(),
            Instruction::GetField(_) => todo!(),
            Instruction::GetSiteField(_) => todo!(),
            Instruction::GetType(_) => todo!(),
            Instruction::Scan => todo!(),
            Instruction::PushSymmetries(_) => todo!(),
            Instruction::PopSymmetries => todo!(),
            Instruction::Push(_) => todo!(),
            Instruction::Pop => todo!(),
            Instruction::Call(_) => todo!(),
            Instruction::Ret => todo!(),
            Instruction::Checksum => todo!(),
            Instruction::Add => todo!(),
            Instruction::Sub => todo!(),
            Instruction::Neg => todo!(),
            Instruction::Mod => todo!(),
            Instruction::Mul => todo!(),
            Instruction::Div => todo!(),
            Instruction::Less => todo!(),
            Instruction::LessEqual => todo!(),
            Instruction::Or => todo!(),
            Instruction::And => todo!(),
            Instruction::Xor => todo!(),
            Instruction::Equal => todo!(),
            Instruction::BitCount => todo!(),
            Instruction::BitScanForward => todo!(),
            Instruction::BitScanReverse => todo!(),
            Instruction::LShift => todo!(),
            Instruction::RShift => todo!(),
            Instruction::Jump(_) => todo!(),
            Instruction::JumpRelativeOffset(_) => todo!(),
            Instruction::JumpZero(_) => todo!(),
            Instruction::JumpNonZero(_) => todo!(),
        }
    }

    fn write_u96<W: WriteBytesExt>(w: &mut W, x: Const) -> Result<(), io::Error> {
        let (raw, sign) = match x {
            Const::Unsigned(x) => (x.0, 0),
            Const::Signed(x) => (x.0 as u128, 1 << 31),
        };
        w.write_u64::<BigEndian>(raw as u64)?;
        w.write_u32::<BigEndian>((raw >> 64) as u32 | sign)?;
        Ok(())
    }

    fn write_pool_string<W: WriteBytesExt>(w: &mut W, x: String) -> Result<(), Error<'input>> {
        w.write_u8(PoolEntry::String as u8)?;
        let data = x.as_bytes();
        w.write_u16::<BigEndian>(data.len() as u16)?;
        w.write_all(data)?;
        Ok(())
    }

    fn write_metadata<'a: 'input, W: WriteBytesExt>(
        w: &mut W,
        e: &Node,
    ) -> Result<(), Error<'input>> {
        todo!()
    }

    pub fn compile_to_writer<W: WriteBytesExt>(&mut self, w: &mut W) -> Result<(), Error<'input>> {
        let ast = substrate::FileParser::new().parse(self.src)?;

        self.process_labels(&ast.body)?;
        for e in &ast.body {
            if let Err(v) = self.process_instruction(e) {
                return Err(v);
            }
        }

        w.write_u32::<BigEndian>(MAGIC_NUMBER)?;
        w.write_u16::<BigEndian>(Self::MINOR_VERSION)?;
        w.write_u16::<BigEndian>(Self::MAJOR_VERSION)?;

        w.write_u8(ast.header.len() as u8)?;
        for e in &ast.header {
            Self::write_metadata(w, e)?;
        }

        w.write_u16::<BigEndian>(self.code.len() as u16)?;
        for i in &self.code {
            w.write_u64::<BigEndian>(*i)?;
        }

        Ok(())
    }
}
