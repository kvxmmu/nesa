use {
    crate::{
        cpu::*,
        cpu_status::*,
    }
};

#[test]
fn test_lda_imm() {
    let mut cpu = NesCpu::new();
    cpu.interpret([
        0xA9, 0x0F, 0x00,
    ]);

    assert_eq!(cpu.regs.acc, 0x0F);

    assert_eq!(cpu.status.by_index(CpuStatus::ZERO), false);
    assert_eq!(cpu.status.by_index(CpuStatus::NEGATIVE), false);
}

#[test]
fn test_lda_imm_zero() {
    let mut cpu = NesCpu::new();
    cpu.interpret([
        0xA9, 0x00, 0x00
    ]);

    assert_eq!(cpu.regs.acc, 0);

    assert_eq!(cpu.status.by_index(CpuStatus::ZERO), true);
    assert_eq!(cpu.status.by_index(CpuStatus::NEGATIVE), false);
}

#[test]
fn test_lda_imm_neg() {
    let mut cpu = NesCpu::new();
    cpu.interpret([
        0xA9, 0xFF, 0x00
    ]);

    assert_eq!(cpu.regs.acc, 0xFF);
    assert_eq!(cpu.status.by_index(CpuStatus::ZERO), false);
    assert_eq!(cpu.status.by_index(CpuStatus::NEGATIVE), true);
}
