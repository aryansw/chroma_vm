use bitvec::{field::BitField, prelude::*};

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    // Moving
    LoadLow(Register, Immediate),
    LoadHigh(Register, Immediate),
    Move(Register, Register),
    // Arithmetic
    Add(Register, Register, Register),
    Subtract(Register, Register, Register),
    Multiply(Register, Register, Register),
    Divide(Register, Register, Register),
    Modulo(Register, Register, Register),
    And(Register, Register, Register),
    Or(Register, Register, Register),
    // Comparisons
    Equal(Register, Register, Register),
    NotEqual(Register, Register, Register),
    GreaterThan(Register, Register, Register),
    LessThan(Register, Register, Register),
    GreaterThanEqual(Register, Register, Register),
    LessThanEqual(Register, Register, Register),
    // IO / Memory
    Alloc(Register, Register),
    MemCopy(Register, Register, Register),
    CurrAddress(Register),
    // Jumps and Calls
    Jump(Register),
    JumpIf(Register, Register),
    Call(Register),
    CallIf(Register, Register),
    Return,
    Halt,
    // Stack
    Push(Register),
    Pop(Register),
}

pub type Immediate = u16;

#[derive(Debug, Clone, Copy)]
pub struct Register {
    pub deref: bool,
    pub value: usize,
}

impl Into<Register> for u8 {
    fn into(self) -> Register {
        let (x, y) = self.view_bits::<Lsb0>().split_at(1);
        Register {
            deref: x[0],
            value: y.load_le(),
        }
    }
}
