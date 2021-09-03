# rv-sim
A RISC-V instruction-accurate simulator

## Quick Start
- Run simulator: ```cargo run <elf path>```
- Run all tests: ```cargo test```
- Run ISA tests: ```python3 compare.py```
    - Assume that riscv-tests is at ```../riscv-tests```

## TODO
- [x] Encapsulate integer registers into a struct, use read/write API to access
- [x] Implement one RV64I instruction and reserve the flexibility for 64-bit mode (ex. ADDIW)
- [x] (49/49) Able to run hello world program (empty main only), match pctrace to Spike
- [x] Able to run dhrystone, match pctrace to Spike
- [ ] Implement important CSR first (mhartid, minstret, mcycle)
- [ ] Run riscv-tests ISA tests and see the Pass/Fail count
    - [ ] Implement ECALL behavior, needs at lease mtvec CSR to work
- [ ] Pass all benchmarks in riscv-tests
- [ ] (/) Able to run coremark, match pctrace to Spike
    - RV porting of cormark: [riscv-coremark](https://github.com/riscv-boom/riscv-coremark)
- [ ] (/) Test with random program (csmith or yarpgen), match pctrace to Spike
- [ ] Stress test with [riscv-torture](https://github.com/ucb-bar/riscv-torture)
- [ ] Align commit log to Spike, compare commit log with RTL or spike
- [ ] (15/40) Support all RV32I instructions
- [ ] (4/15) Support all RV64I instructions
- [ ] (25/49) Support all RVC instructions
- [ ] Pass [riscv-tests](https://github.com/riscv/riscv-tests)
- [ ] Pass https://github.com/riscv/riscv-arch-test
- [ ] Pass https://github.com/riscv/riscof
- [ ] Co-sim with [riscv SAIL model](https://github.com/riscv/sail-riscv)

## Trace how Spike handle fromhost/tohost
- Check fromhost/tohost every INTERLEAVE(5000) instructions
- Need to check the memory every 5000 cycles and write if necessary
- The fromhost/tohost protocol
-   If tohost == 0: no operation
-   Else if tohost & 1: terminate simulation
    -   if a0 == 0: pass ISA test
    -   else: ISA test failed, test number = a0 >> 1
-   Else: syscall, tohost is the base address of uint64_t magic_mem[8]
    -   magic_mem[0] = syscall number when input, after handling syscall, return value is written to here
    -   magic_mem[3] = write length for sys_write

## Build riscv-tests
```bash
export PATH=$PATH:<riscv-toolchain-path>/bin
git clone https://github.com/riscv/riscv-tests
cd riscv-tests
git submodule update --init --recursive
autoconf
./configure
make
```

## Reference Materials
- [The usage of fromhost and tohost symbols in ELF](https://github.com/riscv/riscv-isa-sim/issues/364)
- [BOOM workshop](https://riscv.org/wp-content/uploads/2016/01/Wed1345-RISCV-Workshop-3-BOOM.pdf)
- [RTL commit log](https://docs.boom-core.org/en/latest/sections/parameterization.html)
- Spike commit log:```../configure --enable-commitlog```
- [SAIL language](https://www.cl.cam.ac.uk/~pes20/sail/)

## Generate commit log with Spike
```bash
cd riscv-isa-sim
mkdir build
cd build
../configure --enable-commitlog
make

./spike -l --log-commits ./riscv-tests/benchmarks/dhrystone.riscv
```

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
