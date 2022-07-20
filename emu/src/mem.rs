pub type Byte  = u8;
pub type Word  = u16;
pub type DWord = u16;

pub struct Memory {
    inner: Vec<Byte>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddrMode {
    Immediate,
    ZeroPage,

    ZeroPageX,
    ZeroPageY,

    Relative,
    Absolute,

    AbsoluteX,
    AbsoluteY,

    Indirect,

    IndirectX,
    IndirectY,

    Accumulator,
}

impl Memory {
    pub fn read_word(&self, addr: Word) -> Word {
        (self.read(addr + 1) as Word) << 8 | (self.read(addr) as Word)
    }

    pub fn write_word(&mut self, addr: Word, data: Word) {
        self.write(addr, (data & 0xff) as u8);
        self.write(addr + 1, (data >> 8) as u8);
    }

    #[inline(always)]
    pub fn read(&self, addr: Word) -> Byte {
        self.inner[addr as usize]
    }

    #[inline(always)]
    pub fn write(&mut self, addr: Word, data: Byte) {
        self.inner[addr as usize] = data;
    }

    pub fn copy_from(
        &self,
        addr: Word,
        out: &mut [u8]
    ) {
        todo!()
    }

    pub fn copy_to(
        &mut self,
        addr: Word,
        data: &[u8]
    ) {
        let len = data.len();
        let abs = addr as usize;

        self.inner[abs..abs + len]
            .copy_from_slice(data)
    }

    pub fn new(mem: Vec<u8>) -> Self {
        Self { inner: mem }
    }

    pub fn zeroed() -> Self {
        Self { inner: vec![0; 0xFF_FF] }
    }
}
