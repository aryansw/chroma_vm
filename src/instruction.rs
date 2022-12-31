use bitvec::{field::BitField, prelude::*};

pub enum Instruction {
    ReadSmallValue(Register, u16),
    ReadFullValue(Register, u32),
    RegisterCopy(Register, u16),
    Halt,
}

pub struct Register {
    reference: bool,
    value: u8,
}

impl Into<Register> for u8 {
    fn into(self) -> Register {
        let (x, y) = self.view_bits::<Lsb0>().split_at(1);
        Register {
            reference: x[0],
            value: y.load_le(),
        }
    }
}
