use {
    crate::{
        cpu::*,
        status::*,
    }
};

// ADC

#[test]
fn adc_imm() {
    let mut cpu = Cpu::default();
    cpu.interpret([
        0xA9, 0x1,   // lda 0x1 (imm)
        0x69, 0xFF,  // adc 0xFF (imm)
        
        // Now accumulator should be zero
        // and carry flag is on, let's check

        0x69, 0x1,   // adc 0x1 (imm)

        0x00,        // brk
    ]);

    assert_eq!(cpu.acc, 0x2);
}

// TAX & TAY

#[test]
fn tax() {
    let mut cpu = Cpu::default();

    cpu.interpret([
        0xA9, 0xFF, 0xAA, 0x00
    ]);

    assert_eq!(cpu.x, 0xFF);
}

#[test]
fn tay() {
    let mut cpu = Cpu::default();

    cpu.interpret([
        0xA9, 0xFF, 0xA8, 0x00
    ]);

    assert_eq!(cpu.y, 0xFF);
}

// STY & STX

#[test]
fn stx() {
    let mut cpu = Cpu::default();

    cpu.interpret([
        0xE8, 0xE8, 0x86, 0x0F, 0x00
    ]);

    assert_eq!(cpu.mem.read(0x0F), 0x02);
}

#[test]
fn sty() {
    let mut cpu = Cpu::default();

    cpu.interpret([
        0xC8, 0xC8, 0x84, 0xAF, 0x00,
    ]);

    assert_eq!(cpu.mem.read(0xAF), 0x02);
}

// STA

#[test]
fn sta() {
    let mut cpu = Cpu::default();

    cpu.interpret([
        0xA9, 0xFF, 0x85, 0xFE, 0x00
    ]);

    assert_eq!(cpu.mem.read(0xFE), 0xFF);
}

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
