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
}
