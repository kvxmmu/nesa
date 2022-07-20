use {
    std::sync::atomic::{
        Ordering,
        AtomicBool,
    },

    crate::{
        mem::*,
    }
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitializeState {
    AlreadyInitialized,
    Initialized,
}

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Uninitialized,
    Brk,

    Inx,
    Iny,

    Tax,
    Tay,

    Bcs,
    Bcc,

    Asl(AddrMode, Word),

    And(AddrMode, Word),

    Stx(AddrMode, Word),
    Sty(AddrMode, Word),
    Sta(AddrMode, Word),

    Adc(AddrMode, Word),

    Lda(AddrMode, Word),
}

static __INITIALIZED: AtomicBool = AtomicBool::new(false);
static mut OPCODES: [Opcode; 256] = [Opcode::Uninitialized; 256];

#[inline(always)]
pub fn lookup_opcode(code: Byte) -> Opcode {
    unsafe { OPCODES[code as usize] }
}

unsafe fn init() {
    OPCODES[0x00] = Opcode::Brk;

    // BCC

    OPCODES[0x90] = Opcode::Bcc;

    // BCS
    
    OPCODES[0xB0] = Opcode::Bcs;

    // ASL

    OPCODES[0x0A] = Opcode::Asl(AddrMode::Accumulator, 0);

    OPCODES[0x06] = Opcode::Asl(AddrMode::ZeroPage, 1);
    OPCODES[0x16] = Opcode::Asl(AddrMode::ZeroPageX, 1);

    OPCODES[0x0E] = Opcode::Asl(AddrMode::Absolute, 2);
    OPCODES[0x1E] = Opcode::Asl(AddrMode::AbsoluteX, 2);

    // AND

    OPCODES[0x29] = Opcode::And(AddrMode::Immediate, 1);

    OPCODES[0x25] = Opcode::And(AddrMode::ZeroPage, 1);
    OPCODES[0x35] = Opcode::And(AddrMode::ZeroPageX, 1);

    OPCODES[0x2D] = Opcode::And(AddrMode::Absolute, 2);
    OPCODES[0x3D] = Opcode::And(AddrMode::AbsoluteX, 2);
    OPCODES[0x39] = Opcode::And(AddrMode::AbsoluteY, 2);

    OPCODES[0x21] = Opcode::And(AddrMode::IndirectX, 1);
    OPCODES[0x31] = Opcode::And(AddrMode::IndirectY, 1);

    // ADC

    OPCODES[0x69] = Opcode::Adc(AddrMode::Immediate, 1);

    OPCODES[0x65] = Opcode::Adc(AddrMode::ZeroPage, 1);
    OPCODES[0x75] = Opcode::Adc(AddrMode::ZeroPageX, 1);
    
    OPCODES[0x6D] = Opcode::Adc(AddrMode::Absolute, 2);
    OPCODES[0x7D] = Opcode::Adc(AddrMode::AbsoluteX, 2);
    OPCODES[0x79] = Opcode::Adc(AddrMode::AbsoluteY, 2);

    OPCODES[0x61] = Opcode::Adc(AddrMode::IndirectX, 1);
    OPCODES[0x71] = Opcode::Adc(AddrMode::IndirectY, 1);

    // STA
    OPCODES[0x85] = Opcode::Sta(AddrMode::ZeroPage, 1);
    OPCODES[0x95] = Opcode::Sta(AddrMode::ZeroPageX, 1);

    OPCODES[0x8D] = Opcode::Sta(AddrMode::Absolute, 2);
    OPCODES[0x9D] = Opcode::Sta(AddrMode::AbsoluteX, 2);
    OPCODES[0x99] = Opcode::Sta(AddrMode::AbsoluteY, 2);
    
    OPCODES[0x81] = Opcode::Sta(AddrMode::IndirectX, 1);
    OPCODES[0x91] = Opcode::Sta(AddrMode::IndirectY, 1);

    // STY
    OPCODES[0x84] = Opcode::Sty(AddrMode::ZeroPage, 1);
    OPCODES[0x94] = Opcode::Sty(AddrMode::ZeroPageY, 1);
    OPCODES[0x8C] = Opcode::Sty(AddrMode::Absolute, 2);

    // STX
    OPCODES[0x86] = Opcode::Stx(AddrMode::ZeroPage, 1);
    OPCODES[0x96] = Opcode::Stx(AddrMode::ZeroPageY, 1);
    OPCODES[0x8E] = Opcode::Stx(AddrMode::Absolute, 2);

    // TAX
    OPCODES[0xAA] = Opcode::Tax;

    // TAY
    OPCODES[0xA8] = Opcode::Tay;

    // INX
    OPCODES[0xE8] = Opcode::Inx;

    // INY
    OPCODES[0xC8] = Opcode::Iny;

    // LDA
    OPCODES[0xA9] = Opcode::Lda(AddrMode::Immediate, 1);

    OPCODES[0xA5] = Opcode::Lda(AddrMode::ZeroPage, 1);
    OPCODES[0xB5] = Opcode::Lda(AddrMode::ZeroPageX, 1);
    
    OPCODES[0xAD] = Opcode::Lda(AddrMode::Absolute, 2);
    OPCODES[0xBD] = Opcode::Lda(AddrMode::AbsoluteX, 2);
    OPCODES[0xB9] = Opcode::Lda(AddrMode::AbsoluteY, 2);

    OPCODES[0xA1] = Opcode::Lda(AddrMode::IndirectX, 1);
    OPCODES[0xB1] = Opcode::Lda(AddrMode::IndirectY, 1);
}

pub fn try_init() -> InitializeState {
    if __INITIALIZED.compare_exchange(
        false,
        true,
        Ordering::SeqCst,
        Ordering::Relaxed
    ).is_ok() {
        unsafe { init() };
        InitializeState::Initialized
    } else {
        InitializeState::AlreadyInitialized
    }
}
