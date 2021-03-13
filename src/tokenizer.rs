//! A tokenizer for EWAL.

use std::str::CharIndices;
use unicode_xid::UnicodeXID;

use self::ErrorCode::*;
use self::Tok::*;

pub struct Location(ln: usize, col: usize ); 

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub location: Location,
    pub code: ErrorCode
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorCode {
    UnrecognizedToken,
    UnterminatedEscape,
    UnterminatedStringLiteral,
    UnterminatedCode,
    ExpectedStringLiteral,
}

fn error<T>(c: ErrorCode, l: usize) -> Result<T,Error> {
    Err(Error { location: l, code: c })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tok<'input> {
    // Metadata:
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

    // Instructions:
    Nop,
    Exit,
    SwapSites,
    SetSite,
    SetField,
    SetSiteField,
    GetSite,
    GetField,
    GetSiteField,
    GetType,
    GetParameter,
    Scan,
    SaveSymmetries,
    UseSymmetries,
    RestoreSymmetries,
    Push0,
    Push1,
    Push2,
    Push3,
    Push4,
    Push5,
    Push6,
    Push7,
    Push8,
    Push9,
    Push10,
    Push11,
    Push12,
    Push13,
    Push14,
    Push15,
    Push16,
    Push17,
    Push18,
    Push19,
    Push20,
    Push21,
    Push22,
    Push23,
    Push24,
    Push25,
    Push26,
    Push27,
    Push28,
    Push29,
    Push30,
    Push31,
    Push32,
    Push33,
    Push34,
    Push35,
    Push36,
    Push37,
    Push38,
    Push39,
    Push40,
    Push,
    Pop,
    Dup,
    Over,
    Swap,
    Rot,
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

    // Identifiers:
    Ident(&'input str), // excludes the `"`
    StringLiteral(&'input str), // excludes the `"`
    HexConst(&'input str),
    BinConst(&'input str),
    DecConst(&'input str),
    SignedConst(&'input str),

    // Symbols:
    Colon,
    Comma,
    CommentStart,
    CommentEnd,
    CommentLine,
}

pub struct Tokenizer<'input> {
    text: &'input str,
    chars: CharIndices<'input>,
    lookahead: Option<(usize, char)>,
    shift: usize,
}

pub type Spanned<T> = (usize, T, usize);

const METADATA: &'static [(&'static str, Tok<'static>)] = &[
    (".name", Name),
    (".symbol", Symbol),
    (".desc", Desc),
    (".author", Author),
    (".license", License),
    (".radius", Radius),
    (".bgcolor", BgColor),
    (".fgcolor", FgColor),
    (".symmetries", Symmetries),
    (".field", Field),
    (".parameter", Parameter),
];

const INSTRUCTION: &'static [(&'static str, Tok<'static>)] = &[
    ("nop", Nop),
    ("exit", Exit),
    ("swapsites", SwapSites),
    ("setsite", SetSite),
    ("setfield", SetField),
    ("setsitefield", SetSiteField),
    ("getsite", GetSite),
    ("getfield", GetField),
    ("getsitefield", GetSiteField),
    ("gettype", GetType),
    ("getparameter", GetParameter),
    ("scan", Scan),
    ("savesymmetries", SaveSymmetries),
    ("usesymmetries", UseSymmetries),
    ("restoresymmetries", RestoreSymmetries),
    ("push0", Push0),
    ("push1", Push1),
    ("push2", Push2),
    ("push3", Push3),
    ("push4", Push4),
    ("push5", Push5),
    ("push6", Push6),
    ("push7", Push7),
    ("push8", Push8),
    ("push9", Push9),
    ("push10", Push10),
    ("push11", Push11),
    ("push12", Push12),
    ("push13", Push13),
    ("push14", Push14),
    ("push15", Push15),
    ("push16", Push16),
    ("push17", Push17),
    ("push18", Push18),
    ("push19", Push19),
    ("push20", Push20),
    ("push21", Push21),
    ("push22", Push22),
    ("push23", Push23),
    ("push24", Push24),
    ("push25", Push25),
    ("push26", Push26),
    ("push27", Push27),
    ("push28", Push28),
    ("push29", Push29),
    ("push30", Push30),
    ("push31", Push31),
    ("push32", Push32),
    ("push33", Push33),
    ("push34", Push34),
    ("push35", Push35),
    ("push36", Push36),
    ("push37", Push37),
    ("push38", Push38),
    ("push39", Push39),
    ("push40", Push40),
    ("push", Push),
    ("pop", Pop),
    ("dup", Dup),
    ("over", Over),
    ("swap", Swap),
    ("rot", Rot),
    ("call", Call),
    ("ret", Ret),
    ("checksum", Checksum),
    ("add", Add),
    ("sub", Sub),
    ("neg", Neg),
    ("mod", Mod),
    ("mul", Mul),
    ("div", Div),
    ("less", Less),
    ("lessequal", LessEqual),
    ("or", Or),
    ("and", And),
    ("xor", Xor),
    ("equal", Equal),
    ("bitcount", BitCount),
    ("bitscanforward", BitScanForward),
    ("bitscanreverse", BitScanReverse),
    ("lshift", LShift),
    ("rshift", RShift),
    ("jump", Jump),
    ("jumprelativeoffset", JumpRelativeOffset),
    ("jumpzero", JumpZero),
    ("jumpnonzero", JumpNonZero),
];

impl<'input> Tokenizer<'input> {
    pub fn new(text: &'input str, shift: usize) -> Tokenizer<'input> {
        let mut t = Tokenizer {
            text: text,
            chars: text.char_indices(),
            lookahead: None,
            shift: shift,
        };
        t
    }

    fn escape(&mut self, idx: usize) -> Result<Spanned<Tok<'input>>, Error> {
        todo!()
    }

    fn string_literal(&mut self, idx: usize) -> Result<Spanned<Tok<'input>>, Error> {
        todo!()
    }
}

impl<'input> Iterator for Tokenizer<'input> {
    type Item = Result<Spanned<Tok<'input>>, Error>;

    fn next(&mut self) -> Option<Result<Spanned<Tok<'input>>, Error>> {
        todo!()
    }
} 