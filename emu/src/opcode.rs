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
    Lda(AddrMode, Word),
}

static __INITIALIZED: AtomicBool = AtomicBool::new(false);
static mut OPCODES: [Opcode; 0xff] = [Opcode::Uninitialized; 0xff];

#[inline(always)]
pub fn lookup_opcode(code: Byte) -> Opcode {
    unsafe { OPCODES[code as usize] }
}

unsafe fn init() {
    OPCODES[0x00] = Opcode::Brk;

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
