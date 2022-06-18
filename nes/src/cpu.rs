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
    pub fn interpret(
        &mut self,
        program: &[u8]
    ) {
        self.regs.zero_counter();

        'root: loop {
            let opcode = program[self.regs.program_counter as usize];
            self.regs.inc_pc();

            match opcode {
                // LDA
                0xA9 => {
                    let acc = program[self.regs.program_counter as usize];
                    self.regs.inc_pc();

                    self.regs.accumulator = acc;
                    self.modify_result(acc as u16);
                }

                // TAX
                0xAA => {
                    self.regs.index_x = self.regs.accumulator;
                    self.modify_result(self.regs.accumulator as u16);
                }

                0x00 => {
                    break 'root;
                }

                _ => panic!("Illegal instruction (0x{:x})", opcode)
            }
        }
    }

    fn modify_result(&mut self, by: u16) {
        self.flags.modify_by_predicate(CpuFlags::ZERO, zero_predicate!(by));
        self.flags.modify_by_predicate(CpuFlags::NEGATIVE, neg_predicate!(by));
    }

    pub fn new() -> Self {
        Self { mem: MemoryMapping::new()
             , flags: CpuFlags::zero()
             , regs: CpuRegisters::new() }
    }
}
