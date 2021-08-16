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
- [ ] (15/40) Support all RV32I instructions
- [ ] (4/15) Support all RV64I instructions
- [ ] (25/49) Support all RVC instructions
- [ ] Pass [riscv-tests](https://github.com/riscv/riscv-tests)
