// Must import all dependencies here to run UT
mod memory_interface;
mod memory_model;
mod rv_core;
use goblin::elf;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Error, should provide the ELF to run");
        return
    }

    let elf_path = &args[1];

    let mut core: rv_core::RVCore = rv_core::RVCore::new();
    let mut mem = memory_model::MemoryModel::new();

    let entry = load_elf(&mut mem, elf_path);
    core.bind_mem(&mut mem);

    core.pc = entry;
    core.run(10);
}

fn load_elf(mem: &mut memory_model::MemoryModel, path: &str) -> u32 {
    let bytes = fs::read(path).unwrap();
    let elf = elf::Elf::parse(&bytes).unwrap();
    for ph in elf.program_headers {
        if ph.p_type == goblin::elf::program_header::PT_LOAD {
            for offset in 0..ph.p_filesz {
                mem.write_byte((ph.p_paddr + offset) as u32,
                                    bytes[(ph.p_offset + offset) as usize]);
            }
        }
    }

    println!("Entry = 0x{:x}", elf.entry);
    elf.entry as u32
}
