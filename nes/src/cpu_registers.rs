#[derive(Debug)]
pub struct CpuRegisters {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub index_x: u8,
    pub index_y: u8,
}

impl CpuRegisters {
    pub fn add_sp(&mut self, value: u8) -> u8 {
        self.stack_pointer += value;
        self.stack_pointer
    }

    pub fn add_pc(&mut self, value: u16) -> u16 {
        self.program_counter += value;
        self.program_counter
    }

    pub fn inc_pc(&mut self) -> u16 {
        self.add_pc(1)
    }

    pub fn inc_sp(&mut self) -> u8 {
        self.add_sp(1)
    }

    pub fn new() -> Self {
        Self { program_counter: 0
             , stack_pointer: 0
             , accumulator: 0
             , index_x: 0
             , index_y: 0 }
    }

    pub fn zero_stack(&mut self) {
        self.stack_pointer = 0;
    }

    pub fn zero_accumulator(&mut self) {
        self.accumulator = 0;
    }

    pub fn zero_index_x(&mut self) {
        self.index_x = 0;
    }

    pub fn zero_index_y(&mut self) {
        self.index_y = 0;
    }

    pub fn zero_indexes(&mut self) {
        self.zero_index_x();
        self.zero_index_y();
    }

    pub fn zero_counter(&mut self) {
        self.program_counter = 0;
    }
}
