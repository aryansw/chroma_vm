#![feature(variant_count)]
use anyhow::Context;
use clap::Parser;

pub mod error;
pub mod instruction;
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
    input: Option<String>,

    // TODO: Change this to make it optional
    /// The output image to run, defaults to 'output.png'
    #[arg(short, long, default_value = "output.png")]
    output: Option<String>,

    /// The new program image, defaults to 'program.png'
    #[arg(long, default_value = "program_output.png")]
    program_output: String,
}

fn main() {
    let args = Args::parse();
    let input = args.input.map(|path| {
        image::open(path)
            .context("Failed reading input image")
            .unwrap()
            .into_rgb8()
    });
    let program = image::open(args.program)
        .context("Failed reading program image")
        .unwrap()
        .into_rgb8();
    let (program, output) = run_program(program, input)
        .context("Failed to run program")
        .unwrap();
    output.map(|image| {
        args.output.map(|path| {
            image
                .save(path)
                .context("Unable to save output image")
                .unwrap()
        })
    });
    program
        .save(args.program_output)
        .context("Unable to save program output image")
        .unwrap();
}
