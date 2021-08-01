# rv-sim
A RISC-V instruction-accurate simulator

## Quick Start
- Run simulator: ```cargo run <elf path>```
- Run all tests: ```cargo test```

## TODO
- [x] Encapsulate integer registers into a struct, use read/write API to access
- [ ] Implement one RV64I instruction and reserve the flexibility for 64-bit mode (ex. ADDIW)
- [ ] Pass [riscv-tests](https://github.com/riscv/riscv-tests)

## Instruction Status
Name       | Execution<br>Count | Implemented
-----      | -----    | -----
c.sdsp     |       22 |
c.ldsp     |       20 |
c.addi     |       19 | :heavy_check_mark:
addi       |       14 | :heavy_check_mark:
c.li       |       14 | :heavy_check_mark:
ret        |       10 |
c.mv       |        9 | :heavy_check_mark:
sb         |        9 | :heavy_check_mark:
c.beqz     |        9 |
c.sd       |        7 |
jal        |        6 | :heavy_check_mark:
c.bnez     |        6 | :heavy_check_mark:
c.add      |        6 |
ld         |        6 |
c.lui      |        6 |
c.j        |        5 |
c.ld       |        5 |
lw         |        5 | :heavy_check_mark:
auipc      |        4 | :heavy_check_mark:
c.addiw    |        4 |
beqz       |        4 |
c.jalr     |        4 |
li         |        4 |
bne        |        4 |
c.addi16sp |        4 |
c.sub      |        3 | :heavy_check_mark:
bltu       |        3 |
srai       |        3 |
bgeu       |        2 | :heavy_check_mark:
andi       |        2 | :heavy_check_mark:
c.slli     |        2 |
sub        |        2 |
c.addi4spn |        2 |
beq        |        2 |
slli       |        1 |
jalr       |        1 |
c.andi     |        1 |
sd         |        1 |
c.lw       |        1 |
blt        |        1 |
c.sw       |        1 |
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
