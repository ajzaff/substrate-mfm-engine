#[macro_use]
use lalrpop_util::lalrpop_mod;

use crate::ast::{Arg, Args, Field, File, MetaArg, Node};
use crate::base;
use crate::base::arith::{I96, U96};
use crate::base::op::{MetaOp, Op};
use crate::base::Const;
use byteorder::LittleEndian;
use byteorder::WriteBytesExt;
use lalrpop_util::lexer::Token;
use lalrpop_util::ParseError;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::BufWriter;
use std::io::Write;

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

const MAGIC_NUMBER: u32 = 0x02030741;

pub struct Compiler<'input> {
    src: &'input str,
    const_pool: Vec<MetaArg>,
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
    fn metadata_instruction(op: MetaOp, a1: u16, a2: Option<u16>, a3: Option<u16>) -> u64 {
        (op as u64) << 48
            | (a1 as u64) << 32
            | (a2.unwrap_or_default() as u64) << 16
            | a3.unwrap_or_default() as u64
    }

    fn process_metadata(&mut self, n: &Node) -> Result<(), Error> {
        let (op, arg) = match n {
            Node::MetaInstruction(op, arg) => (op, arg),
            _ => return Err(Error::InternalUnexpectedNodeType),
        };
        let n = self.const_pool.len();
        match op {
            MetaOp::Name
            | MetaOp::Symbol
            | MetaOp::Desc
            | MetaOp::Author
            | MetaOp::License
            | MetaOp::BgColor
            | MetaOp::FgColor => match arg {
                MetaArg::String(_) => self.const_pool.push(arg.clone()),
                _ => return Err(Error::InternalUnexpectedArgType),
            },
            MetaOp::Field => match arg {
                MetaArg::Field(i, f) => {
                    self.field_map.insert(i.to_owned(), *f);
                    self.const_pool.push(arg.clone());
                }
                _ => return Err(Error::InternalUnexpectedArgType),
            },
            MetaOp::Parameter => match arg {
                MetaArg::Parameter(i, x) => {
                    self.const_map.insert(i.to_owned(), *x);
                }
                _ => return Err(Error::InternalUnexpectedArgType),
            },
            MetaOp::Radius => match arg {
                MetaArg::Radius(x) => {
                    // inline
                    self.metadata.push(Self::metadata_instruction(
                        MetaOp::Radius,
                        *x as u16,
                        None,
                        None,
                    ));
                    return Ok(());
                }
                _ => return Err(Error::InternalUnexpectedArgType),
            },
            MetaOp::Symmetries => match arg {
                MetaArg::Symmetries(x) => {
                    // inline
                    self.metadata.push(Self::metadata_instruction(
                        MetaOp::Symmetries,
                        x.bits() as u16,
                        None,
                        None,
                    ));
                    return Ok(());
                }
                _ => return Err(Error::InternalUnexpectedArgType),
            },
        }
        self.metadata
            .push(Self::metadata_instruction(*op, n as u16, None, None));
        Ok(())
    }

    fn count_instruction_field(f: &Field) -> usize {
        match f {
            Field::Ref(_) => 2,
            Field::Selector(x) => {
                if *x == base::FieldSelector::ALL {
                    2
                } else {
                    1
                }
            }
        }
    }

    fn count_instruction_arg(a: &Arg) -> usize {
        match a {
            Arg::SiteNumber(_, f) => 1 + Self::count_instruction_field(f),
            Arg::Register(_, f) => 1 + Self::count_instruction_field(f),
            Arg::ConstRef(_, f) => Self::count_instruction_field(f),
            Arg::Type(_) => 1,
            _ => 0,
        }
    }

    fn count_instruction_args(args: &Args) -> usize {
        match args  {
            Args::Null => 0,
            Args::Unary(a) => Self::count_instruction_arg(a),
            Args::Binary(a, b) => Self::count_instruction_arg(a) + Self::count_instruction_arg(b),
            Args::Ternary(a, b, c) => {
                Self::count_instruction_arg(a)
                    + Self::count_instruction_arg(b)
                    + Self::count_instruction_arg(c)
            }
        }
    }

    fn process_labels(&mut self, ns: &Vec<Node>) -> Result<(), Error> {
        let mut ln = 0;
        for n in ns {
            match n {
                Node::Label(x) => {
                    self.label_map.insert(x.to_owned(), ln + 1);
                }
                Node::Instruction(_, a) => ln += Self::count_instruction_args(a),
                _ => return Err(Error::InternalUnexpectedNodeType),
            };
        }
        Ok(())
    }

    fn validate_instruction_args(op: &Op, args: &Args) -> Result<(), Error> {
        match op {
            // nullary
            Op::Nop | Op::Exit | Op::Ret => match args {
                Args::Null => Ok(()),
                _ => return Err(Error::InternalUnexpectedArgsCount),
            },
            // unary
            Op::UseSymmetries
            | Op::RestoreSymmetries
            | Op::Push
            | Op::Pop
            | Op::Call
            | Op::Jump
            | Op::LookupSite
            | Op::LookupRegister => match args {
                Args::Unary(_) => Ok(()),
                _ => return Err(Error::InternalUnexpectedArgsCount),
            },
            // binary
            Op::Copy
            | Op::Swap
            | Op::Scan
            | Op::Checksum
            | Op::Neg
            | Op::BitCount
            | Op::BitScanForward
            | Op::BitScanReverse => match args {
                Args::Binary(_, _) => Ok(()),
                _ => return Err(Error::InternalUnexpectedArgsCount),
            },
            // ternary
            Op::Add
            | Op::Sub
            | Op::Mod
            | Op::Mul
            | Op::Div
            | Op::Less
            | Op::LessEqual
            | Op::Or
            | Op::And
            | Op::Xor
            | Op::Equal
            | Op::LShift
            | Op::RShift
            | Op::JumpRelativeOffset
            | Op::JumpZero
            | Op::JumpNonZero => match args {
                Args::Ternary(_, _, _) => Ok(()),
                _ => return Err(Error::InternalUnexpectedArgsCount),
            },
        }
    }

    fn process_instruction(&mut self, n: &Node) -> Result<(), Error> {
        let (op, args) = match n {
            Node::Instruction(op, args) => {
                Self::validate_instruction_args(op, args)?;
                (op, args)
            }
            _ => return Err(Error::InternalUnexpectedNodeType),
        };
        let argv = match args {
            Args::Null => vec![],
            Args::Unary(x) => vec![x],
            Args::Binary(x, y) => vec![x, y],
            Args::Ternary(x, y, z) => vec![x, y, z],
        };
        let mut instr: u64 = (*op as u64) << 48;
        let mut i: u8 = 48;
        for a in argv {
            i -= 16;
            match a {
                Arg::Label(x) => instr |= (self.label_map[x.as_str()] as u64) << i,
                Arg::SiteNumber(_, _) => todo!(),
                Arg::Symmetries(_) => todo!(),
                Arg::Register(_, _) => todo!(),
                Arg::ConstRef(_, _) => todo!(),
                Arg::Type(_) => todo!(),
            }
        }
        self.code.push(instr);
        Ok(())
    }

    fn write_u96<W: WriteBytesExt>(w: &mut W, x: Const) -> Result<(), io::Error> {
        let (raw, sign) = match x {
            Const::Unsigned(x) => (x.0, 0),
            Const::Signed(x) => (x.0 as u128, 1 << 31),
        };
        w.write_u64::<LittleEndian>(raw as u64)?;
        w.write_u32::<LittleEndian>((raw >> 64) as u32 | sign)?;
        Ok(())
    }

    fn write_pool_string<W: WriteBytesExt>(w: &mut W, x: String) -> Result<(), Error> {
        w.write_u8(MetaArg::String as u8)?;
        let data = x.as_bytes();
        w.write_u16::<LittleEndian>(data.len() as u16)?;
        w.write_all(data)?;
        Ok(())
    }

    fn write_pool_entry<W: WriteBytesExt>(w: &mut W, a: MetaArg) -> Result<(), Error> {
        match a {
            MetaArg::String(x) => Self::write_pool_string(w, x),
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

        w.write_u32::<LittleEndian>(MAGIC_NUMBER)?;
        w.write_u16::<LittleEndian>(Self::MINOR_VERSION)?;
        w.write_u16::<LittleEndian>(Self::MAJOR_VERSION)?;

        w.write_u16::<LittleEndian>(self.const_pool.len() as u16)?;
        for a in &self.const_pool {
            Self::write_pool_entry(w, a.clone())?
        }

        w.write_u8(self.metadata.len() as u8)?;
        for i in &self.metadata {
            w.write_u64::<LittleEndian>(*i)?;
        }

        w.write_u16::<LittleEndian>(self.code.len() as u16)?;
        for i in &self.code {
            w.write_u64::<LittleEndian>(*i)?;
        }

        Ok(())
    }
}
