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

    /// Get program rom span access
    pub fn program_rom_span(&mut self) -> &mut [u8] {
        &mut self.inner[0x8001..=0xFFFF]
    }

    /// Get save span
    pub fn save_span(&mut self) -> &mut [u8] {
        &mut self.inner[0x6001..=0x8000]
    }

    /// Get Expansion ROM Span
    pub fn expansion_rom_span(&mut self) -> &mut [u8] {
        &mut self.inner[0x4021..=0x6000]
    }

    /// Get IO Memory span
    pub fn io_span(&mut self) -> &mut [u8] {
        &mut self.inner[0x2001..=0x4020]
    }

    /// Get CPU Memory span
    pub fn cpu_span(&mut self) -> &mut [u8] {
        &mut self.inner[0x0000..=0x2000]
    }

    pub fn new() -> Self {
        Self { inner: vec![0; 0xFFFF] }
    }
}
