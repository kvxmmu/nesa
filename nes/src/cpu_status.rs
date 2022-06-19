#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpuStatus {
    inner: u8,
}

impl CpuStatus {
    pub fn by_index(&mut self, index: u8) -> bool {
        let mask = 1 << index;
        (self.inner & (1 << index)) == mask
    }

    pub fn turn_off(&mut self, index: u8) {
        self.inner &= !(1 << index);
    }

    pub fn turn_on(&mut self, index: u8) {
        self.inner |= 1 << index;
    }

    pub fn turn<T: Fn() -> bool>(
        &mut self,
        index: u8,
        predicate: T
    ) -> u8 {
        if predicate() {
            self.turn_on(index);
        } else {
            self.turn_off(index);
        }

        self.inner
    }

    pub fn new() -> Self {
        Self { inner: 0 }
    }
}

impl CpuStatus {
    pub const CARRY: u8       = 7;
    pub const ZERO: u8        = 6;
    pub const INTERRUPT: u8   = 5;
    pub const DECIMAL: u8     = 4;
    pub const BREAK: u8       = 3;

    /* *Gap* */

    pub const OVERFLOW: u8    = 1;
    pub const NEGATIVE: u8    = 0;
}
