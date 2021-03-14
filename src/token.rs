//! A tokenizer for EWAL.

use enquote::enquote;

use std::str::CharIndices;

use self::ErrorCode::*;
use self::Tok::*;

type Location = usize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub location: Location,
    pub code: ErrorCode,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorCode {
    UnrecognizedToken,
    UnterminatedEscape,
    UnterminatedStringLiteral,
    UnterminatedCode,
    ExpectedStringLiteral,
}

fn error<T>(c: ErrorCode, l: Location) -> Result<T, Error> {
    Err(Error {
        location: l,
        code: c,
    })
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
    StringLiteral(&'input str), // excludes the `"`
    Ident(&'input str),
    HexConst(&'input str),
    BinConst(&'input str),
    DecConst(&'input str),
    SignedConst(&'input str),

    // Symbols:
    Colon,
    Comma,
    CommentStart, // "/*"
    CommentEnd,   // "*/"
    CommentLine,  // "//"
}

impl<'input> fmt::Display for Token<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Token::*;

        let s = match *self {
            // Metadata:
            Name => ".name",
            Symbol => ".symbol",
            Desc => ".desc",
            Author => ".author",
            License => ".license",
            Radius => ".radius",
            BgColor => ".bgcolor",
            FgColor => ".fgcolor",
            Symmetries => ".symmetries",
            Field => ".field",
            Parameter => ".parameter",

            // Instructions:
            Nop => "nop",
            Exit => "exit",
            SwapSites => "swapsites",
            SetSite => "setsite",
            SetField => "setfield",
            SetSiteField => "setsitefield",
            GetSite => "getsite",
            GetField => "getfield",
            GetSiteField => "getsitefield",
            GetType => "gettype",
            GetParameter => "getparameter",
            Scan => "scan",
            SaveSymmetries => "savesymmetries",
            UseSymmetries => "usesymmetries",
            RestoreSymmetries => "restoresymmetries",
            Push0 => "push0",
            Push1 => "push1",
            Push2 => "push2",
            Push3 => "push3",
            Push4 => "push4",
            Push5 => "push5",
            Push6 => "push6",
            Push7 => "push7",
            Push8 => "push8",
            Push9 => "push9",
            Push10 => "push10",
            Push11 => "push11",
            Push12 => "push12",
            Push13 => "push13",
            Push14 => "push14",
            Push15 => "push15",
            Push16 => "push16",
            Push17 => "push17",
            Push18 => "push18",
            Push19 => "push19",
            Push20 => "push20",
            Push21 => "push21",
            Push22 => "push22",
            Push23 => "push23",
            Push24 => "push24",
            Push25 => "push25",
            Push26 => "push26",
            Push27 => "push27",
            Push28 => "push28",
            Push29 => "push29",
            Push30 => "push30",
            Push31 => "push31",
            Push32 => "push32",
            Push33 => "push33",
            Push34 => "push34",
            Push35 => "push35",
            Push36 => "push36",
            Push37 => "push37",
            Push38 => "push38",
            Push39 => "push39",
            Push40 => "push40",
            Push => "push",
            Pop => "pop",
            Dup => "dup",
            Over => "over",
            Swap => "swap",
            Rot => "rot",
            Call => "call",
            Ret => "ret",
            Checksum => "checksum",
            Add => "add",
            Sub => "sub",
            Neg => "neg",
            Mod => "mod",
            Mul => "mul",
            Div => "div",
            Less => "less",
            LessEqual => "lessequal",
            Or => "or",
            And => "and",
            Xor => "xor",
            Equal => "equal",
            BitCount => "bitcount",
            BitScanForward => "bitscanforward",
            BitScanReverse => "bitscanreverse",
            LShift => "lshift",
            RShift => "rshift",
            Jump => "jump",
            JumpRelativeOffset => "jumprelativeoffset",
            JumpZero => "jumpzero",
            JumpNonZero => "jumpnonzero",

            // Identifiers:
            StringLiteral(x) => enquote('"', x),
            Ident(x) => x,
            HexConst(x) => x,
            BinConst(x) => x,
            DecConst(x) => x,
            SignedConst(x) => x,

            // Symbols:
            Colon => ":",
            Comma => ",",
        };
        s.fmt(f)
    }
}

pub struct Tokenizer<'input> {
    text: &'input str,
    chars: CharIndices<'input>,
    lookahead: Option<(Location, char)>,
    shift: Location,
}

pub type Spanned<T> = (Location, T, Location);

impl<'input> Tokenizer<'input> {
    pub fn new(text: &'input str, shift: Location) -> Tokenizer<'input> {
        let mut t = Tokenizer {
            text: text,
            chars: text.char_indices(),
            lookahead: None,
            shift: shift,
        };
        t
    }

    fn bump(&mut self) -> Option<(Location, char)> {
        self.lookahead = self.chars.next();
        self.lookahead
    }

    fn take_while<F>(&mut self, start: Location, mut keep_going: F) -> (Location, &'input str)
    where
        F: FnMut(char) -> bool,
    {
        self.take_until(start, |c| !keep_going(c))
    }

    fn take_until<F>(&mut self, start: Location, mut terminate: F) -> (Location, &'input str)
    where
        F: FnMut(char) -> bool,
    {
        while let Some((idx, c)) = self.lookahead {
            if terminate(c) {
                return Some((idx, c));
            }
            self.bump();
        }
        None
    }

    fn const(&mut self, start: Location ) -> SpannedToken<'input> {
        todo!()
    }

    fn identifier(&mut self, start: Location) -> SpannedToken<'input> {
        let (mut end, mut ident) = self.take_while(start, is_ident_continue);
        match self.lookahead {
            Some((_, c)) if c == '!' => {
                self.bump();
                end.column += 1.into();
                end.absolute += 1.into();
                ident = self.slice(start, end);
            }
            _ => (),
        }

        let token = match ident {
            src => Token::Identifier(src),
        };

        pos::spanned2(start, end, token)
    }
}

impl<'input> Iterator for Tokenizer<'input> {
    type Item = Result<Spanned<Tok<'input>>, Error>;

    fn next(&mut self) -> Option<Result<Spanned<Tok<'input>>, Error>> {
        todo!()
    }
}
