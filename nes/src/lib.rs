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
    fn all_3_ops() {
        let mut cpu = NesCpu::new();
        cpu.interpret(&[0xA9, 0xA, 0xAA, 0x00]);

        assert_eq!(cpu.flags.by_index(CpuFlags::ZERO), Bit::Zero);
        assert_eq!(cpu.flags.by_index(CpuFlags::NEGATIVE), Bit::Zero);
        assert_eq!(cpu.regs.accumulator, 0xA);
        assert_eq!(cpu.regs.index_x, 0xA);
    }

    #[test]
    fn overflowing_inx() {
        let mut cpu = NesCpu::new();
        cpu.interpret(&[0xA9, 0xFF, 0xAA, 0xE8, 0xE8, 0x00]);

        assert_eq!(cpu.regs.index_x, 1);
    }

    #[test]
    fn lda_zero_page() {
        let mut cpu = NesCpu::new();
        cpu.mem.write(0x10, 0xF);

        cpu.interpret(&[0xA5, 0x10, 0x00]);

        assert_eq!(cpu.regs.accumulator, 0xF);
    }

    #[test]
    fn lda_zero_page_x() {
        let mut cpu = NesCpu::new();
        cpu.mem.write(0x11, 0xF);

        cpu.interpret(&[0xA9, 0x1, 0xAA, 0xB5, 0x10, 0x00]);

        assert_eq!(cpu.regs.index_x, 0x1);
        assert_eq!(cpu.regs.accumulator, 0xF);
    }

    #[test]
    fn lda_absolute() {
        let mut cpu = NesCpu::new();
        cpu.mem.write(0x11, 0xF);

        cpu.interpret(&[0xAD, 0x11, 0x00, 0x00]);

        assert_eq!(cpu.regs.accumulator, 0xF);
    }

    #[test]
    fn lda_absolute_xy() {
        let mut cpu = NesCpu::new();
        cpu.mem.write(0x11, 0xF);

        cpu.interpret(&[0xA9, 0x1, 0xA8, 0xB9, 0x10, 0x00, 0x00]);

        assert_eq!(cpu.regs.accumulator, 0xF);
    }

    #[test]
    fn tay_copy() {
        let mut cpu = NesCpu::new();
        
        cpu.interpret(&[0xA9, 0xF, 0xA8, 0x00]);

        assert_eq!(cpu.regs.index_y, 0xF);
    }

    #[test]
    fn basic_inx() {
        let mut cpu = NesCpu::new();
        cpu.interpret(&[0xA9, 0xA, 0xAA, 0xE8, 0xE8, 0xE8, 0x00]);

        assert_eq!(cpu.regs.index_x, 0xD);
    }

    #[test]
    fn tax_copy() {
        let mut cpu = NesCpu::new();
        cpu.interpret(&[0xA9, 0xA, 0xAA, 0x00]);

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
