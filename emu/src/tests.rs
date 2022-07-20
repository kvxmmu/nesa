use {
    crate::{
        cpu::*,
        status::*,
    }
};

#[test]
fn lda_abs_x() {
    let mut cpu = Cpu::default();

    cpu.mem.write(0x0AFF, 0xAE);
    cpu.interpret([
        0xE8, 0xBD, 0xFE, 0x0A, 0x00
    ]);

    assert_eq!(cpu.acc, 0xAE);
}

#[test]
fn lda_abs_y() {
    let mut cpu = Cpu::default();

    cpu.mem.write(0x0AFF, 0xAE);
    cpu.interpret([
        0xC8, 0xB9, 0xFE, 0x0A, 0x00,
    ]);

    assert_eq!(cpu.acc, 0xAE);
}

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