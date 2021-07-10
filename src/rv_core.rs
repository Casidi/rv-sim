#[derive(Default)]
pub struct RVCore {
    pc: u32,
    regs: [u32; 32],
}

impl RVCore {
    fn step(&mut self, inst: u32) {
		self.decode_inst(inst);
    }
    
    pub fn run(&mut self, num_steps: i32) {
        let mut step_count = 0;
        while step_count < num_steps {
            self.step(0x00002197); //AUIPC
            step_count += 1;
        }
    }

	fn decode_inst(&mut self, inst: u32) {
		match inst & 0b11 {
			0 | 1 | 2 => {
				self.pc += 2;
			}
			_ => {
				self.execute_inst_4byte(inst);
				self.pc += 4;
			}
		}
	}

	fn execute_inst_4byte(&mut self, inst: u32) {
		let opcode = inst & 0x7f;
		let funct3 = (inst & 0x00007000) >> 12;
		let rd = ((inst & 0x00000f80) >> 7) as usize;
		let rs1 = ((inst & 0x000f8000) >> 15) as usize;
		match opcode {
			0x13 => {
				let imm = inst >> 20;
				match funct3 {
					0x0 => {
						self.inst_addi(rd, rs1, imm);
					}
					_ => panic!("Invalid instruction"),
				}
			}
			0x17 => {
				let imm = inst & 0xfffff000;
				self.inst_auipc(rd, imm);
			}
			_ => panic!("Invalid instruction"),
		}
	}

	fn inst_auipc(&mut self, rd: usize, imm: u32) {
		self.regs[rd] = self.pc + imm;
	}

	fn inst_addi(&mut self, rd: usize, rs: usize, imm: u32) {
		self.regs[rd] = self.regs[rs] + imm;
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
