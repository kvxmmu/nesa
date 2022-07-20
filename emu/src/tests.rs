use {
    crate::{
        cpu::*,
        status::*,
    }
};

// LDA

#[test]
fn lda_abs_neg_zero() {
    let mut cpu = Cpu::default();
    cpu.mem.write(0xAFF, 0xFF);

    cpu.interpret([
        0xAD, 0xFF, 0x0A, 0x00
    ]);

    assert_eq!(cpu.acc, 0xFF);
    assert_eq!(cpu.status.fetch(CpuStatus::ZERO), false);
    assert_eq!(cpu.status.fetch(CpuStatus::NEGATIVE), true);
}

#[test]
fn lda_indirect() {
    let mut cpu = Cpu::default();

    cpu.mem.write_word(0xF, 0xAFF);
    cpu.mem.write(0xAFF, 0xFF);

    cpu.interpret([
        0xA1, 0xF, 0x00
    ]);

    assert_eq!(cpu.acc, 0xFF);
}

#[test]
fn lda_indirect_x() {
    let mut cpu = Cpu::default();

    cpu.mem.write_word(0x10, 0xAFF);
    cpu.mem.write(0xAFF, 0xA);

    cpu.interpret([
        0xE8, 0xA1, 0x0F, 0x00
    ]);

    assert_eq!(cpu.acc, 0xA);
}

#[test]
fn lda_indirect_y() {
    let mut cpu = Cpu::default();

    cpu.mem.write_word(0xFF, 0xAFF);
    cpu.mem.write(0xB00, 0xAE);

    cpu.interpret([
        0xC8, 0xB1, 0xFF, 0x00,
    ]);

    assert_eq!(cpu.acc, 0xAE);
}

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
