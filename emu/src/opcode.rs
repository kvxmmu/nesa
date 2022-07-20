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
    OPCODES[0xA9] = Opcode::Lda(AddrMode::Immediate, 1);
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
