mod inst_type;
mod inst_decoder;
mod inst_info;

#[derive(Default)]
pub struct RVCore {
    pc: u32,
    regs: [u32; 32],
	id_instance: inst_decoder::InstDecoder,
}

impl RVCore {
    fn step(&mut self, inst_bytes: u32) {
		let inst = self.id_instance.decode(inst_bytes);
		(inst_info::inst_info_table[inst.id as usize].operate)(self, &inst);
        self.pc += inst.len;
    }

    pub fn run(&mut self, num_steps: i32) {
        let mut step_count = 0;
        while step_count < num_steps {
            self.step(0x00002197); //AUIPC
            step_count += 1;
        }
    }

    fn inst_nop(&mut self, _inst: &inst_type::InstType) {}

    fn inst_auipc(&mut self, inst: &inst_type::InstType) {
        self.regs[inst.get_rd()] = self.pc + inst.get_imm_utype();
    }

    fn inst_addi(&mut self, inst: &inst_type::InstType) {
        self.regs[inst.get_rd()] = self.regs[inst.get_rs1()] + inst.get_imm_itype();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        core.inst_auipc(&inst_type::inst_auipc_code(1, 0xffff1000));
        assert_eq!(core.regs[1], 0xffff1000 + 0x1234);
    }

    #[test]
    fn test_inst_addi() {
        let mut core: RVCore = Default::default();
        core.regs[2] = 0x1234;
        core.inst_addi(&inst_type::inst_addi_code(1, 2, 0x7ff));
        assert_eq!(core.regs[1], 0x7ff + 0x1234);
    }
}
