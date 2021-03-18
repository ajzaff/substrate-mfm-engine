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
use stderrlog;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "ewimops", about = "Run EWAL image processing tasks.")]
struct Cli {
    #[structopt(name = "INPUT", help = "An image file to process", required = true)]
    input: String,

    #[structopt(
        long = "output",
        short = "o",
        help = "Output file name for the output image."
    )]
    output: Option<String>,

    #[structopt(
        long = "init",
        help = "A compiled EWAL program which initializes the image operation."
    )]
    init: String,

    #[structopt(
        long = "op",
        help = "Compiled EWAL programs which execute the image operation."
    )]
    ops: Vec<String>,

    #[structopt(
        long = "grid-scale",
        help = "Grid scale factor relative to the input image.",
        default_value = "1"
    )]
    scale: u8,

    #[structopt(
        long = "random-seed",
        help = "A 64 bit random seed used to initialize the random number generator.",
        default_value = "1337"
    )]
    random_seed: u64,

    #[structopt(short = "q", long = "quiet", help = "Silence all logging output.")]
    quiet: bool,

    #[structopt(
        short = "v",
        long = "verbose",
        help = "Configure logging verbosity",
        parse(from_occurrences)
    )]
    verbose: usize,
}

fn main() {
    let args = Cli::from_args();
    stderrlog::new()
        .quiet(args.quiet)
        .verbosity(args.verbose)
        .init()
        .unwrap();
    ewimops_main(&args);
}

fn ewimops_main(args: &Cli) {
    let mut runtime = Runtime::new();

    let mut file = File::open(Path::new::<String>(&args.input)).expect("Failed to open input file");
    let mut r = BufReader::new(&mut file);
    let atom = runtime
        .load_from_reader(&mut r)
        .expect("Failed to process input file");
}
