pub enum Instruction {
    Read,
    Halt,
}

pub struct Register {
    pub value: u8,
    pub reference: bool,
}
