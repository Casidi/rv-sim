struct InstType {
    data: u32,
    len: u32,
    operate: fn(&mut RVCore, &InstType),
}

impl InstType {
    fn get_rd(&self) -> usize {
        ((self.data & 0x00000f80) >> 7) as usize
    }

    fn get_rs1(&self) -> usize {
        ((self.data & 0x000f8000) >> 15) as usize
    }

    fn get_imm_itype(&self) -> u32 {
        self.data >> 20
    }

    fn get_imm_utype(&self) -> u32 {
        self.data & 0xfffff000
    }
}

#[derive(Default)]
pub struct RVCore {
    pc: u32,
    regs: [u32; 32],
}

impl RVCore {
    fn step(&mut self, inst_bytes: u32) {
        let inst = self.decode_inst(inst_bytes);
        (inst.operate)(self, &inst);
        self.pc += inst.len;
    }

    pub fn run(&mut self, num_steps: i32) {
        let mut step_count = 0;
        while step_count < num_steps {
            self.step(0x00002197); //AUIPC
            step_count += 1;
        }
    }

    fn decode_inst(&mut self, inst_bytes: u32) -> InstType {
        let mut new_inst = InstType {
            data: inst_bytes,
            len: 0,
            operate: RVCore::inst_nop,
        };
        match inst_bytes & 0b11 {
            0 | 1 | 2 => {
                new_inst.len = 2;
            }
            _ => {
                self.decode_inst_4byte(inst_bytes, &mut new_inst);
                new_inst.len = 4;
            }
        }

        new_inst
    }

    fn decode_inst_4byte(&mut self, inst_bytes: u32, inst: &mut InstType) {
        let opcode = inst_bytes & 0x7f;
        let funct3 = (inst_bytes & 0x00007000) >> 12;
        match opcode {
            0x13 => match funct3 {
                0x0 => {
                    inst.operate = RVCore::inst_addi;
                }
                _ => panic!("Invalid instruction"),
            },
            0x17 => {
                inst.operate = RVCore::inst_auipc;
            }
            _ => panic!("Invalid instruction"),
        }
    }

    fn inst_nop(&mut self, _inst: &InstType) {}

    fn inst_auipc(&mut self, inst: &InstType) {
        self.regs[inst.get_rd()] = self.pc + inst.get_imm_utype();
    }

    fn inst_addi(&mut self, inst: &InstType) {
        self.regs[inst.get_rd()] = self.regs[inst.get_rs1()] + inst.get_imm_itype();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inst_auipc_code(rd: u32, imm: u32) -> InstType {
        InstType {
            data: (rd << 7) | (imm & 0xfffff000) | 0x17,
            len: 4,
            operate: RVCore::inst_auipc,
        }
    }

    fn inst_addi_code(rd: u32, rs1: u32, imm: u32) -> InstType {
        InstType {
            data: (imm << 20) | (rs1 << 15) | (rd << 7) | 0x13,
            len: 4,
            operate: RVCore::inst_addi,
        }
    }

    #[test]
    fn test_core_run() {
        let mut core: RVCore = Default::default();
        assert_eq!(0, core.pc);

        core.run(5);
        assert_eq!(20, core.pc);
    }

    #[test]
    fn test_inst_auipc() {
        let mut core: RVCore = Default::default();
        core.pc = 0x1234;
        core.inst_auipc(&inst_auipc_code(1, 0xffff1000));
        assert_eq!(core.regs[1], 0xffff1000 + 0x1234);
    }

    #[test]
    fn test_inst_addi() {
        let mut core: RVCore = Default::default();
        core.regs[2] = 0x1234;
        core.inst_addi(&inst_addi_code(1, 2, 0x7ff));
        assert_eq!(core.regs[1], 0x7ff + 0x1234);
    }
}
