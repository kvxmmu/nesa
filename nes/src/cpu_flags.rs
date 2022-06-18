#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Bit {
    One = 1,
    Zero = 0,
}

pub struct CpuFlags {
    inner: u8,
}

impl CpuFlags {
    pub fn by_index(&self, index: u8) -> Bit {
        Bit::from(((self.inner & (1 << index)) != 0) as u8) 
    }

    pub fn modify_bit(
        &mut self,
        index: u8,
        to: Bit
    ) {
        /* Equal to
         * match to {
         *     Bit::One  => self.inner |= 1 << index,
         *     Bit::Zero => self.inner &= !(1 << index),
         * }
         */
        self.inner = (self.inner & !(1 << index)) | ((to as u8) << index);
    }

    pub fn modify_by_predicate<T: Fn(Bit) -> Bit>(
        &mut self,
        index: u8,
        predicate: T
    ) {
        self.modify_bit(index, predicate(self.by_index(index)));
    }

    pub fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub fn zero() -> Self {
        Self { inner: 0 }
    }
}

impl Bit {
    pub fn on(self) -> bool {
        self == Bit::One
    }

    pub fn off(self) -> bool {
        self == Bit::Zero
    }
}

impl From<u8> for Bit {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Zero,
            1 => Self::One,

            value => panic!("Illegal bit field, expected 1 or 0, got: {}", value)
        }
    }
}

impl CpuFlags {
    pub const CARRY: u8       = 7;
    pub const ZERO: u8        = 6;
    pub const INTERRUPT: u8   = 5;
    pub const DECIMAL: u8     = 4;
    pub const BREAK: u8       = 3;

    /* *Gap* */

    pub const OVERFLOW: u8    = 1;
    pub const NEGATIVE: u8    = 0;
}
