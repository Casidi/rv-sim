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
- [x] Implement important CSR first (mhartid, minstret, mcycle)
- [x] Run riscv-tests ISA tests and see the Pass/Fail count
    - [x] Implement ECALL behavior, needs at lease mtvec CSR to work
- [x] Support all RV32I instructions
- [x] Support all RV64I instructions
- [x] Complete and test all RV64I instructions
- [x] Pass rv64uc-p-rvc to ensure the quality of RV64C instructions
- [x] Pass rv64uf-p-* tests
- [ ] Pass rv64ud-p-* tests
- [ ] Pass rv64ua-p-* tests
- [ ] Refactor: implement extract_bits function to simplify inst decoding
- [ ] (79/124) Pass all rv64*-p-* tests
- [ ] Pass all benchmarks in riscv-tests
- [ ] (/) Able to run coremark, match pctrace to Spike
    - RV porting of cormark: [riscv-coremark](https://github.com/riscv-boom/riscv-coremark)
- [ ] (/) Test with random program (csmith or yarpgen), match pctrace to Spike
- [ ] Stress test with [riscv-torture](https://github.com/ucb-bar/riscv-torture)
- [ ] Align commit log to Spike, compare commit log with RTL or spike
- [ ] (25/49) Support all RVC instructions
- [ ] Pass [riscv-tests](https://github.com/riscv/riscv-tests)
    - Relatively old, but simple to setup
    - Hand-written test cases
- [ ] Pass https://github.com/riscv/riscv-arch-test
    - Generates tests with formal description
- [ ] Pass https://github.com/riscv/riscof (new arch test)
- [ ] Co-sim with [riscv SAIL model](https://github.com/riscv/sail-riscv)

## Benchmark Status
- Done: Succeeds and matches Spike's pctrace

Name      | Status | Description
-----     | ------ | -----
dhrystone | Pass   | Done
median    | Pass   | Done
mm        | Crash  | Unimplemented: fcvt.d.l
mt-matmul | Crash  | Unimplemented: amoadd.w
mt-vvadd  | Crash  | Unimplemented: amoadd.w
multiply  | Pass   | Done
pmp       | Crash  | Unimplemented: sfence.vma
qsort     | Pass   | Done
rsort     | Pass   | Done
spmv      | Crash  | Unimplemented: fld, c.fld, ...
towers    | Pass   | Done
vvadd     | Pass   | Done

## Run RISCV Architecture Test
```bash
git clone https://github.com/riscv/riscv-arch-test.git
sudo apt install python3-pip
pip3 install git+https://github.com/riscv/riscof.git
mkdir riscof
cd riscof
riscof setup --dutname=spike
#riscof validateyaml --config=config.ini
#riscof testlist --config=config.ini --suite ../riscv-arch-test/riscv-test-suite/rv64i_m --env ../riscv-arch-test/riscv-test-suite/env

# Replace content in spike/spike_isa.yaml with https://github.com/riscv/riscv-config/blob/master/examples/rv64i_isa.yaml
# Add spike to PATH
# Build sail, add to PATH
sudo apt-get install opam  build-essential libgmp-dev z3 pkg-config zlib1g-dev
opam init -y --disable-sandboxing
opam switch create ocaml-base-compiler.4.06.1
opam install sail -y
eval $(opam config env)
git clone https://github.com/rems-project/sail-riscv.git
cd sail-riscv
make
cd ..

#Edit spike/riscof_spike.py, around Line 100
#execute += self.dut_exe + ' --log-commits --log dump --isa={0} +signature={1} +signature-granularity=4 {2};'.format(self.isa, sig_file, elf)

# Add riscv gnu toolchain to path

riscof run --config=config.ini --suite ../riscv-arch-test/riscv-test-suite/rv64i_m --env ../riscv-arch-test/riscv-test-suite/env
```

## Spike signature protocol
- In fesvr/htif.cc
```cpp
void htif_t::stop()
{
  if (!sig_file.empty() && sig_len) // print final torture test signature
  {
    std::vector<uint8_t> buf(sig_len);
    mem.read(sig_addr, sig_len, buf.data());

    std::ofstream sigs(sig_file);
    assert(sigs && "can't open signature file!");
    sigs << std::setfill('0') << std::hex;

    for (addr_t i = 0; i < sig_len; i += line_size)
    {
      for (addr_t j = line_size; j > 0; j--)
          if (i+j <= sig_len)
            sigs << std::setw(2) << (uint16_t)buf[i+j-1];
          else
            sigs << std::setw(2) << (uint16_t)0;
      sigs << '\n';
    }

    sigs.close();
  }

  stopped = true;
}
```
```cpp
// In load_program
if (symbols.count("begin_signature") && symbols.count("end_signature"))
{
sig_addr = symbols["begin_signature"];
sig_len = symbols["end_signature"] - sig_addr;
}
```

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

#Note: enable-commitlog will greatly increase the executable size (97MB -> 160MB)
#, --with-boost=no won't help
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
