use crate::ast::{Instruction, Metadata, Node};
use crate::base;
use crate::base::arith::Const;
use byteorder::BigEndian;
use byteorder::WriteBytesExt;
use lalrpop_util;
use lalrpop_util::lalrpop_mod;
use std::collections::HashMap;
use std::fmt;
use std::io;

lalrpop_mod!(pub substrate); // syntesized by LALRPOP

#[derive(Clone, Debug, PartialEq)]
pub enum Error<'input> {
    IOError,
    ParseError(lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'input>, &'input str>),
    InternalError,
    InternalUnexpectedNodeType,
    InternalUnexpectedArgsCount,
    InternalUnexpectedArgType,
}

impl From<io::Error> for Error<'_> {
    fn from(_: io::Error) -> Self {
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
            Self::ParseError(x) => return x.fmt(f),
            Self::InternalError => "internal error",
            Self::InternalUnexpectedNodeType => "internal: unexpected node type",
            Self::InternalUnexpectedArgsCount => "internal: unexpected args count",
            Self::InternalUnexpectedArgType => "internal: unexpected arg type",
        };
        write!(f, "{}", s)
    }
}

struct CodeEntry {
    args: Vec<u8>,
}

impl CodeEntry {
    fn new() -> Self {
        Self { args: Vec::new() }
    }
}

const MAGIC_NUMBER: u32 = 0x02030741;

pub struct Compiler {
    build_tag: String,
    type_map: HashMap<String, u16>,
}

impl Compiler {
    const MINOR_VERSION: u16 = 1;
    const MAJOR_VERSION: u16 = 0;

    pub fn new(build_tag: &str) -> Self {
        Self {
            build_tag: build_tag.to_owned(),
            type_map: Self::new_type_map(),
        }
    }

    fn new_type_map() -> HashMap<String, u16> {
        let mut m = HashMap::new();
        m.insert("Empty".to_owned(), 0);
        m
    }

    fn new_field_map() -> HashMap<&'static str, base::FieldSelector> {
        let mut m = HashMap::new();
        m.insert("type", base::FieldSelector::TYPE);
        m.insert("header", base::FieldSelector::HEADER);
        m.insert("data", base::FieldSelector::DATA);
        m
    }

    fn index_metadata_node<'input>(
        n: Node<'input>,
        const_map: &mut HashMap<&'input str, Const>,
        field_map: &mut HashMap<&'input str, base::FieldSelector>,
    ) -> Result<(), Error<'input>> {
        match n {
            Node::Metadata(i) => match i {
                Metadata::Parameter(i, c) => {
                    const_map.insert(i, c);
                }
                Metadata::Field(i, f) => {
                    field_map.insert(i, f);
                }
                _ => {}
            },
            _ => return Err(Error::InternalUnexpectedNodeType),
        }
        Ok(())
    }

    fn index_code_node<'input>(
        ln: &mut u16,
        n: Node<'input>,
        code_index: &mut HashMap<u16, CodeEntry>,
        label_map: &mut HashMap<&'input str, u16>,
    ) -> Result<(), Error<'input>> {
        match n {
            Node::Label(x) => {
                label_map.insert(x, *ln + 1);
            }
            Node::Instruction(i) => *ln += 1,
            _ => return Err(Error::InternalUnexpectedNodeType),
        }
        Ok(())
    }

    fn write_u96<W: WriteBytesExt>(w: &mut W, x: Const) -> Result<(), io::Error> {
        let v = x.as_u128();
        w.write_u32::<BigEndian>((v >> 64) as u32)?;
        w.write_u64::<BigEndian>(v as u64)
    }

    fn write_string<'input, W: WriteBytesExt>(
        w: &mut W,
        x: &'input str,
    ) -> Result<(), Error<'input>> {
        let data = x.as_bytes();
        w.write_u8(data.len() as u8)?;
        w.write_all(data)?;
        Ok(())
    }

    fn write_metadata<'input, W: WriteBytesExt>(
        w: &mut W,
        n: Node<'input>,
        const_map: &HashMap<&'input str, Const>,
        field_map: &HashMap<&'input str, base::FieldSelector>,
    ) -> Result<(), Error<'input>> {
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
            Metadata::Radius(x) => w.write_u8(x).map_err(|x| x.into()),
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

    fn write_code_index<'input, W: WriteBytesExt>(
        w: &mut W,
        code_index: &HashMap<u16, CodeEntry>,
    ) -> Result<(), Error<'input>> {
        todo!()
    }

    fn write_instruction<'input, W: WriteBytesExt>(
        w: &mut W,
        n: Node<'input>,
        ln: &mut u16,
        type_map: &HashMap<String, u16>,
        label_map: &HashMap<&'input str, u16>,
        const_map: &HashMap<&'input str, Const>,
        field_map: &HashMap<&'input str, base::FieldSelector>,
    ) -> Result<(), Error<'input>> {
        let i = match n {
            Node::Label(_) => return Ok(()),
            Node::Instruction(i) => i,
            _ => return Err(Error::InternalUnexpectedNodeType),
        };
        *ln += 1;
        w.write_u8(i.as_u8())?;
        match i {
            Instruction::Nop => Ok(()),
            Instruction::Exit => Ok(()),
            Instruction::SwapSites => Ok(()),
            Instruction::SetSite => Ok(()),
            Instruction::SetField(x) => w.write_u16::<BigEndian>(field_map[x.ast()].as_u16()),
            Instruction::SetSiteField(x) => w.write_u16::<BigEndian>(field_map[x.ast()].as_u16()),
            Instruction::GetSite => Ok(()),
            Instruction::GetField(x) => w.write_u16::<BigEndian>(field_map[x.ast()].as_u16()),
            Instruction::GetSiteField(x) => w.write_u16::<BigEndian>(field_map[x.ast()].as_u16()),
            Instruction::GetType(x) => w.write_u16::<BigEndian>(type_map[x.ast().to_owned()]),
            Instruction::GetParameter(x) => Self::write_u96(w, const_map[x.ast()]),
            Instruction::Scan => Ok(()),
            Instruction::SaveSymmetries => Ok(()),
            Instruction::UseSymmetries(x) => w.write_u8(x.bits() as u8),
            Instruction::RestoreSymmetries => Ok(()),
            Instruction::Push0
            | Instruction::Push1
            | Instruction::Push2
            | Instruction::Push3
            | Instruction::Push4
            | Instruction::Push5
            | Instruction::Push6
            | Instruction::Push7
            | Instruction::Push8
            | Instruction::Push9
            | Instruction::Push10
            | Instruction::Push11
            | Instruction::Push12
            | Instruction::Push13
            | Instruction::Push14
            | Instruction::Push15
            | Instruction::Push16
            | Instruction::Push17
            | Instruction::Push18
            | Instruction::Push19
            | Instruction::Push20
            | Instruction::Push21
            | Instruction::Push22
            | Instruction::Push23
            | Instruction::Push24
            | Instruction::Push25
            | Instruction::Push26
            | Instruction::Push27
            | Instruction::Push28
            | Instruction::Push29
            | Instruction::Push30
            | Instruction::Push31
            | Instruction::Push32
            | Instruction::Push33
            | Instruction::Push34
            | Instruction::Push35
            | Instruction::Push36
            | Instruction::Push37
            | Instruction::Push38
            | Instruction::Push39
            | Instruction::Push40 => Ok(()),
            Instruction::Push(x) => Self::write_u96(w, x),
            Instruction::Pop | Instruction::Dup | Instruction::Over | Instruction::Swap => Ok(()),
            Instruction::Rot => Ok(()),
            Instruction::Call(x) => w.write_u16::<BigEndian>(label_map[x.ast()]),
            Instruction::Ret => Ok(()),
            Instruction::Checksum => Ok(()),
            Instruction::Add
            | Instruction::Sub
            | Instruction::Neg
            | Instruction::Mod
            | Instruction::Mul
            | Instruction::Div
            | Instruction::Less
            | Instruction::LessEqual
            | Instruction::Or
            | Instruction::And
            | Instruction::Xor
            | Instruction::Equal
            | Instruction::BitCount
            | Instruction::BitScanForward
            | Instruction::BitScanReverse
            | Instruction::LShift
            | Instruction::RShift => Ok(()),
            Instruction::Jump(x) => w.write_u16::<BigEndian>(label_map[x.ast()]),
            Instruction::JumpRelativeOffset => Ok(()),
            Instruction::JumpZero(x) => w.write_u16::<BigEndian>(label_map[x.ast()]),
            Instruction::JumpNonZero(x) => w.write_u16::<BigEndian>(label_map[x.ast()]),
        }
        .map_err(|x| x.into())
    }

    pub fn compile_to_writer<'input, W: WriteBytesExt>(
        &'input mut self,
        w: &mut W,
        src: &'input str,
    ) -> Result<(), Error<'input>> {
        let ast = substrate::FileParser::new().parse(src)?;
        let mut code_index: HashMap<u16, CodeEntry> = HashMap::new();
        let mut label_map: HashMap<&'input str, u16> = HashMap::new();
        let mut const_map: HashMap<&'input str, Const> = HashMap::new();
        let mut field_map: HashMap<&'input str, base::FieldSelector> = Self::new_field_map();

        for n in ast.header.iter() {
            Self::index_metadata_node(*n, &mut const_map, &mut field_map)?;
        }

        let mut ln = 0u16;
        for n in ast.body.iter() {
            Self::index_code_node(&mut ln, *n, &mut code_index, &mut label_map)?;
        }

        w.write_u32::<BigEndian>(MAGIC_NUMBER)?;
        w.write_u16::<BigEndian>(Self::MINOR_VERSION)?;
        w.write_u16::<BigEndian>(Self::MAJOR_VERSION)?;
        Self::write_string(w, self.build_tag.as_str())?;

        w.write_u8(ast.header.len() as u8)?;
        for e in ast.header.iter() {
            Self::write_metadata(w, *e, &const_map, &field_map)?;
        }

        w.write_u16::<BigEndian>(code_index.len() as u16)?;
        // Self::write_code_index(w, &code_index)?;

        w.write_u16::<BigEndian>(ln)?;
        ln = 0;
        for e in ast.body.iter() {
            Self::write_instruction(
                w,
                *e,
                &mut ln,
                &self.type_map,
                &label_map,
                &const_map,
                &field_map,
            )?;
        }

        Ok(())
    }
}
