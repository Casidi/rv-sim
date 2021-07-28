# rv-sim
A RISC-V instruction-accurate simulator

## Quick Start
- Run simulator: ```cargo run <elf path>```
- Run all tests: ```cargo test```

## TODO
- [x] Encapsulate integer registers into a struct, use read/write API to access
- [ ] Implement one RV64I instruction and reserve the flexibility for 64-bit mode
- [ ] Pass [riscv-tests](https://github.com/riscv/riscv-tests)

## Instruction Status
Name       | Executed Count | Implemented
---------- | -------------  | -----------
c.addi     | 22             | :heavy_check_mark:
c.swsp     | 22             | :heavy_check_mark:
c.lwsp     | 21             | :heavy_check_mark:
addi       | 14             | :heavy_check_mark:
c.li       | 14             | :heavy_check_mark:
sb         | 13             | :heavy_check_mark:
lw         | 11             | :heavy_check_mark:
c.mv       | 10             | :heavy_check_mark:
ret        | 10             | 
c.beqz     | 9              | 
c.jal      | 6              | 
c.bnez     | 6              | 
c.add      | 6              | 
c.sw       | 6              | 
c.lw       | 6              | 
c.lui      | 6              | 
c.j        | 5              | 
auipc      | 4              | :heavy_check_mark:
beqz       | 4              | 
c.jalr     | 4              | 
li         | 4              | 
bne        | 4              | 
c.sub      | 3              | :heavy_check_mark:
srai       | 3              | 
bgeu       | 2              | 
andi       | 2              | 
slli       | 2              | 
sw         | 2              | 
sub        | 2              | 
c.addi4spn | 2              | 
c.addi16sp | 2              | 
beq        | 2              | 
jalr       | 1              |
c.andi     | 1              |
bltu       | 1              |
blt        | 1              |
bltz       | 1              |
c.slli     | 1              |
sll        | 1              |
c.and      | 1              |
lbu        | 1              |
ecall      | 1              |
