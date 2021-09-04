type AddressType = u64;

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
        self.reg_bank[idx as usize]
    }

    pub fn write(&mut self, idx: AddressType, val: AddressType) {
        self.reg_bank[idx as usize] = val;
    }

    fn name(idx: AddressType) -> &'static str {
        match idx {
            _ => "invalid gpr name",
        }
    }
}
