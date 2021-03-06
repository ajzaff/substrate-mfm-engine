#[path = "../runtime/mod.rs"]
mod runtime;

#[path = "../base/mod.rs"]
mod base;

#[path = "../ast.rs"]
mod ast;

use crate::runtime::mfm::{
  debug_event_window, select_symmetries, EventWindow, MinimalEventWindow, Rand,
};
use crate::runtime::{Cursor, Runtime};
use clap::arg_enum;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use stderrlog;
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

  #[structopt(short = "q", long = "quiet", help = "Silence all logging output.")]
  quiet: bool,

  #[structopt(
    short = "v",
    long = "verbose",
    help = "Configure logging verbosity",
    parse(from_occurrences)
  )]
  verbose: usize,

  #[structopt(long = "checksum", help = "Perform checksums on output states.")]
  checksum: bool,
}

fn main() {
  let args = Cli::from_args();
  stderrlog::new()
    .quiet(args.quiet)
    .verbosity(args.verbose)
    .init()
    .unwrap();
  ewar_main(&args);
}

fn ewar_main(args: &Cli) {
  let mut runtime = Runtime::new();

  let mut file = File::open(Path::new::<String>(&args.input)).expect("Failed to open input file");
  let mut r = BufReader::new(&mut file);
  let init = runtime
    .load_from_reader(&mut r)
    .expect("Failed to process input file");

  let mut rng = SmallRng::from_entropy();
  let mut ew = MinimalEventWindow::new(&mut rng);
  let s = select_symmetries(ew.rand_u32(), init.symmetries);
  let mut cursor = Cursor::with_symmetry(s);
  ew.set(0, init.new_atom());
  Runtime::execute(&mut ew, &mut cursor, &runtime.code_map).expect("Failed to execute");
  debug_event_window(&ew, &mut std::io::stdout(), &runtime.type_map)
    .expect("Failed to debug event window");
}
