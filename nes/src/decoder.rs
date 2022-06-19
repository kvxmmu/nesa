use {
    crate::{
        opcode::*,
        memory::AddressingMode,
    }
};

pub static OPCODES: [Opcode; 11] = [
    // LDA
    Opcode::new(0xA9, 1, OpcodeType::Lda, AddressingMode::Immediate),

    Opcode::new(0xA5, 1, OpcodeType::Lda, AddressingMode::ZeroPage),
    Opcode::new(0xB5, 1, OpcodeType::Lda, AddressingMode::ZeroPageX),

    Opcode::new(0xAD, 2, OpcodeType::Lda, AddressingMode::Absolute),
    Opcode::new(0xBD, 2, OpcodeType::Lda, AddressingMode::AbsoluteX),
    Opcode::new(0xB9, 2, OpcodeType::Lda, AddressingMode::AbsoluteY),

    Opcode::new(0xA1, 1, OpcodeType::Lda, AddressingMode::IndirectX),
    Opcode::new(0xB1, 1, OpcodeType::Lda, AddressingMode::IndirectY),

    // INX
    Opcode::zero_offset(0xE8, OpcodeType::Inx),

    // INY
    Opcode::zero_offset(0xC8, OpcodeType::Iny),

    // BRK
    Opcode::zero_offset(0x00, OpcodeType::Brk),
];
