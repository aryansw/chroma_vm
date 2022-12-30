use anyhow::{Context, Result};
use image::load_from_memory;
use vm::run_program;

use wasm_bindgen::prelude::*;

pub mod vm;

#[wasm_bindgen(getter_with_clone)]
pub struct Output {
    pub program: Vec<u8>,
    pub output: Vec<u8>,
}

#[wasm_bindgen(js_name = processImage)]
pub fn run_image(program: Vec<u8>, input: Vec<u8>) -> Result<Output, String> {
    process_image(program, input).map_err(|e| e.to_string())
}

pub fn process_image(program: Vec<u8>, input: Vec<u8>) -> Result<Output> {
    let program = load_from_memory(&program)
        .context("Failed reading program image")?
        .to_rgb8();
    let input = load_from_memory(&input)
        .context("Failed reading input image")?
        .to_rgb8();

    let (program, output) = run_program(program, input).context("Failed to Run Program")?;

    Ok(Output {
        program: program.to_vec(),
        output: output.to_vec(),
    })
}

// Use this to reduce the size of the allocator
extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
