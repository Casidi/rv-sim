#[derive(Default)]
struct RVCore {
    pc: u32,
    regs: [u32; 32],
}

impl RVCore {
    fn step(&mut self, inst: u32) {
        println!("PC = {}, inst = {}", self.pc, inst);
        self.pc += 4;
    }
    
    fn run(&mut self, num_steps: i32) {
        let mut step_count = 0;
        while step_count < num_steps {
            self.step(0);
            step_count += 1;
        }
    }
}

fn main() {
    let mut core: RVCore = Default::default();
    core.run(5);
}
