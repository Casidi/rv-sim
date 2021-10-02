type AddressType = u64;

pub const FFLAGS: AddressType = 0x1;
pub const FFLAGS_RW_MASK: AddressType = 0x1f;
pub const FRM: AddressType = 0x2;
pub const FCSR: AddressType = 0x3;
pub const FCSR_RW_MASK: AddressType = 0xff;
pub const MTVEC: AddressType = 0x305;
pub const MCAUSE: AddressType = 0x342;
pub const MCYCLE: AddressType = 0xb00;
pub const MINSTRET: AddressType = 0xb02;
pub const EXC_ECALL_FROM_U: AddressType = 8;
pub const EXC_ECALL_FROM_S: AddressType = 9;
pub const EXC_ECALL_FROM_M: AddressType = 11;

pub struct CSRegisters {
    reg_bank: [AddressType; 4096],
}

impl CSRegisters {
    pub fn new() -> CSRegisters {
        CSRegisters {
            reg_bank: [0; 4096],
        }
    }

    pub fn read(&self, idx: AddressType) -> AddressType {
        if idx == FFLAGS {
            self.reg_bank[FCSR as usize] & FFLAGS_RW_MASK
        } else if idx == FRM {
            (self.reg_bank[FCSR as usize] >> 5) & 7
        } else {
            self.reg_bank[idx as usize]
        }
    }

    pub fn write(&mut self, idx: AddressType, val: AddressType) {
        if idx == FCSR {
            self.reg_bank[idx as usize] = val & FCSR_RW_MASK;
        } else if idx == FFLAGS {
            self.reg_bank[FCSR as usize] &= !FFLAGS_RW_MASK;
            self.reg_bank[FCSR as usize] |= val & FFLAGS_RW_MASK;
        } else if idx == FRM {
            self.reg_bank[FCSR as usize] &= !(0x7 << 5);
            self.reg_bank[FCSR as usize] |= val << 5;
        } else {
            self.reg_bank[idx as usize] = val;
        }
        //println!("JC_DEBUG: write csr {}, val {:#x}", idx, val)
    }

    /*fn name(idx: AddressType) -> &'static str {
        match idx {
            _ => "invalid gpr name",
        }
    }*/
}
