#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    Immediate,

    ZeroPage,
    ZeroPageX,
    ZeroPageY,

    Absolute,
    AbsoluteX,
    AbsoluteY,

    Indirect,
    IndirectX,
    IndirectY,
}

#[derive(Debug, Clone)]
pub struct NesMemory {
    inner: Vec<u8>,
}

impl NesMemory {
    pub fn bulk_write(&mut self, addr: u16, bulk: &[u8]) -> &mut Self {
        let addr = addr as usize;

        self.inner[addr .. (addr + bulk.len())]
            .copy_from_slice(bulk);
        self
    }

    pub fn bulk_write_zp(&mut self, offset: u8, bulk: &[u8]) -> &mut Self {
        self.bulk_write(offset as u16, bulk)
    }

    pub fn write_u16(&mut self, addr: u16, value: u16) -> &mut Self {
        self.inner[addr as usize] = (value & 0xff) as u8;
        self.inner[addr.wrapping_add(1) as usize] = (value >> 8) as u8;

        self
    }

    pub fn write_u16_zp(&mut self, offset: u8, value: u16) -> &mut Self {
        self.inner[offset as usize] = (value & 0xff) as u8;
        self.inner[offset.wrapping_add(1) as usize] = (value >> 8) as u8;

        self
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.inner[addr as usize] = value;
    }

    pub fn write_zp(&mut self, offset: u8, value: u8) {
        self.inner[offset as usize] = value;
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        let f = self.inner[addr as usize] as u16;
        let l = self.inner[addr.wrapping_add(1) as usize] as u16;

        (l << 8) | f
    }

    pub fn read_u16_zp(&self, offset: u8) -> u16 {
        self.read_u16(offset as u16)
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.inner[addr as usize]
    }

    pub fn read_zp(&self, offset: u8) -> u8 {
        self.inner[offset as usize]
    }

    pub fn new() -> Self {
        Self { inner: vec![0; 0xFFFF] }
    }
}
