use {
    crate::{
        mem::*,
        cpu_flags::*,
        cpu_registers::*,
    }
};

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
                0xA9 => {
                    let acc = program[self.regs.program_counter as usize];
                    self.regs.inc_pc();

                    self.regs.accumulator = acc;
                    dbg!(acc);

                    self.flags.modify_by_predicate(
                        CpuFlags::ZERO,
                        |_| if self.regs.accumulator == 0 {
                            Bit::One
                        } else {
                            Bit::Zero
                        }
                    );
                    self.flags.modify_by_predicate(
                        CpuFlags::NEGATIVE,
                        |_| if self.regs.accumulator & (1 << 7) != 0 {
                            Bit::One
                        } else {
                            Bit::Zero
                        }
                    );
                }

                0x00 => {
                    break 'root;
                }

                _ => panic!("Illegal instruction (0x{:x})", opcode)
            }
        }
    }

    pub fn new() -> Self {
        Self { mem: MemoryMapping::new()
             , flags: CpuFlags::zero()
             , regs: CpuRegisters::new() }
    }
}
