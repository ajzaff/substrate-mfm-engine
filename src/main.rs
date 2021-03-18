extern crate lalrpop_util;
extern crate lazy_static;

mod ast;
mod base;
mod code;

use crate::code::Compiler;
use atty::Stream;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(name = "INPUT", help = "Input EWAL source files.")]
    input: Vec<String>,

    #[structopt(
        long = "output",
        short = "o",
        help = "Output target directory. Will be created if not found. Stdout is - but will fail given multiple inputs."
    )]
    output_dir: Option<String>,

    #[structopt(
        long = "build-tag",
        short = "t",
        help = "Build tag compiled into the output binary.",
        default_value = "ephemeral"
    )]
    build_tag: String,
}

fn main() {
    let args = Cli::from_args();
    ewac_main(&args);
}

fn ewac_main(args: &Cli) {
    let is_explicit_stdout = args.output_dir == Some("-".to_string());
    let is_pipe = is_explicit_stdout || (args.output_dir.is_none() && !atty::is(Stream::Stdout));
    if is_pipe && args.input.len() != 1 {
        eprintln!("Pipes are only supported with one input file.");
        exit(1);
    }

    if args.input.len() == 0 {
        eprintln!("No input files.");
        exit(1);
    }

    let curr_dir = env::current_dir().expect("Could not get current directory");
    let output_dir = if let Some(dir) = args.output_dir.as_ref() {
        let d = Path::new::<String>(&dir);
        if !is_explicit_stdout {
            fs::create_dir_all(d).expect("Failed to create target directory");
        }
        d
    } else {
        let path = curr_dir
            .to_str()
            .expect("Current directory is not valid UTF-8");
        Path::new::<str>(path)
    };

    let mut compiler = Compiler::new(args.build_tag.as_str());

    for i in &args.input {
        let filename = Path::new::<String>(&i);
        let mut file = File::open(filename).expect("Failed to open input file");
        let mut v = Vec::new();
        let mut s = String::new();
        file.read_to_string(&mut s)
            .expect("Failed to read input file");
        compiler
            .compile_to_writer(&mut v, s.as_str())
            .expect("Failed to compile input file");

        if is_pipe {
            io::stdout()
                .write_all(v.as_slice())
                .expect("Failed to write to stdout");
        } else {
            let path = output_dir.join(filename.file_stem().unwrap());
            fs::write(path, v).expect("Failed to write target")
        }
    }
}
