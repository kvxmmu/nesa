use {
    crate::{
        opcode::*,
        memory::AddressingMode,
    }
};

pub static OPCODES: [Opcode; 26] = [
    // TAX
    Opcode::zero_offset(0xAA, OpcodeType::Tax),

    // TAY
    Opcode::zero_offset(0xA8, OpcodeType::Tay),

    // STA
    Opcode::new(0x85, 1, OpcodeType::Sta, AddressingMode::ZeroPage),
    Opcode::new(0x95, 1, OpcodeType::Sta, AddressingMode::ZeroPageX),

    Opcode::new(0x8D, 2, OpcodeType::Sta, AddressingMode::Absolute),
    Opcode::new(0x9D, 2, OpcodeType::Sta, AddressingMode::AbsoluteX),
    Opcode::new(0x99, 2, OpcodeType::Sta, AddressingMode::AbsoluteY),

    Opcode::new(0x81, 1, OpcodeType::Sta, AddressingMode::IndirectX),
    Opcode::new(0x91, 1, OpcodeType::Sta, AddressingMode::IndirectY),

    // STX
    Opcode::new(0x86, 1, OpcodeType::Stx, AddressingMode::ZeroPage),
    Opcode::new(0x96, 1, OpcodeType::Stx, AddressingMode::ZeroPageY),
    Opcode::new(0x8E, 2, OpcodeType::Stx, AddressingMode::Absolute),

    // STX
    Opcode::new(0x84, 1, OpcodeType::Sty, AddressingMode::ZeroPage),
    Opcode::new(0x94, 1, OpcodeType::Sty, AddressingMode::ZeroPageY),
    Opcode::new(0x8C, 2, OpcodeType::Sty, AddressingMode::Absolute),

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
