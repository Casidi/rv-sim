# rv-sim
A RISC-V instruction-accurate simulator

## Quick Start
- Run simulator: ```cargo run <elf path>```
- Run all tests: ```cargo test```

## TODO
- [x] Encapsulate integer registers into a struct, use read/write API to access
- [x] Implement one RV64I instruction and reserve the flexibility for 64-bit mode (ex. ADDIW)
- [ ] (28/49) Able to run hello world program (empty main only)
- [ ] (/40) Support all RV32I instructions
- [ ] (/15) Support all RV64I instructions
- [ ] (/49) Support all RVC instructions
- [ ] Pass [riscv-tests](https://github.com/riscv/riscv-tests)

## Instruction Status
Name       | Execution<br>Count | Implemented
-----      | -----    | -----
c.sdsp     |       22 | :heavy_check_mark:
c.ldsp     |       20 |
c.addi     |       19 | :heavy_check_mark:
addi       |       14 | :heavy_check_mark:
c.li       |       14 | :heavy_check_mark:
ret(c.jr)  |       10 | :heavy_check_mark:
c.mv       |        9 | :heavy_check_mark:
sb         |        9 | :heavy_check_mark:
c.beqz     |        9 | :heavy_check_mark:
c.sd       |        7 | :heavy_check_mark:
jal        |        6 | :heavy_check_mark:
c.bnez     |        6 | :heavy_check_mark:
c.add      |        6 | :heavy_check_mark:
ld         |        6 | :heavy_check_mark:
c.lui      |        6 | :heavy_check_mark:
c.j        |        5 | :heavy_check_mark:
c.ld       |        5 |
lw         |        5 | :heavy_check_mark:
auipc      |        4 | :heavy_check_mark:
c.addiw    |        4 | :heavy_check_mark:
beqz       |        4 | next
c.jalr     |        4 |
li         |        4 |
bne        |        4 |
c.addi16sp |        4 |
c.sub      |        3 | :heavy_check_mark:
bltu       |        3 | :heavy_check_mark:
srai       |        3 | :heavy_check_mark:
bgeu       |        2 | :heavy_check_mark:
andi       |        2 | :heavy_check_mark:
c.slli     |        2 | :heavy_check_mark:
sub        |        2 | :heavy_check_mark:
c.addi4spn |        2 |
beq        |        2 |
slli       |        1 | :heavy_check_mark:
jalr       |        1 | :heavy_check_mark:
c.andi     |        1 | :heavy_check_mark:
sd         |        1 | :heavy_check_mark:
c.lw       |        1 | :heavy_check_mark:
blt        |        1 | :heavy_check_mark:
c.sw       |        1 | :heavy_check_mark:
c.lwsp     |        1 | :heavy_check_mark:
addiw      |        1 |
bltz       |        1 |
sw         |        1 |
sllw       |        1 |
c.and      |        1 |
lbu        |        1 |
ecall      |        1 |
c.swsp     |          | :heavy_check_mark:
c.jal      |          | :heavy_check_mark:
srli       |          | :heavy_check_mark:
