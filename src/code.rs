#[macro_use]
use lalrpop_util::lalrpop_mod;

use lalrpop_util::lexer::Token;
use lalrpop_util::ParseError;
use std::fmt;

lalrpop_mod!(pub substrate); // syntesized by LALRPOP

#[derive(Debug)]
pub enum CompileErr<'input> {
    ParseError(ParseError<usize, Token<'input>, &'static str>),
}

impl fmt::Display for CompileErr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(x) => write!(f, "{}", x),
        }
    }
}

pub fn compile_to_bytes<'input>(src: &'input str) -> Result<&'input [u8], CompileErr> {
    let p = substrate::FileParser::new()
        .parse(src)
        .map_err(|e| CompileErr::ParseError(e));
    if p.is_err() {
        return Err(p.unwrap_err());
    }

    Ok(&[])
}
