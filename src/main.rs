// Must import all dependencies here to run UT
mod memory_interface;
mod memory_model;
mod rv_core;
use goblin::elf;
use std::cell::RefCell;
use std::env;
use std::fs;
use std::rc::Rc;
type AddressType = u64;

struct InfoFromElf {
    tohost_addr: AddressType,
    fromhost_addr: AddressType,
    entry: AddressType,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Error, should provide the ELF to run");
        return;
    }

    let elf_path = &args[1];

    let mut core: rv_core::RVCore = rv_core::RVCore::new();
    let mem = Rc::new(RefCell::new(memory_model::MemoryModel::new()));

    // Hack for hello world
    //core.regs.write(2, 0x3ffffffb50);
    //mem.write_byte(0x3ffffffb50, 0x1);

    let elf_info = load_elf(&mut mem.borrow_mut(), elf_path);
    const RESET_VEC_SIZE: u32 = 8;
    let start_pc = elf_info.entry;
    let reset_vec: [u32; RESET_VEC_SIZE as usize] = [
        0x297,                                // auipc  t0,0x0
        0x28593 + (RESET_VEC_SIZE * 4 << 20), // addi   a1, t0, &dtb
        0xf1402573,                           // csrr   a0, mhartid
        0x0182b283u32,                        // ld     t0,24(t0)
        0x28067,                              // jr     t0
        0,
        (start_pc & 0xffffffff) as u32,
        (start_pc >> 32) as u32,
    ];

    for i in 0..reset_vec.len() {
        mem.borrow_mut()
            .write_word((0x1000 + i * 4) as AddressType, reset_vec[i]);
    }

    let mem_if: Rc<RefCell<dyn memory_interface::MemoryInterface>> = mem.clone();
    core.bind_mem(mem_if.clone());
    core.pc = 0x1000;

    for _i in 0..80 {
        core.run(5000);
        let tohost = mem.borrow_mut().read_word(elf_info.tohost_addr) as u64;
        if tohost != 0 {
            if (tohost & 1) == 1 {
                // End simulation
                let test_result = core.regs.read(10);
                if test_result == 0 {
                    println!("RISCV_TEST_PASS");
                } else {
                    println!("RISCV_TEST_FAIL");
                }

                break;
            } else {
                let sys_write_len = mem.borrow_mut().read_word(tohost + 24) as u64;
                mem.borrow_mut().write_word(tohost, sys_write_len as u32);
                mem.borrow_mut().write_word(elf_info.tohost_addr, 0);
                mem.borrow_mut().write_word(elf_info.fromhost_addr, 1);
            }
        }
    }

    //println!("Simulation ends");
}

fn load_elf(mem: &mut memory_model::MemoryModel, path: &str) -> InfoFromElf {
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

    let mut tohost_addr: AddressType = 0;
    let mut fromhost_addr: AddressType = 0;
    for sym in elf.syms.iter() {
        let name = (elf.strtab.get_at(sym.st_name)).unwrap();
        if name == "tohost" {
            //println!("{}: {:#010x}", name, sym.st_value);
            tohost_addr = sym.st_value;
        } else if name == "fromhost" {
            //println!("{}: {:#010x}", name, sym.st_value);
            fromhost_addr = sym.st_value;
        }
    }

    //println!("Entry = 0x{:x}", elf.entry);
    InfoFromElf {
        tohost_addr: tohost_addr,
        fromhost_addr: fromhost_addr,
        entry: elf.entry as AddressType,
    }
}
