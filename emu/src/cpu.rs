use {
    crate::{
        consts::*,
        mem::*,
        status::*,
        opcode::*,
        error::*,
    },

    std::{
        ops::BitAnd
    },
    num::Integer
};

pub struct Cpu {
    pub pc: Word,

    pub x: Byte,
    pub y: Byte,

    pub acc: Byte,

    pub mem: Memory,
    pub status: CpuStatus
}

impl Cpu {
    pub fn inx(
        &mut self,
    ) {
        self.inc_x();
        self.chk_zero_neg_b(self.x);
    }

    pub fn tax(
        &mut self,
    ) {
        self.x = self.acc;
        self.chk_zero_neg_b(self.x);
    }

    pub fn tay(
        &mut self
    ) {
        self.y = self.acc;
        self.chk_zero_neg_b(self.y);
    }

    pub fn iny(
        &mut self
    ) {
        self.inc_y();
        self.chk_zero_neg_b(self.y);
    }

    pub fn lda(
        &mut self,
        mode: AddrMode
    ) {
        let target = self.translate(mode);
        let data = self.mem.read(target);

        self.acc = data;
        self.chk_zero_neg_b(data);
    }

    pub fn stx(
        &mut self,
        mode: AddrMode
    ) {
        let target = self.translate(mode);
        self.mem.write(target, self.x);
    }

    pub fn sty(
        &mut self,
        mode: AddrMode
    ) {
        let target = self.translate(mode);
        self.mem.write(target, self.y);
    }

    pub fn sta(
        &mut self,
        mode: AddrMode,
    ) {
        let target = self.translate(mode);
        self.mem.write(target, self.acc);
    }

    pub fn adc(
        &mut self,
        mode: AddrMode
    ) {
        let contents = self.mem.read(self.translate(mode)) as Word;
        let acc = self.acc as u16;
        let carry = self.status.fetch(CpuStatus::CARRY) as Word;

        let (am, overflow1) = contents.overflowing_add(acc);
        let (amc, overflow2) = am.overflowing_add(carry);

        if (amc & CARRY_MASK) == CARRY_MASK {
            self.status.set_on(CpuStatus::CARRY);
        } else {
            self.status.set_off(CpuStatus::CARRY);
        }

        self.acc = amc as u8;
        self.set_overflow(overflow1 || overflow2);
        self.chk_zero_neg_b(self.acc);
    }
}

impl Cpu {
    #[inline]
    pub fn exec(
        &mut self,
        opcode: Opcode
    ) -> Result<ExecStatus, ExecError> {
        match opcode {
            Opcode::Tax => self.tax(),
            Opcode::Tay => self.tay(),

            Opcode::Adc(mode, length) => {
                self.adc(mode);
                self.add_pc(length);
            }

            Opcode::Sta(mode, length) => {
                self.sta(mode);
                self.add_pc(length);
            }

            Opcode::Inx => self.inx(),
            Opcode::Iny => self.iny(),

            Opcode::Stx(mode, length) => {
                self.stx(mode);
                self.add_pc(length);
            }

            Opcode::Sty(mode, length) => {
                self.sty(mode);
                self.add_pc(length);
            }

            Opcode::Lda(mode, length) => {
                self.lda(mode);
                self.add_pc(length);
            }

            Opcode::Brk => return Ok(ExecStatus::Exit),
            Opcode::Uninitialized => return Err(ExecError::InvalidInstruction),
        }

        Ok(ExecStatus::Executing)
    }

    pub fn exec_until_brk(
        &mut self
    ) -> Option<ExecError> {
        loop {
            match self.exec_next() {
                Ok(ExecStatus::Exit) => break,
                Err(e) => return Some(e),

                Ok(ExecStatus::Executing) => continue,
            }
        }

        None
    }

    #[inline(always)]
    pub fn exec_next(&mut self) -> Result<ExecStatus, ExecError> {
        let opcode = self.next();
        self.exec(opcode)
    }

    #[inline(always)]
    pub fn next(&mut self) -> Opcode {
        let opcode = self.mem.read(self.pc);
        self.inc_pc();

        lookup_opcode(opcode)
    }

    pub fn try_interpret<T: AsRef<[u8]>>(
        &mut self,
        code: T,
    ) -> Option<ExecError> {
        self.reset_load_rom(code.as_ref());
        self.exec_until_brk()
    }

    pub fn interpret<T: AsRef<[u8]>>(
        &mut self,
        code: T,
    ) {
        if let Some(error) = self.try_interpret(code) {
            panic!("Interpret error: {:#?}", error);
        }
    }

    pub fn reset_load_rom(&mut self, rom: &[u8]) {
        self.reset();
        self.load_rom(rom);
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.mem.copy_to(ROM_ENTRYPOINT, rom);
        self.pc = ROM_ENTRYPOINT;
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.x = 0;
        self.y = 0;
        self.acc = 0;
    }

    pub fn new(
        pc: Word,
        x: Byte,
        y: Byte,
        acc: Byte,

        mem: Memory,
        status: CpuStatus
    ) -> Self {
        try_init();

        Self { x, y,
               mem, status,
               pc, acc, }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, Memory::zeroed(), CpuStatus::default())
    }
}

impl Cpu {
    #[inline(always)]
    pub fn inc_pc(&mut self) {
        self.add_pc(1);
    }

    #[inline(always)]
    pub fn inc_x(&mut self) {
        self.add_x(1);
    }

    #[inline(always)]
    pub fn inc_y(&mut self) {
        self.add_y(1);
    }

    #[inline(always)]
    pub fn add_x(&mut self, x: Byte) {
        self.x = self.x.wrapping_add(x);
    }

    #[inline(always)]
    pub fn add_y(&mut self, y: Byte) {
        self.y = self.y.wrapping_add(y);
    }

    #[inline(always)]
    pub fn add_pc(&mut self, acc: Word) {
        self.pc = self.pc.wrapping_add(acc);
    }

    #[inline(always)]
    pub fn chk_zero_neg_b(
        &mut self,
        value: Byte
    ) {
        self.chk_zero_neg(value, NEG_MASK);
    }

    #[inline(always)]
    pub fn set_overflow(&mut self, to: bool) {
        if to {
            self.status.set_on(CpuStatus::OVERFLOW);
        } else {
            self.status.set_off(CpuStatus::OVERFLOW);
        }
    }

    #[inline(always)]
    pub fn chk_zero_neg_w(
        &mut self,
        value: Word
    ) {
        self.chk_zero_neg(value, NEG_MASK as Word);
    }

    #[inline(always)]
    pub fn chk_zero_neg<T>(
        &mut self,
        value: T,
        mask: T,
    )
    where T: Copy + Integer + BitAnd<Output = T> {
        self.chk_neg(value, mask);
        self.chk_zero(value);
    }

    #[inline(always)]
    pub fn chk_neg<T>(
        &mut self,
        value: T,
        mask: T,
    )
    where T: BitAnd<Output = T> + Integer {
        if T::bitand(value, mask) != T::zero() {
            self.status.set_on(CpuStatus::NEGATIVE);
        } else {
            self.status.set_off(CpuStatus::NEGATIVE);
        }
    }

    #[inline(always)]
    pub fn chk_zero<T>(
        &mut self,
        value: T,
    )
    where T: Integer {
        if value.is_zero() {
            self.status.set_on(CpuStatus::ZERO);
        } else {
            self.status.set_off(CpuStatus::ZERO);
        }
    }
}

impl Cpu {
    pub fn translate(
        &self,
        mode: AddrMode
    ) -> Word {
        match mode {
            AddrMode::Relative => self.mem.read(self.pc) as Word,
            AddrMode::Immediate => self.pc,
            AddrMode::ZeroPage  => self.mem.read(self.pc) as Word,

            AddrMode::ZeroPageX => (self.mem.read(self.pc) as Word).wrapping_add(self.x as Word),
            AddrMode::ZeroPageY => (self.mem.read(self.pc) as Word).wrapping_add(self.y as Word),

            AddrMode::Absolute  => self.mem.read_word(self.pc),

            AddrMode::AbsoluteX => self.mem.read_word(self.pc).wrapping_add(self.x as Word),
            AddrMode::AbsoluteY => self.mem.read_word(self.pc).wrapping_add(self.y as Word),

            AddrMode::Indirect  => self.mem.read_word(self.mem.read_word(self.pc)),
            
            AddrMode::IndirectX => self.mem.read_word((self.mem.read(self.pc) as Word).wrapping_add(self.x as Word)),
            AddrMode::IndirectY => self.mem.read_word(self.mem.read(self.pc) as Word).wrapping_add(self.y as Word),
        }
    }
}
