// Must import all dependencies here to run UT
mod memory_interface;
mod memory_model;
mod rv_core;

fn main() {
    let mut core: rv_core::RVCore = Default::default();
    core.run(5);
}
