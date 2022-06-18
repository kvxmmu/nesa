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
    pub fn test_lda_load_flag() {
        let mut cpu = NesCpu::new();
        cpu.interpret(&[0xA9, 0x00, 0x00]);
    
        assert_eq!(cpu.flags.by_index(CpuFlags::ZERO), Bit::One);
    }
    
}
