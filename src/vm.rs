use std::mem;

use crate::error::Error::*;
use crate::instruction::{
    Instruction::{self, *},
    Register,
};
use anyhow::{Context, Ok, Result};
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
        println!("{:?}", instr);
        program.run_instruction(instr)?;
    }
    Ok((program.program, program.output))
}

impl Program {
    fn new(program: Image, input: Option<Image>) -> Self {
        let mut registers = Vec::new();
        registers.resize_with(32, || Rgb::<u8>::from([0, 0, 0]));
        Self {
            program,
            input,
            output: None,
            registers,
        }
    }

    // Decode an instruction, and move the instruction pointer to the next instruction
    fn decode_instr(&mut self) -> Result<Instruction> {
        // Fetch instruction
        let (x, y) = self.iptr();
        let hex: &Hex = match self.program.get_pixel_checked(x, y) {
            Some(hex) => hex,
            None => return Ok(Halt),
        };
        // Take first 6 bits of the hex code
        let opcode = hex[0] >> 2;
        // The different types of instructions available
        let count = mem::variant_count::<Instruction>() as u8;
        let instr = match opcode % count {
            0..=1 => {
                let r1: Register = Self::get_register(hex, 1);
                let value = Self::get_hex_slice(hex, 12, 18) as u16;
                match opcode % 2 {
                    0 => LoadLow(r1, value),
                    1 => LoadHigh(r1, value),
                    _ => unreachable!(),
                }
            }
            2 => {
                let r1: Register = Self::get_register(hex, 1);
                let r2: Register = Self::get_register(hex, 2);
                Move(r1, r2)
            }
            // Arithmetic and Comparisons
            3..=15 => {
                let r1: Register = Self::get_register(hex, 1);
                let r2: Register = Self::get_register(hex, 2);
                let r3: Register = Self::get_register(hex, 3);
                match opcode % 13 {
                    0 => Add(r1, r2, r3),
                    1 => Subtract(r1, r2, r3),
                    2 => Multiply(r1, r2, r3),
                    3 => Divide(r1, r2, r3),
                    4 => Modulo(r1, r2, r3),
                    5 => And(r1, r2, r3),
                    6 => Or(r1, r2, r3),
                    7 => Equal(r1, r2, r3),
                    8 => NotEqual(r1, r2, r3),
                    9 => GreaterThan(r1, r2, r3),
                    10 => LessThan(r1, r2, r3),
                    11 => GreaterThanEqual(r1, r2, r3),
                    12 => LessThanEqual(r1, r2, r3),
                    _ => unreachable!(),
                }
            }
            16 => {
                let r1 = Self::get_register(hex, 1);
                let r2 = Self::get_register(hex, 1);
                Alloc(r1, r2)
            }
            17 => {
                let r1: Register = Self::get_register(hex, 1);
                let r2: Register = Self::get_register(hex, 2);
                let r3: Register = Self::get_register(hex, 3);
                MemCopy(r1, r2, r3)
            }
            18 => {
                let r1 = Self::get_register(hex, 1);
                CurrAddress(r1)
            }
            19..=20 => {
                let r1 = Self::get_register(hex, 1);
                match opcode % 2 {
                    0 => Jump(r1),
                    1 => Call(r1),
                    _ => unreachable!(),
                }
            }
            21..=22 => {
                let r1 = Self::get_register(hex, 1);
                let r2 = Self::get_register(hex, 2);
                match opcode % 2 {
                    0 => JumpIf(r1, r2),
                    1 => CallIf(r1, r2),
                    _ => unreachable!(),
                }
            }
            23 => Return,
            24..=25 => {
                let r1 = Self::get_register(hex, 1);
                match opcode % 2 {
                    0 => Push(r1),
                    1 => Pop(r1),
                    _ => unreachable!(),
                }
            }
            _ => Halt,
        };
        self.next_iptr();
        Ok(instr)
    }

    fn run_instruction(&mut self, instr: Instruction) -> Result<()> {
        match instr {
            LoadLow(r1, value) => {
                let high = (value >> 8) as u8;
                let low = value as u8;
                if r1.deref {
                    let (x, y) = Program::get_location(&self.registers[r1.value]);
                    let hex = self
                        .program
                        .get_pixel_mut_checked(x, y)
                        .ok_or(PixelNotPresent(x, y))?;
                    hex.0[1] = high | (hex.0[0] & 0b11110000);
                    hex.0[2] = low;
                } else {
                    let val = &mut self.registers[r1.value];
                    val[1] = high | (val[0] & 0b11110000);
                    val[2] = low;
                }
            }
            LoadHigh(r1, value) => {
                let high = (value >> 4) as u8;
                let low = ((value & 0b1111) << 4) as u8;
                if r1.deref {
                    let (x, y) = Program::get_location(&self.registers[r1.value]);
                    let hex = self
                        .program
                        .get_pixel_mut_checked(x, y)
                        .ok_or(PixelNotPresent(x, y))?;
                    hex.0[0] = high;
                    hex.0[1] = low | (hex.0[1] & 0b1111);
                } else {
                    let val = &mut self.registers[r1.value];
                    val[0] = high;
                    val[1] = low | (val[1] & 0b1111);
                }
            }
            Move(r1, r2) => {
                let value = if r2.deref {
                    let (x, y) = Program::get_location(&self.registers[r2.value]);
                    self.program
                        .get_pixel_checked(x, y)
                        .ok_or(PixelNotPresent(x, y))?
                        .to_owned()
                } else {
                    self.registers[r2.value]
                };

                if r1.deref {
                    let (x, y) = Program::get_location(&self.registers[r1.value]);
                    let hex = self
                        .program
                        .get_pixel_mut_checked(x, y)
                        .ok_or(PixelNotPresent(x, y))?;
                    *hex = value;
                } else {
                    self.registers[r1.value] = value;
                }
            }
            _ => (),
        }
        Ok(())
    }

    fn get_hex_slice(hex: &Hex, start: u8, end: u8) -> u32 {
        hex.0.as_bits::<Lsb0>()[start as usize..end as usize].load::<u32>()
    }

    fn get_register(hex: &Hex, pos: usize) -> Register {
        let pos = pos * 6;
        let bits = hex.0.as_bits::<Lsb0>()[pos..pos + 6].load::<u8>();
        bits.into()
    }

    fn get_location(hex: &Hex) -> (u32, u32) {
        let bytes = hex.0;
        let x = (bytes[0] as u32) << 4 | (bytes[1] as u32) >> 4;
        let y = ((bytes[1] as u32) & 0b1111) << 8 | (bytes[2] as u32);
        (x, y)
    }

    // First 12 bits for x, last 12 bits for y
    fn iptr(&self) -> (u32, u32) {
        Program::get_location(&self.registers[31])
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
