use crate::rv_core::inst_info::InstID;

pub struct InstType {
    pub data: u32,
    pub len: u32,
    pub id: InstID,
}

impl InstType {
    pub fn get_rd(&self) -> usize {
        ((self.data >> 7) & 0x1f) as usize
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

    pub fn get_imm_ci(&self) -> u32 {
        ((self.data >> 2) & 0x1f) | (((self.data >> 12) & 1) << 5)
    }
}

pub fn inst_auipc_code(rd: u32, imm: u32) -> InstType {
    InstType {
        data: (rd << 7) | (imm & 0xfffff000) | 0x17,
        len: 4,
        id: InstID::AUIPC,
    }
}

pub fn inst_addi_code(rd: u32, rs1: u32, imm: u32) -> InstType {
    InstType {
        data: (imm << 20) | (rs1 << 15) | (rd << 7) | 0x13,
        len: 4,
        id: InstID::ADDI,
    }
}

pub fn inst_c_addi_code(rd: u32, imm: u32) -> InstType {
    InstType {
        data: (((imm >> 5) & 1) << 12) | (rd << 7) | ((imm & 0x1f) << 2) | 0x1,
        len: 2,
        id: InstID::C_ADDI,
    }
}
