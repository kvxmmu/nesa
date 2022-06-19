use {
    crate::{
        memory::*,
        cpu_registers::*,
        cpu_status::*,

        opcode::*,
        decoder::*,
    }
};

#[derive(Debug, Clone)]
pub struct NesCpu {
    pub mem: NesMemory,
    pub regs: CpuRegisters,
    pub status: CpuStatus,
}

impl NesCpu {
    pub fn lda(
        &mut self,
        mode: AddressingMode,
    ) {
        let addr = self.translate_addr(mode);
        let value = self.mem.read(addr);

        self.regs.acc = value;
        self.check_zero_neg(value);
    }

    fn check_zero_neg(&mut self, value: u8) {
        self.status.turn(CpuStatus::ZERO, || value == 0);
        self.status.turn(CpuStatus::NEGATIVE, || (value & (1 << 7)) != 0);
    }
}

impl NesCpu {
    pub fn translate_addr(&self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Absolute  => self.mem.read_u16(self.regs.pc),
            AddressingMode::AbsoluteX => self.mem.read_u16(self.regs.pc).wrapping_add(self.regs.x as u16),
            AddressingMode::AbsoluteY => self.mem.read_u16(self.regs.pc).wrapping_add(self.regs.y as u16),

            AddressingMode::Immediate => self.regs.pc,

            AddressingMode::ZeroPage  => self.mem.read(self.regs.pc) as u16,
            AddressingMode::ZeroPageX => self.mem.read(self.regs.pc).wrapping_add(self.regs.x) as u16,
            AddressingMode::ZeroPageY => self.mem.read(self.regs.pc).wrapping_add(self.regs.y) as u16,

            AddressingMode::Indirect  => self.mem.read_u16(self.mem.read(self.regs.pc) as u16),
            AddressingMode::IndirectX => self.mem.read_u16(self.mem.read(self.regs.pc).wrapping_add(
                self.regs.x,
            ) as u16),
            AddressingMode::IndirectY => self.mem.read_u16(self.mem.read(self.regs.pc) as u16).wrapping_add(
                self.regs.y as u16
            )
        }
    }

    pub fn interpret<T: AsRef<[u8]>>(
        &mut self,
        program: T
    ) -> &mut Self {
        self.load_reset(program)
            .run()
    }

    pub fn run(&mut self) -> &mut Self {
        'root: loop {
            let mut decoded = false;
            let opcode_hex = self.fetch();

            for opcode in OPCODES.iter() {
                if opcode_hex == opcode.hex {
                    if self.dispatch_opcode(opcode).is_err() {
                        break 'root;
                    }

                    decoded = true;
                    break;
                }
            }

            if !decoded {
                panic!("Illegal instruction: 0x{:x}", opcode_hex);
            }
        }

        self
    }

    fn dispatch_opcode(
        &mut self,
        opcode: &Opcode,
    ) -> Result<(), ()> {
        match opcode.opcode {
            OpcodeType::Lda => { self.lda(opcode.mode); }

            OpcodeType::Brk => return Err(()),
            _ => panic!("Unhandled opcode: {:#?}", opcode)
        }
        
        self.regs.add_pc(opcode.pc_offset);

        Ok(())
    }

    pub fn load_reset<T: AsRef<[u8]>>(
        &mut self,
        program: T
    ) -> &mut Self {
        self.load(program)
            .reset()
    }

    pub fn load<T: AsRef<[u8]>>(
        &mut self,
        program: T
    ) -> &mut Self {
        let program = program.as_ref();
        self.mem.bulk_write(0x8000, program);
        self.mem.write_u16(0xFFC, 0x8000);

        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.regs.acc = 0;
        self.regs.pc = self.mem.read_u16(0xFFC);
        self.regs.sp = 0;
        self.regs.x = 0;
        self.regs.y = 0;

        self
    }

    pub fn fetch(&mut self) -> u8 {
        let value = self.mem.read(self.regs.pc);
        self.regs.inc_pc();

        value
    }

    pub fn fetch_u16(&mut self) -> u16 {
        let value = self.mem.read_u16(self.regs.pc);
        self.regs.add_pc(2);

        value
    }

    pub fn new() -> Self {
        Self { mem: NesMemory::new()
             , regs: CpuRegisters::new()
             , status: CpuStatus::new() }
    }
}
