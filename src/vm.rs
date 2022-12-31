use std::mem;

use crate::instruction::{
    Instruction::{self, *},
    Register,
};
use anyhow::Result;
use bitvec::prelude::*;
use image::{ImageBuffer, Rgb};

pub type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;
type Hex = Rgb<u8>;

struct Program {
    program: Image,
    input: Option<Image>,
    output: Option<Image>,
    registers: Vec<Hex>,
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
        let instr = program.decode_instr()?;
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
            registers: Vec::with_capacity(32),
        }
    }

    // Decode an instruction, and move the instruction pointer to the next instruction
    fn decode_instr(&mut self) -> Result<Instruction> {
        // Fetch instruction
        let (x, y) = self.iptr();
        let hex: &Hex = self.program.get_pixel(x, y);
        // Take first 6 bits of the hex code
        let opcode = hex[0] >> 2;
        // The different types of instructions available
        let count = mem::variant_count::<Instruction>() as u8;
        let instr = match opcode % count {
            0 => {
                let register = Self::get_hex_slice(hex, 6, 12) as u8;
                let value = Self::get_hex_slice(hex, 12, 18) as u16;
                ReadSmallValue(register.into(), value)
            }
            1 => {
                let register = Self::get_hex_slice(hex, 6, 12) as u8;
                let (x, y) = self.get_next_iptr((x, y));
                let hex: &Hex = self.program.get_pixel(x, y);
                let value = Self::get_hex_slice(hex, 0, 24) as u32;
                self.next_iptr();
                ReadFullValue(register.into(), value)
            }
            2 => {
                let register = Self::get_hex_slice(hex, 6, 12) as u8;
                let value = Self::get_hex_slice(hex, 12, 18) as u16;
                Halt
            }
            _ => Halt,
        };
        self.next_iptr();
        Ok(instr)
    }

    fn get_hex_slice(hex: &Hex, start: u8, end: u8) -> u32 {
        hex.0.as_bits::<Lsb0>()[start as usize..end as usize].load::<u32>()
    }

    // First 12 bits for x, last 12 bits for y
    fn iptr(&self) -> (u32, u32) {
        let bytes = self.registers[31].0;
        let x = (bytes[0] as u32) << 4 | (bytes[1] as u32) >> 4;
        let y = ((bytes[1] as u32) & 0b1111) << 8 | (bytes[2] as u32);
        (x, y)
    }

    // Update the instruction pointer to the next instruction
    fn set_iptr(&mut self, (x, y): (u32, u32)) {
        let bytes = &mut self.registers[31].0;
        *bytes = [
            (x >> 4) as u8,
            ((x & 0b1111) << 4 | (y >> 8)) as u8,
            (y & 0b11111111) as u8,
        ];
    }

    // Move the instruction pointer to the next instruction to read
    fn next_iptr(&mut self) {
        self.set_iptr(self.get_next_iptr(self.iptr()))
    }

    // What the next instruction pointer would be
    fn get_next_iptr(&self, (x, y): (u32, u32)) -> (u32, u32) {
        if x == self.program.width() - 1 {
            (0, y + 1)
        } else {
            (x + 1, y)
        }
    }
}
