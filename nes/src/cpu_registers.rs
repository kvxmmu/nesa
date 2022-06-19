#[derive(Debug, Clone)]
pub struct CpuRegisters {
    pub pc: u16,
    pub sp: u8,

    pub x: u8,
    pub y: u8,

    pub acc: u8,
}

impl CpuRegisters {    
    pub fn add_pc(&mut self, value: u16) -> &mut Self {
        self.pc += value;
        self
    }

    pub fn add_sp(&mut self, value: u8) -> &mut Self {
        self.sp += value;
        self
    }

    pub fn inc_pc(&mut self) -> &mut Self {
        self.add_pc(1)
    }

    pub fn inc_sp(&mut self) -> &mut Self {
        self.add_sp(1)
    }

    pub fn new() -> Self {
        Self { pc: 0
             , sp: 0
             , x: 0
             , y: 0
             , acc: 0 }
    }
}
