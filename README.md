# rv-sim
A RISC-V instruction-accurate simulator

## Quick Start
- Run simulator: ```cargo run <elf path>```
- Run all tests: ```cargo test```

## TODO
- [x] Encapsulate integer registers into a struct, use read/write API to access
- [x] Implement one RV64I instruction and reserve the flexibility for 64-bit mode (ex. ADDIW)
- [x] (49/49) Able to run hello world program (empty main only), match pctrace to Spike
- [ ] (/) Able to run coremark, match pctrace to Spike
    - RV porting of cormark: [riscv-coremark](https://github.com/riscv-boom/riscv-coremark)
- [ ] (/) Test with random program (csmith or yarpgen), match pctrace to Spike
- [ ] Stress test with [riscv-torture](https://github.com/ucb-bar/riscv-torture)
- [ ] Align commit log to Spike, compare commit log with RTL or spike
- [ ] (15/40) Support all RV32I instructions
- [ ] (4/15) Support all RV64I instructions
- [ ] (25/49) Support all RVC instructions
- [ ] Pass [riscv-tests](https://github.com/riscv/riscv-tests)

## Reference Materials
- [The usage of fromhost and tohost symbols in ELF](https://github.com/riscv/riscv-isa-sim/issues/364)
- [BOOM workshop](https://riscv.org/wp-content/uploads/2016/01/Wed1345-RISCV-Workshop-3-BOOM.pdf)
- [RTL commit log](https://docs.boom-core.org/en/latest/sections/parameterization.html)
- Spike commit log:```../configure --enable-commitlog```

## Chipyard simulation
```
sudo apt get verilator
#download riscv gnu toolchain
export RISCV=path to toolchain
./scripts/init-submodules-no-riscv-tools.sh
cd sims/verilator
make

https://chipyard.readthedocs.io/en/dev/Chipyard-Basics/Initial-Repo-Setup.html
https://chipyard.readthedocs.io/en/dev/Simulation/Software-RTL-Simulation.html#verilator-open-source
```
