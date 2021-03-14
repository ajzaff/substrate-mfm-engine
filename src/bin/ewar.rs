#[path = "../runtime/mod.rs"]
mod runtime;

#[path = "../base/mod.rs"]
mod base;

#[path = "../ast.rs"]
mod ast;

use crate::runtime::mfm::EventWindow;
use crate::runtime::Runtime;
use clap::arg_enum;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use structopt::StructOpt;

arg_enum! {
  #[derive(Debug)]
    enum Output {
      BeforeAfter,
      After,
    }
}

arg_enum! {
  #[derive(Debug)]
    enum OutputMode {
      Raw,
      Graphical,
    }
}

arg_enum! {
  #[derive(Debug)]
    enum ColorMode {
      None,
      Color,
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
  name = "ewar",
  about = "Execute event window assembly (EWAL) and collect detailed statistics."
)]
struct Cli {
  #[structopt(name = "INPUT", required = true)]
  input: String,

  #[structopt(
    long = "random-seed",
    help = "A 64 bit random seed used to initialize the random number generator. Random state is never reseeded in case multiple trials are used.",
    default_value = "1337"
  )]
  random_seed: u64,

  #[structopt(
    long = "trials",
    short = "n",
    help = "The number of distinct trials to run. The event window will be cleared and reseeded but random state will not be reset.",
    default_value = "1"
  )]
  n: u32,

  #[structopt(
    long = "seed-element",
    short = "s",
    help = "An element executed prior to each run which initializes the event window. The input element will be placed automatically after each time executing the seed."
  )]
  seed_element: Option<String>,

  #[structopt(
    long = "test",
    short = "t",
    help = "Configures test mode, which asserts the contents of the event window matches the given representation (b64; rfc-4648). An exit code 0 indicates a PASS and 1 a FAIL."
  )]
  expect: Option<String>,

  #[structopt(
    long = "output",
    short = "o",
    possible_values = &Output::variants(),
    case_insensitive = true,
    help = "Configures output artifacts (such as event window images).",
    default_value = "beforeafter",
  )]
  output: Output,

  #[structopt(
    long = "output_mode",
    possible_values = &OutputMode::variants(),
    case_insensitive = true,
    help = "Configures output display mode.",
    default_value = "graphical",
  )]
  output_mode: OutputMode,

  #[structopt(
    long = "color",
    possible_values = &ColorMode::variants(),
    case_insensitive = true,
    help = "Configures color display mode.",
    default_value = "color",
  )]
  color: ColorMode,
}

fn main() {
  let args = Cli::from_args();
  ewar_main(&args);
}

fn ewar_main(args: &Cli) {
  let mut runtime = Runtime::new();

  let mut file = File::open(Path::new::<String>(&args.input)).expect("Failed to open input file");
  let mut r = BufReader::new(&mut file);
  let atom = runtime
    .load_from_reader(&mut r)
    .expect("Failed to process input file");

  let mut ew = EventWindow::new();
  ew.set_type_map(&runtime.type_map);
  *ew.get_mut(0).unwrap() = atom;
  Runtime::execute(&mut ew, runtime.code_map).expect("Failed to execute");
  println!("{}", ew);
}
