pub struct MemoryMapping {
    inner: Vec<u8>
}

impl MemoryMapping {
    pub fn global_mut(&mut self) -> &mut [u8] {
        &mut self.inner
    }

    pub fn global(&self) -> &[u8] {
        &self.inner
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.inner[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) -> u8 {
        self.inner[addr as usize] = value;
        value
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        let lhs = self.read(addr);
        let rhs = self.read(addr + 1);

        ((rhs as u16) << 8) | (lhs as u16)
    }

    pub fn write_u16(&mut self, addr: u16, value: u16) -> u16 {
        self.inner[addr as usize] = (value & 0xff) as u8;
        self.inner[(addr + 1) as usize] = (value >> 8) as u8;

        value
    }

    pub fn bulk_write(&mut self, addr: u16, bulk: &[u8]) -> u16 {
        let len = bulk.len() as u16;
        self.inner[addr as usize .. (addr + len) as usize]
            .copy_from_slice(bulk);

        len
    }

    pub fn new() -> Self {
        Self { inner: vec![0; 0xFFFF] }
    }
}
