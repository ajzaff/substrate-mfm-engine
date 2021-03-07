#[macro_use]
use lalrpop_util::lalrpop_mod;

use crate::ast::{Instruction, Metadata, Node};
use crate::base;
use crate::base::Const;
use byteorder::BigEndian;
use byteorder::WriteBytesExt;
use std::collections::HashMap;
use std::fmt;
use std::io;

lalrpop_mod!(pub substrate); // syntesized by LALRPOP

#[derive(Copy, Clone, Debug)]
pub enum Error {
    IOError,
    ParseError, // FIXME: include lalrpop ParseError
    InternalError,
    InternalUnexpectedNodeType,
    InternalUnexpectedArgsCount,
    InternalUnexpectedArgType,
}

impl From<io::Error> for Error {
    fn from(x: io::Error) -> Self {
        Error::IOError
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::InternalError => "internal error",
            Self::IOError => "IO error",
            Self::ParseError => "parse error",
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
    const_pool: Vec<PoolEntry>,
    metadata: Vec<u64>,
    code: Vec<u64>,
    const_map: HashMap<String, Const>,
    field_map: HashMap<String, base::FieldSelector>,
    label_map: HashMap<String, usize>,
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
            label_map: HashMap::new(),
        }
    }

    /// See instruction layout in LAYOUT.md
    fn metadata_instruction(op: Metadata) -> u64 {
        todo!()
    }

    fn process_metadata(&mut self, n: &Node) -> Result<(), Error> {
        match n {
            Node::Metadata(_) => (),
            _ => return Err(Error::InternalUnexpectedNodeType),
        };
        todo!()
    }

    fn process_labels(&mut self, ns: &Vec<Node>) -> Result<(), Error> {
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

    fn process_instruction(&mut self, n: &Node) -> Result<(), Error> {
        match n {
            Node::Instruction(i) => (),
            _ => return Err(Error::InternalUnexpectedNodeType),
        };
        todo!()
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

    fn write_pool_string<W: WriteBytesExt>(w: &mut W, x: String) -> Result<(), Error> {
        w.write_u8(PoolEntry::String as u8)?;
        let data = x.as_bytes();
        w.write_u16::<BigEndian>(data.len() as u16)?;
        w.write_all(data)?;
        Ok(())
    }

    fn write_pool_entry<W: WriteBytesExt>(w: &mut W, e: PoolEntry) -> Result<(), Error> {
        match e {
            PoolEntry::String(x) => Self::write_pool_string(w, x),
            _ => Err(Error::InternalError),
        }
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
        self.process_labels(&ast.body)?;
        for e in &ast.body {
            if let Err(v) = self.process_instruction(e) {
                return Err(v);
            }
        }

        w.write_u32::<BigEndian>(MAGIC_NUMBER)?;
        w.write_u16::<BigEndian>(Self::MINOR_VERSION)?;
        w.write_u16::<BigEndian>(Self::MAJOR_VERSION)?;

        w.write_u16::<BigEndian>(self.const_pool.len() as u16)?;
        for a in &self.const_pool {
            Self::write_pool_entry(w, a.clone())?
        }

        w.write_u8(self.metadata.len() as u8)?;
        for i in &self.metadata {
            w.write_u64::<BigEndian>(*i)?;
        }

        w.write_u16::<BigEndian>(self.code.len() as u16)?;
        for i in &self.code {
            w.write_u64::<BigEndian>(*i)?;
        }

        Ok(())
    }
}
