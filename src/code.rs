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

const MAGIC_NUMBER: u32 = 0x02030741;

pub struct Compiler<'input> {
    src: &'input str,
    tag: u64,
    code_index: Vec<u8>,
    const_map: HashMap<String, Const>,
    field_map: HashMap<String, base::FieldSelector>,
    label_map: HashMap<String, u16>,
    type_map: HashMap<String, u16>,
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
            tag: 0, // FIXME
            code_index: Vec::new(),
            const_map: HashMap::new(),
            field_map: Self::new_field_map(),
            label_map: HashMap::new(),
            type_map: Self::new_type_map(),
        }
    }

    fn new_field_map() -> HashMap<String, base::FieldSelector> {
        let mut m = HashMap::new();
        m.insert("type".to_string(), base::FieldSelector::TYPE);
        m.insert("header".to_string(), base::FieldSelector::HEADER);
        m.insert("data".to_string(), base::FieldSelector::DATA);
        m
    }

    fn new_type_map() -> HashMap<String, u16> {
        let mut m = HashMap::new();
        m.insert("Empty".to_string(), 0);
        m
    }

    /// resolve labels, constants, and fields.
    fn build_code_index(&mut self, ns: &Vec<Node>) -> Result<(), Error<'input>> {
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

    fn write_u96<W: WriteBytesExt>(w: &mut W, x: &Const) -> Result<(), io::Error> {
        let (raw, sign) = match x {
            Const::Unsigned(x) => (x.0, 0),
            Const::Signed(x) => (x.0 as u128, 1 << 31),
        };
        w.write_u64::<BigEndian>(raw as u64)?;
        w.write_u32::<BigEndian>((raw >> 64) as u32 | sign)?;
        Ok(())
    }

    fn write_string<W: WriteBytesExt>(w: &mut W, x: &String) -> Result<(), Error<'input>> {
        let data = x.as_bytes();
        w.write_u8(data.len() as u8)?;
        w.write_all(data)?;
        Ok(())
    }

    fn write_metadata<W: WriteBytesExt>(w: &mut W, n: &Node) -> Result<(), Error<'input>> {
        let m = match n {
            Node::Metadata(m) => m,
            _ => return Err(Error::InternalUnexpectedNodeType),
        };
        w.write_u8(m.as_u8())?;
        match m {
            Metadata::Name(x) => Self::write_string(w, x),
            Metadata::Symbol(x) => Self::write_string(w, x),
            Metadata::Desc(x) => Self::write_string(w, x),
            Metadata::Author(x) => Self::write_string(w, x),
            Metadata::License(x) => Self::write_string(w, x),
            Metadata::Radius(x) => w.write_u8(*x).map_err(|x| x.into()),
            Metadata::BgColor(x) => Self::write_string(w, x),
            Metadata::FgColor(x) => Self::write_string(w, x),
            Metadata::Symmetries(x) => w.write_u8(x.bits() as u8).map_err(|x| x.into()),
            Metadata::Field(i, f) => {
                Self::write_string(w, i)?;
                w.write_u16::<BigEndian>(f.as_u16()).map_err(|x| x.into())
            }
            Metadata::Parameter(i, c) => {
                Self::write_string(w, i)?;
                Self::write_u96(w, c).map_err(|x| x.into())
            }
        }
    }

    fn write_instruction<W: WriteBytesExt>(
        &mut self,
        w: &mut W,
        n: &Node,
    ) -> Result<(), Error<'input>> {
        let i = match n {
            Node::Label(_) => return Ok(()),
            Node::Instruction(i) => i,
            _ => return Err(Error::InternalUnexpectedNodeType),
        };
        w.write_u8(i.as_u8())?;
        match i {
            Instruction::Nop => Ok(()),
            Instruction::Exit => Ok(()),
            Instruction::SwapSites => Ok(()),
            Instruction::SetSite => Ok(()),
            Instruction::SetField(x) => w.write_u16::<BigEndian>(self.field_map[x].as_u16()),
            Instruction::SetSiteField(x) => w.write_u16::<BigEndian>(self.field_map[x].as_u16()),
            Instruction::GetSite => Ok(()),
            Instruction::GetField(x) => w.write_u16::<BigEndian>(self.field_map[x].as_u16()),
            Instruction::GetSiteField(x) => w.write_u16::<BigEndian>(self.field_map[x].as_u16()),
            Instruction::GetType(x) => w.write_u16::<BigEndian>(self.type_map[x]),
            Instruction::Scan => Ok(()),
            Instruction::PushSymmetries(x) => w.write_u8(x.bits() as u8),
            Instruction::PopSymmetries => Ok(()),
            Instruction::Push(x) => Self::write_u96(w, x),
            Instruction::Pop => Ok(()),
            Instruction::Call(x) => w.write_u16::<BigEndian>(self.label_map[x]),
            Instruction::Ret => todo!(),
            Instruction::Checksum => todo!(),
            Instruction::Add => Ok(()),
            Instruction::Sub => Ok(()),
            Instruction::Neg => Ok(()),
            Instruction::Mod => Ok(()),
            Instruction::Mul => Ok(()),
            Instruction::Div => Ok(()),
            Instruction::Less => Ok(()),
            Instruction::LessEqual => Ok(()),
            Instruction::Or => Ok(()),
            Instruction::And => Ok(()),
            Instruction::Xor => Ok(()),
            Instruction::Equal => Ok(()),
            Instruction::BitCount => Ok(()),
            Instruction::BitScanForward => Ok(()),
            Instruction::BitScanReverse => Ok(()),
            Instruction::LShift => Ok(()),
            Instruction::RShift => Ok(()),
            Instruction::Jump(x) => w.write_u16::<BigEndian>(self.label_map[x]),
            Instruction::JumpRelativeOffset(x) => w.write_u16::<BigEndian>(self.label_map[x]),
            Instruction::JumpZero(x) => w.write_u16::<BigEndian>(self.label_map[x]),
            Instruction::JumpNonZero(x) => w.write_u16::<BigEndian>(self.label_map[x]),
        }
        .map_err(|x| x.into())
    }

    pub fn compile_to_writer<W: WriteBytesExt>(&mut self, w: &mut W) -> Result<(), Error<'input>> {
        let ast = substrate::FileParser::new().parse(self.src)?;

        w.write_u32::<BigEndian>(MAGIC_NUMBER)?;
        w.write_u16::<BigEndian>(Self::MINOR_VERSION)?;
        w.write_u16::<BigEndian>(Self::MAJOR_VERSION)?;
        w.write_u64::<BigEndian>(self.tag)?;

        self.build_code_index(&ast.body)?;

        w.write_u8(ast.header.len() as u8)?;
        for e in &ast.header {
            Self::write_metadata(w, e)?;
        }

        // TODO: Implement code table for recording typed arguments.
        // w.write_u16::<BigEndian>(self.code_index.len() as u16)?;
        // w.write_all(self.code_index.as_slice())?;

        w.write_u16::<BigEndian>(ast.body.len() as u16)?;
        for e in &ast.body {
            self.write_instruction(w, e)?;
        }

        Ok(())
    }
}
