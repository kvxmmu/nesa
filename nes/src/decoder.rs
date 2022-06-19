use {
    crate::{
        opcode::*,
        memory::AddressingMode,
    }
};

pub static OPCODES: [Opcode; 2] = [
    Opcode::new(0xA9, 1, OpcodeType::Lda, AddressingMode::Immediate),
    Opcode::zero_offset(0x00, OpcodeType::Brk),
];
