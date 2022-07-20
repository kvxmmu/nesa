use {
    crate::{
        cpu::*,
        status::*,
    }
};

#[test]
fn lda_imm() {
    let mut cpu = Cpu::default();
    cpu.interpret([
        0xA9, 0x0F, 0x00,
    ]);

    assert_eq!(cpu.acc, 0x0F);

    assert_eq!(cpu.status.fetch(CpuStatus::ZERO), false);
    assert_eq!(cpu.status.fetch(CpuStatus::NEGATIVE), false);
}
