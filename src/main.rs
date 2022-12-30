use anyhow::Context;
use clap::Parser;

pub mod vm;

use vm::run_program;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The program image to run
    #[arg(short, long)]
    program: String,

    // TODO: Change this to make it optional
    /// The input image to run
    #[arg(short, long)]
    input: String,

    // TODO: Change this to make it optional
    /// The output image to run, defaults to 'output.png'
    #[arg(short, long, default_value = "output.png")]
    output: String,

    /// The new program image, defaults to 'program.png'
    #[arg(long, default_value = "program_output.png")]
    program_output: String,
}

fn main() {
    let args = Args::parse();
    let input = image::open(args.input)
        .context("Failed reading input image")
        .unwrap()
        .into_rgb8();
    let program = image::open(args.program)
        .context("Failed reading program image")
        .unwrap()
        .into_rgb8();
    let (program, output) = run_program(program, input)
        .context("Failed to run program")
        .unwrap();
    output
        .save(args.output)
        .context("Unable to save output image")
        .unwrap();
    program
        .save(args.program_output)
        .context("Unable to save program output image")
        .unwrap();
}
