mod rv_core;
mod memory_model;

fn main() {
    let mut core: rv_core::RVCore = Default::default();
    core.run(5);
}
