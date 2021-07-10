mod rv_core;

fn main() {
    let mut core: rv_core::RVCore = Default::default();
    core.run(5);
}

#[test]
fn test_core_run() {
    let mut core: rv_core::RVCore = Default::default();
	assert_eq!(0, core.pc);

    core.run(5);
	assert_eq!(20, core.pc);
}
