// Must import all dependencies here to run UT
mod memory_interface;
mod memory_model;
mod rv_core;
use goblin::elf;
use std::env;
use std::fs;
type AddressType = u64;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Error, should provide the ELF to run");
        return;
    }

    let elf_path = &args[1];

    let mut core: rv_core::RVCore = rv_core::RVCore::new();
    let mut mem = memory_model::MemoryModel::new();

    // Hack for hello world
    //core.regs.write(2, 0x3ffffffb50);
    //mem.write_byte(0x3ffffffb50, 0x1);

    let entry = load_elf(&mut mem, elf_path);
    const reset_vec_size: u32 = 8;
    let start_pc = entry;
    let reset_vec: [u32; reset_vec_size as usize] = [
        0x297,                                      // auipc  t0,0x0
        0x28593 + (reset_vec_size * 4 << 20),       // addi   a1, t0, &dtb
        0xf1402573,                                 // csrr   a0, mhartid
        0x0182b283u32,                              // ld     t0,24(t0)
        0x28067,                                    // jr     t0
        0,
        (start_pc & 0xffffffff) as u32,
        (start_pc >> 32) as u32
    ];

    for i in 0..reset_vec.len() {
        mem.write_word((0x1000 + i*4) as AddressType, reset_vec[i]);
    }

    core.bind_mem(&mut mem);
    core.pc = 0x1000;
    core.run(1000000);

    //println!("Simulation ends");
}

fn load_elf(mem: &mut memory_model::MemoryModel, path: &str) -> AddressType {
    let bytes = fs::read(path).unwrap();
    let elf = elf::Elf::parse(&bytes).unwrap();
    for ph in elf.program_headers {
        if ph.p_type == goblin::elf::program_header::PT_LOAD {
            for offset in 0..ph.p_filesz {
                mem.write_byte(
                    (ph.p_paddr + offset) as AddressType,
                    bytes[(ph.p_offset + offset) as usize],
                );
            }
        }
    }

    //println!("Entry = 0x{:x}", elf.entry);
    elf.entry as AddressType
}
