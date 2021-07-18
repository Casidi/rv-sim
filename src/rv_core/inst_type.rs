use crate::rv_core::RVCore;

pub struct InstType {
    pub data: u32,
    pub len: u32,
    pub operate: fn(&mut RVCore, &InstType),
}

impl InstType {
    pub fn get_rd(&self) -> usize {
        ((self.data & 0x00000f80) >> 7) as usize
    }

    pub fn get_rs1(&self) -> usize {
        ((self.data & 0x000f8000) >> 15) as usize
    }

    pub fn get_imm_itype(&self) -> u32 {
        self.data >> 20
    }

    pub fn get_imm_utype(&self) -> u32 {
        self.data & 0xfffff000
    }
}

pub fn inst_auipc_code(rd: u32, imm: u32) -> InstType {
	InstType {
		data: (rd << 7) | (imm & 0xfffff000) | 0x17,
		len: 4,
		operate: RVCore::inst_auipc,
	}
}

pub fn inst_addi_code(rd: u32, rs1: u32, imm: u32) -> InstType {
	InstType {
		data: (imm << 20) | (rs1 << 15) | (rd << 7) | 0x13,
		len: 4,
		operate: RVCore::inst_addi,
	}
}
