pub const NEG_MASK: u8 = 1 << 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecStatus {
    Executing,
    Exit,
}

#[derive(Debug)]
pub struct CpuStatus {
    flags: u8
}

impl CpuStatus {
    #[inline(always)]
    pub fn fetch(&self, mask: u8) -> bool {
        (self.flags & mask) == mask
    }

    #[inline(always)]
    pub fn set_on(&mut self, mask: u8) {
        self.flags |= mask;
    }

    #[inline(always)]
    pub fn set_off(&mut self, mask: u8) {
        self.flags &= !mask;
    }

    pub fn new(flags: u8) -> Self {
        Self { flags }
    }
}

impl CpuStatus {
    pub const CARRY: u8       = 1 << 7;
    pub const ZERO: u8        = 1 << 6;
    pub const INTERRUPT: u8   = 1 << 5;
    pub const DECIMAL: u8     = 1 << 4;
    pub const BREAK: u8       = 1 << 3;

    /* *Gap* */

    pub const OVERFLOW: u8    = 1 << 1;
    pub const NEGATIVE: u8    = 1 << 0;
}

impl Default for CpuStatus {
    fn default() -> Self {
        Self::new(0)
    }
}
