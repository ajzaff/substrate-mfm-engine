extern crate lalrpop_util;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate lazy_static;

mod ast;
mod base;
mod code;

use crate::code::Compiler;
use atty::Stream;
use quicli::prelude::*;
use regex::Regex;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug)]
enum Error {
    CliError,
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Error {
        Error::CliError
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error")
    }
}

struct Input {
    num: Option<u16>,
    src: String,
    dst: Option<String>,
}

impl Input {
    fn default_dst(&self) -> String {
        let mut s = self.src.clone();
        s.push_str(".bin");
        s
    }
}

fn parse_input(s: &str) -> Result<Input, Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new("(?:(0|[1-9][0-9]*)=)?([^:]+)(?::(.+?))?").unwrap();
    }
    let caps = RE.captures(s).unwrap();
    let num = if let Some(v) = caps.get(1) {
        Some(u16::from_str(v.as_str())?)
    } else {
        None
    };
    Ok(Input {
        num: num,
        src: caps.get(2).unwrap().as_str().to_owned(),
        dst: caps.get(3).map(|x| x.as_str().to_owned()),
    })
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(long = "input", short = "i", parse(try_from_str = parse_input))]
    input: Vec<Input>,

    #[structopt(long = "build-tag", short = "t", default_value = "ephemeral")]
    build_tag: String,
}

fn main() -> CliResult {
    Ok(ewac_main()?)
}

fn ewac_main() -> Result<(), failure::Error> {
    let args = Cli::from_args();
    let one_input = args.input.len() == 1;

    for i in args.input {
        // TODO: Move type mapping out of compiler to allow it to live longer.
        let mut c = Compiler::new(args.build_tag.as_str());

        let out = match &i.dst {
            Some(o) => o.clone(),
            None => {
                if !one_input || atty::is(Stream::Stdout) {
                    i.default_dst()
                } else {
                    "-".to_string()
                }
            }
        };
        let mut file = File::open(Path::new::<String>(&i.src))?;
        let mut s = String::new();
        if let Err(why) = file.read_to_string(&mut s) {
            return Err(format_err!("failed to read input file: {}", why));
        }
        let mut v = Vec::new();
        let res = c.compile_to_writer(&mut v, s.as_ref(), i.num);
        if let Err(why) = res {
            return Err(format_err!("failed to compile input file: {}", why));
        }
        if out == "-" {
            io::stdout().write_all(v.as_slice())?;
            return Ok(());
        }
        if let Err(why) = fs::write(Path::new::<String>(&out), v) {
            return Err(format_err!("failed to write output file: {}", why));
        }
    }

    Ok(())
}
