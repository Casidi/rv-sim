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

    pub fn get_rs2(&self) -> usize {
        ((self.data >> 2) & 0x1f) as usize
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

    pub fn get_imm_css(&self) -> u32 {
        (self.data >> 7) & 0x3f
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

pub fn inst_c_swsp_code(rs2: u32, imm: u32) -> InstType {
    InstType {
        data: (((imm >> 2) & 0xf) << 9) | (((imm >> 6) & 0x3) << 7) | ((rs2 & 0x1f) << 2)
                | 0x2 | (0x6 << 13),
        len: 2,
        id: InstID::C_SWSP,
    }
}

pub fn inst_c_lwsp_code(rd: u32, offset: u32) -> InstType {
    InstType {
        data: (((offset >> 2) & 0x7) << 4)
                | (((offset >> 6) & 0x3) << 2)
                | (((offset >> 5) & 0x1) << 12)
                | ((rd & 0x1f) << 7)
                | 0x2 | (0x2 << 13),
        len: 2,
        id: InstID::C_LWSP,
    }
}
