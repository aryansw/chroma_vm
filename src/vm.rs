use std::mem;

use crate::instruction::Instruction;
use anyhow::Result;
use image::{ImageBuffer, Rgb};

pub type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;
type Hex = Rgb<u8>;

struct Program {
    program: Image,
    input: Option<Image>,
    output: Option<Image>,
    iptr: (u32, u32),
}

pub fn run_program(program: Image, input: Option<Image>) -> Result<(Image, Option<Image>)> {
    // Images can only be 4096 x 4096 pixels, let's make sure
    if program.width() > 4096 || program.height() > 4096 {
        return Err(anyhow::anyhow!("Program image is too large"));
    }
    if let Some(input) = &input {
        if input.width() > 4096 || input.height() > 4096 {
            return Err(anyhow::anyhow!("Input image is too large"));
        }
    }

    let mut program = Program::new(program, input);
    loop {
        let instr = program.decode_instr();
        match instr {
            Instruction::Halt => break,
            _ => (),
        }
    }
    Ok((program.program, program.output))
}

impl Program {
    fn new(program: Image, input: Option<Image>) -> Self {
        Self {
            program,
            input,
            output: None,
            iptr: (0, 0),
        }
    }

    // Decode an instruction, and move the instruction pointer to the next instruction
    fn decode_instr(&mut self) -> Instruction {
        let hex: &Hex = self.program.get_pixel(self.iptr.0, self.iptr.1);
        // Take first 6 bits of the hex code
        let opcode = hex[0] >> 2;
        // The different types of instructions available
        let count = mem::variant_count::<Instruction>() as u8;
        let instr = match opcode % count {
            0 => Instruction::Halt,
            _ => Instruction::Halt,
        };
        self.next_instr();
        instr
    }

    // Move the instruction pointer to the next instruction to read
    fn next_instr(&mut self) {
        self.iptr.0 += 1;
        if self.iptr.0 >= self.program.width() {
            self.iptr.0 = 0;
            self.iptr.1 += 1;
        }
    }
}
