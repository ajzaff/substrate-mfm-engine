extern crate lalrpop_util;

#[macro_use]
extern crate failure;

mod ast;
mod base;
mod code;

use atty::Stream;
use quicli::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    file: String,

    #[structopt(long = "output", short = "o", default_value = "")]
    output: String,

    #[structopt(flatten)]
    verbosity: Verbosity,
}

fn main() -> CliResult {
    Ok(ewac_main()?)
}

const DEFAULT_OUTPUT_FILENAME: &'static str = "a.ewac";

fn ewac_main() -> Result<(), failure::Error> {
    let mut args = Cli::from_args();

    if args.output.is_empty() {
        args.output = if atty::is(Stream::Stdout) {
            String::from(DEFAULT_OUTPUT_FILENAME)
        } else {
            String::from("-")
        }
    }

    let file = File::open(Path::new::<String>(&args.file));
    let mut s = String::new();
    if let Err(why) = file?.read_to_string(&mut s) {
        return Err(format_err!("failed to read input file: {}", why));
    }

    let out = code::compile_to_bytes(s.as_ref());
    if let Err(why) = out {
        return Err(format_err!("failed to compile input file: {}", why));
    }

    todo!()
}
