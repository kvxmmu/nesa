pub mod cpu;

pub mod cpu_registers;
pub mod cpu_flags;

pub mod mem;

#[cfg(test)]
mod tests {
    use {
        crate::{
            cpu::*,
            cpu_flags::*,
        }
    };

    #[test]
    fn tax_copy() {
        let mut cpu = NesCpu::new();
        cpu.regs.accumulator = 10;
        cpu.interpret(&[0xAA, 0x00]);

        assert_eq!(cpu.regs.index_x, 10);
    }
    
    #[test]
    fn lda_load_zero_flag() {
        let mut cpu = NesCpu::new();
        cpu.interpret(&[0xA9, 0x00, 0x00]);
    
        assert_eq!(cpu.flags.by_index(CpuFlags::ZERO), Bit::One);
    }

    #[test]
    fn lda_load_no_flags() {
        let mut cpu = NesCpu::new();
        cpu.interpret(&[0xA9, 0xA, 0x00]);

        assert_eq!(cpu.flags.by_index(CpuFlags::NEGATIVE), Bit::Zero);
        assert_eq!(cpu.flags.by_index(CpuFlags::ZERO), Bit::Zero);
    }

    #[test]
    fn lda_load_neg_flag() {
        let mut cpu = NesCpu::new();
        cpu.interpret(&[0xA9, 0xFF, 0x00]);

        assert_eq!(cpu.flags.by_index(CpuFlags::NEGATIVE), Bit::One);
    }
    
}
