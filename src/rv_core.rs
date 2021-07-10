#[derive(Default)]
pub struct RVCore {
    pc: u32,
    regs: [u32; 32],
}

impl RVCore {
    fn step(&mut self, inst: u32) {
        println!("PC = {}, inst = {}", self.pc, inst);
        self.pc += 4;
    }
    
    pub fn run(&mut self, num_steps: i32) {
        let mut step_count = 0;
        while step_count < num_steps {
            self.step(0);
            step_count += 1;
        }
    }

	fn inst_auipc(&mut self, rd: usize, imm: u32) {
		self.regs[rd] = self.pc + imm;
		self.pc += 4;
	}

	fn inst_addi(&mut self, rd: usize, rs: usize, imm: u32) {
		self.regs[rd] = self.regs[rs] + imm;
		self.pc += 4;
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
		core.inst_auipc(1, 0xffff1000);
		assert_eq!(core.regs[1], 0xffff1000 + 0x1234);
	}

	#[test]
	fn test_inst_addi() {
		let mut core: RVCore = Default::default();
		core.regs[2] = 0x1234;
		core.inst_addi(1, 2, 0xffff1000);
		assert_eq!(core.regs[1], 0xffff1000 + 0x1234);
	}
}
