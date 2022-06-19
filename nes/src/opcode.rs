use {
    crate::memory::AddressingMode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpcodeType {
    /// Load accumulator
    Lda,

    /// Increment X
    Inx,

    /// Increment Y
    Iny,

    /// Stop execution
    Brk,
}

#[derive(Debug, Clone)]
pub struct Opcode {
    pub hex: u8,
    pub pc_offset: u16,

    pub opcode: OpcodeType,
    pub mode: AddressingMode,
}

impl Opcode {
    pub const fn zero_offset(
        hex: u8,
        opcode: OpcodeType
    ) -> Self {
        Self::new(hex, 0, opcode, AddressingMode::Immediate)
    }

    pub const fn new(
        hex: u8,
        pc_offset: u16,
        opcode: OpcodeType,
        mode: AddressingMode,
    ) -> Self {
        Self { hex
             , opcode
             , mode
             , pc_offset }
    }
}
