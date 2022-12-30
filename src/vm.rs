use anyhow::Result;
use image::{ImageBuffer, Rgb};

pub type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;

pub fn run_program(program: Image, input: Image) -> Result<(Image, Image)> {
    // VM goes here
    Ok((program, input))
}
