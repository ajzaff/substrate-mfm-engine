#[path = "../runtime/mod.rs"]
mod runtime;

#[path = "../base/mod.rs"]
mod base;

#[path = "../ast.rs"]
mod ast;

use crate::runtime::mfm::{select_symmetries, DenseGrid, EventWindow, Rand, SparseGrid};
use crate::runtime::{Cursor, Runtime};
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView};
use log::trace;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use runtime::mfm::Blit;
use std::fs;
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
        help = "A 64 bit seed used to initialize the random number generator.",
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
    let image = ImageReader::open(Path::new::<String>(&args.input))
        .expect("Failed to open input image")
        .decode()
        .expect("Failed to decode input image");
    let mut init_file =
        File::open(Path::new::<String>(&args.init)).expect("Failed to open init file");
    let mut r = BufReader::new(&mut init_file);
    let init = runtime
        .load_from_reader(&mut r)
        .expect("Failed to process init file");
    for op in &args.ops {
        let mut file = File::open(Path::new::<String>(op)).expect("Failed to open op file");
        let mut r = BufReader::new(&mut file);
        runtime
            .load_from_reader(&mut r)
            .expect("Failed to process op file");
    }
    let mut rng = SmallRng::from_entropy();
    let (width, height) = image.dimensions();
    let mut ew = SparseGrid::new(&mut rng, (width as usize, height as usize));
    ew.blit_image(&image.into_rgba8());
    ew.set(0, init.new_atom());
    let mut cursor = Cursor::with_symmetry(select_symmetries(ew.rand_u32(), init.symmetries));
    for _ in 0..10000000 {
        Runtime::execute(&mut ew, &mut cursor, &runtime.code_map).expect("Failed to execute");
        ew.reset();
        cursor.reset(select_symmetries(ew.rand_u32(), init.symmetries));
    }
    if let Some(output) = &args.output {
        let mut im = DynamicImage::new_rgba8(width, height);
        ew.unblit_image(im.as_mut_rgba8().unwrap());
        let mut file = fs::File::create(Path::new::<String>(output))
            .expect("Failed to create output image file");
        im.write_to(&mut file, image::ImageOutputFormat::Png)
            .expect("Failed to write output image");
    }
}
