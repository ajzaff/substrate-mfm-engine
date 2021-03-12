#[path = "../runtime/mod.rs"]
mod runtime;

#[path = "../base/mod.rs"]
mod base;

#[path = "../ast.rs"]
mod ast;

use crate::runtime::mfm::EventWindow;
use crate::runtime::Runtime;
use quicli::prelude::*;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
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

#[derive(StructOpt)]
struct Cli {
  #[structopt(long = "input", short = "i")]
  input: Vec<String>,
}

fn main() -> CliResult {
  Ok(ewar_main()?)
}

fn ewar_main() -> Result<(), failure::Error> {
  let args = Cli::from_args();
  let mut runtime = Runtime::new();

  for i in args.input {
    let mut file = File::open(Path::new::<String>(&i))?;
    let mut r = BufReader::new(&mut file);
    if let Err(why) = runtime.load_from_reader(&mut r) {
      return Err(format_err!("failed to compile input file: {}", why));
    }
  }

  let mut ew = EventWindow::new();
  if let Err(why) = runtime.execute(&mut ew) {
    return Err(format_err!("failed to execute: {}", why));
  }

  Ok(())
}
