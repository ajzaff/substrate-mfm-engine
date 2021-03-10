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
        static ref RE: Regex = Regex::new("^([^:]+)(?::(.+))?$").unwrap();
    }
    let caps = RE.captures(s).unwrap();
    Ok(Input {
        src: caps.get(1).unwrap().as_str().to_owned(),
        dst: caps.get(2).map(|x| x.as_str().to_owned()),
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
    let mut compiler = Compiler::new(args.build_tag.as_str());

    for i in args.input {
        let dst = match i.dst {
            Some(o) => o,
            None => {
                if !one_input || atty::is(Stream::Stdout) {
                    i.default_dst()
                } else {
                    "-".to_string()
                }
            }
        };
        let mut file = File::open(Path::new::<String>(&i.src))?;
        let mut v = Vec::new();
        let mut s = String::new();
        if let Err(why) = file.read_to_string(&mut s) {
            return Err(format_err!("failed to read input file: {}", why));
        }
        if let Err(why) = compiler.compile_to_writer(&mut v, s.as_str()) {
            return Err(format_err!("failed to compile input file: {}", why));
        }
        if dst == "-" {
            io::stdout().write_all(v.as_slice())?;
            return Ok(());
        }
        if let Err(why) = fs::write(Path::new::<String>(&dst), v) {
            return Err(format_err!("failed to write output file: {}", why));
        }
    }

    Ok(())
}
