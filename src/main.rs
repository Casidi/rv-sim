mod rv_core;

fn main() {
    let mut core: rv_core::RVCore = Default::default();
    core.run(5);
}
