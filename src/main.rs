// Must import all dependencies here to run UT
mod memory_interface;
mod memory_model;
mod rv_core;

fn main() {
    let mut core: rv_core::RVCore = Default::default();
    let mut mem = memory_model::MemoryModel::new();

    core.bind_mem(&mut mem);
    core.run(5);
}
