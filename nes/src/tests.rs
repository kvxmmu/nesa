use {
    crate::{
        cpu::*,
        cpu_status::*,
    }
};

#[test]
fn stx() {
    let mut cpu = NesCpu::new();

    cpu.interpret([
        0xE8, 0xE8, 0x86, 0x0F, 0x00
    ]);

    assert_eq!(cpu.mem.read(0x0F), 0x02);
}

#[test]
fn sty() {
    let mut cpu = NesCpu::new();

    cpu.interpret([
        0xC8, 0xC8, 0x84, 0xAF, 0x00,
    ]);

    assert_eq!(cpu.mem.read(0xAF), 0x02);
}

#[test]
fn sta() {
    let mut cpu = NesCpu::new();

    cpu.interpret([
        0xA9, 0xFF, 0x85, 0xFE, 0x00
    ]);

    assert_eq!(cpu.mem.read(0xFE), 0xFF);
}

#[test]
fn lda_abs_x() {
    let mut cpu = NesCpu::new();

    cpu.mem.write(0x0AFF, 0xAE);
    cpu.interpret([
        0xE8, 0xBD, 0xFE, 0x0A, 0x00
    ]);

    assert_eq!(cpu.regs.acc, 0xAE);
}

#[test]
fn lda_abs_y() {
    let mut cpu = NesCpu::new();

    cpu.mem.write(0x0AFF, 0xAE);
    cpu.interpret([
        0xC8, 0xB9, 0xFE, 0x0A, 0x00,
    ]);

    assert_eq!(cpu.regs.acc, 0xAE);
}

#[test]
fn lda_indirect() {
    let mut cpu = NesCpu::new();

    cpu.mem.write_u16(0xF, 0xAFF);
    cpu.mem.write(0xAFF, 0xFF);

    cpu.interpret([
        0xA1, 0xF, 0x00
    ]);

    assert_eq!(cpu.regs.acc, 0xFF);
}

#[test]
fn lda_indirect_x() {
    let mut cpu = NesCpu::new();

    cpu.mem.write_u16(0x10, 0xAFF);
    cpu.mem.write(0xAFF, 0xA);

    cpu.interpret([
        0xE8, 0xA1, 0x0F, 0x00
    ]);

    assert_eq!(cpu.regs.acc, 0xA);
}

#[test]
fn lda_indirect_y() {
    let mut cpu = NesCpu::new();

    cpu.mem.write_u16(0xFF, 0xAFF);
    cpu.mem.write(0xB00, 0xAE);

    cpu.interpret([
        0xC8, 0xB1, 0xFF, 0x00,
    ]);

    assert_eq!(cpu.regs.acc, 0xAE);
}

#[test]
fn inx() {
    let mut cpu = NesCpu::new();
    cpu.interpret([
        0xE8, 0xE8, 0x00
    ]);

    assert_eq!(cpu.regs.x, 0x2);
}

#[test]
fn iny() {
    let mut cpu = NesCpu::new();

    cpu.interpret([
        0xC8, 0xC8, 0x00
    ]);

    assert_eq!(cpu.regs.y, 0x2);
}

#[test]
fn lda_abs_neg_zero() {
    let mut cpu = NesCpu::new();
    cpu.mem.write(0xAFF, 0xFF);

    cpu.interpret([
        0xAD, 0xFF, 0x0A, 0x00
    ]);

    assert_eq!(cpu.regs.acc, 0xFF);
    assert_eq!(cpu.status.by_index(CpuStatus::ZERO), false);
    assert_eq!(cpu.status.by_index(CpuStatus::NEGATIVE), true);
}

#[test]
fn lda_imm() {
    let mut cpu = NesCpu::new();
    cpu.interpret([
        0xA9, 0x0F, 0x00,
    ]);

    assert_eq!(cpu.regs.acc, 0x0F);

    assert_eq!(cpu.status.by_index(CpuStatus::ZERO), false);
    assert_eq!(cpu.status.by_index(CpuStatus::NEGATIVE), false);
}

#[test]
fn lda_imm_zero() {
    let mut cpu = NesCpu::new();
    cpu.interpret([
        0xA9, 0x00, 0x00
    ]);

    assert_eq!(cpu.regs.acc, 0);

    assert_eq!(cpu.status.by_index(CpuStatus::ZERO), true);
    assert_eq!(cpu.status.by_index(CpuStatus::NEGATIVE), false);
}

#[test]
fn lda_imm_neg() {
    let mut cpu = NesCpu::new();
    cpu.interpret([
        0xA9, 0xFF, 0x00
    ]);

    assert_eq!(cpu.regs.acc, 0xFF);
    assert_eq!(cpu.status.by_index(CpuStatus::ZERO), false);
    assert_eq!(cpu.status.by_index(CpuStatus::NEGATIVE), true);
}
