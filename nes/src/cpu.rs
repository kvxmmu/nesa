use {
    crate::{
        mem::*,
        cpu_flags::*,
        cpu_registers::*,
    },
};

macro_rules! zero_predicate {
    ($by:expr) => {
        |_| if $by == 0 {
            Bit::One
        } else {
            Bit::Zero
        }
    };
}

macro_rules! neg_predicate {
    ($by:expr) => {
        |_| if ($by & (1 << 7)) != 0 {
            Bit::One
        } else {
            Bit::Zero
        }
    };
}

pub struct NesCpu {
    pub mem: MemoryMapping,
    pub flags: CpuFlags,
    pub regs: CpuRegisters,
}

impl NesCpu {
    fn check_overflow(&mut self, overflowing: bool) -> bool {
        self.flags.modify_by_predicate(CpuFlags::OVERFLOW, |_| if overflowing {
            Bit::One
        } else {
            Bit::Zero
        });
        overflowing
    }

    fn check_zero_or_neg(&mut self, by: u16) {
        self.flags.modify_by_predicate(CpuFlags::ZERO, zero_predicate!(by));
        self.flags.modify_by_predicate(CpuFlags::NEGATIVE, neg_predicate!(by));
    }

    fn fetch(&mut self) -> u8 {
        self.regs.inc_pc();
        self.mem.read(self.regs.program_counter - 1)
    }

    pub fn reset(
        &mut self
    ) {
        self.regs.reset();
        self.regs.program_counter = self.mem.read_u16(0xFFFC);
    }

    pub fn load(
        &mut self,
        program: &[u8]
    ) {
        self.mem.bulk_write(0x8000, program);
        self.mem.write_u16(0xFFFC, 0x8000);
    }

    pub fn new() -> Self {
        Self { mem: MemoryMapping::new()
             , flags: CpuFlags::zero()
             , regs: CpuRegisters::new() }
    }
}

impl NesCpu {
    pub fn lda(&mut self, mode: Addressing) {
        let addr = self.get_operand_address(mode);
        let acc = self.mem.read(addr);

        self.regs.accumulator = acc;
        self.check_zero_or_neg(acc as u16);
    }
}

impl NesCpu {
    pub fn interpret(
        &mut self,
        program: &[u8]
    ) {
        self.load(program);
        self.reset();

        'root: loop {
            let opcode = self.fetch();

            match opcode {
                // LDA (Immediate)
                0xA9 => {
                    self.lda(Addressing::Immediate);
                    self.regs.add_pc(1);
                }

                // LDA (Zero page)
                0xA5 => {
                    self.lda(Addressing::ZeroPage);
                    self.regs.add_pc(1);
                }

                // LDA (Zero page X)
                0xB5 => {
                    self.lda(Addressing::ZeroPageX);
                    self.regs.add_pc(1);
                }

                // LDA (Absolute)
                0xAD => {
                    self.lda(Addressing::Absolute);
                    self.regs.add_pc(2);
                }

                // LDA (Absolute X)
                0xBD => {
                    self.lda(Addressing::AbsoluteX);
                    self.regs.add_pc(2);
                }

                // LDA (Absolute Y)
                0xB9 => {
                    self.lda(Addressing::AbsoluteY);
                    self.regs.add_pc(2);
                }

                0xA1 => {
                    self.lda(Addressing::IndirectX);
                    self.regs.add_pc(1);
                }

                0xB1 => {
                    self.lda(Addressing::IndirectY);
                    self.regs.add_pc(1);
                }

                // TAX
                0xAA => {
                    self.regs.index_x = self.regs.accumulator;
                    self.check_zero_or_neg(self.regs.accumulator as u16);
                }

                // TAY
                0xA8 => {
                    self.regs.index_y = self.regs.accumulator;
                    self.check_zero_or_neg(self.regs.accumulator as u16);
                }

                // INX
                0xE8 => {
                    let result = self.regs.index_x.wrapping_add(1);
                    self.regs.index_x = result;

                    self.check_zero_or_neg(result as u16);
                }

                // BRK
                0x00 => {
                    break 'root;
                }

                _ => panic!("Illegal instruction (0x{:x})", opcode)
            }
        }
    }

    pub fn get_operand_address(&self, mode: Addressing) -> u16 {
        match mode {
            Addressing::Immediate => self.regs.program_counter,
            Addressing::ZeroPage  => self.mem.read(self.regs.program_counter) as u16,

            Addressing::Absolute  => self.mem.read_u16(self.regs.program_counter),
            Addressing::AbsoluteX => self.mem.read_u16(self.regs.program_counter).wrapping_add(
                self.regs.index_x as u16),
            Addressing::AbsoluteY => self.mem.read_u16(self.regs.program_counter).wrapping_add(
                self.regs.index_y as u16),

            Addressing::ZeroPageX => self.mem.read(self.regs.program_counter).wrapping_add(self.regs.index_x) as u16,
            Addressing::ZeroPageY => self.mem.read(self.regs.program_counter).wrapping_add(self.regs.index_y) as u16,

            Addressing::Indirect => {
                let zero_page_addr = self.mem.read(self.regs.program_counter);
                let global_address = self.mem.read_u16(zero_page_addr as u16);

                global_address
            }

            Addressing::IndirectX => {
                let zero_page_addr = self.mem.read(self.regs.program_counter).wrapping_add(self.regs.index_x);
                let global_address = self.mem.read_u16(zero_page_addr as u16);

                global_address
            }

            Addressing::IndirectY => {
                let zero_page_addr = self.mem.read(self.regs.program_counter) as u16;
                let global_address = self.mem.read_u16(zero_page_addr);

                global_address.wrapping_add(self.regs.index_y as u16)
            }
        }
    }
}
