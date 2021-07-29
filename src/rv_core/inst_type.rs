use crate::rv_core::inst_info::InstID;
type AddressType = u64;

pub struct InstType {
    pub data: AddressType,
    pub len: AddressType,
    pub id: InstID,
}

impl InstType {
    pub fn get_rd(&self) -> usize {
        ((self.data >> 7) & 0x1f) as usize
    }

    pub fn get_rd_3b(&self) -> usize {
        (((self.data >> 7) & 0x7) + 8) as usize
    }

    pub fn get_rs1(&self) -> usize {
        (((self.data >> 15) & 0x1f)) as usize
    }

    pub fn get_rs2(&self) -> usize {
        ((self.data >> 2) & 0x1f) as usize
    }

    pub fn get_rs2_3b(&self) -> usize {
        (((self.data >> 2) & 0x7) + 8) as usize
    }

    pub fn get_rs2_stype(&self) -> usize {
        ((self.data >> 20) & 0x1f) as usize
    }

    pub fn get_imm_itype(&self) -> AddressType {
        self.data >> 20
    }

    pub fn get_imm_utype(&self) -> AddressType {
        self.data & 0xfffff000
    }

    pub fn get_imm_stype(&self) -> AddressType {
        ((self.data >> 7) & 0x1f) | (((self.data >> 25) & 0x7f) << 5)
    }

    pub fn get_imm_ci(&self) -> AddressType {
        ((self.data >> 2) & 0x1f) | (((self.data >> 12) & 1) << 5)
    }

    pub fn get_imm_cj(&self) -> AddressType {
        (self.data >> 2) & 0x7ff
    }

    pub fn get_imm_css(&self) -> AddressType {
        (self.data >> 7) & 0x3f
    }
}

#[cfg(test)]
pub mod tests {
    use super::InstType;
    use super::AddressType;
    use crate::rv_core::inst_info::InstID;

    pub fn inst_auipc_code(rd: AddressType, imm: AddressType) -> InstType {
        InstType {
            data: (rd << 7) | (imm & 0xfffff000) | 0x17,
            len: 4,
            id: InstID::AUIPC,
        }
    }

    pub fn inst_addi_code(rd: AddressType, rs1: AddressType, imm: AddressType) -> InstType {
        InstType {
            data: (imm << 20) | (rs1 << 15) | (rd << 7) | 0x13,
            len: 4,
            id: InstID::ADDI,
        }
    }

    pub fn inst_c_addi_code(rd: AddressType, imm: AddressType) -> InstType {
        InstType {
            data: (((imm >> 5) & 1) << 12) | (rd << 7) | ((imm & 0x1f) << 2) | 0x1,
            len: 2,
            id: InstID::C_ADDI,
        }
    }

    pub fn inst_c_jal_code(imm: AddressType) -> InstType {
        InstType {
            data: 0x1 | (1 << 13)
                        | (((imm >> 11) & 1) << 12)
                        | (((imm >> 4) & 1) << 11)
                        | (((imm >> 8) & 3) << 9)
                        | (((imm >> 10) & 1) << 8)
                        | (((imm >> 6) & 1) << 7)
                        | (((imm >> 7) & 1) << 6)
                        | (((imm >> 1) & 7) << 3)
                        | (((imm >> 5) & 1) << 2),
            len: 2,
            id: InstID::C_JAL,
        }
    }

    pub fn inst_c_swsp_code(rs2: AddressType, imm: AddressType) -> InstType {
        InstType {
            data: (((imm >> 2) & 0xf) << 9) | (((imm >> 6) & 0x3) << 7) | ((rs2 & 0x1f) << 2)
                    | 0x2 | (0x6 << 13),
            len: 2,
            id: InstID::C_SWSP,
        }
    }

    pub fn inst_c_lwsp_code(rd: AddressType, offset: AddressType) -> InstType {
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

    pub fn inst_c_li_code(rd: AddressType, imm: AddressType) -> InstType {
        InstType {
            data: (((imm >> 5) & 1) << 12) | (rd << 7) | ((imm & 0x1f) << 2) | 0x1 | (0x2 << 13),
            len: 2,
            id: InstID::C_LI,
        }
    }

    pub fn inst_c_mv_code(rd: AddressType, rs2: AddressType) -> InstType {
        InstType {
            data: (rd << 7) | (rs2 << 2) | 0x2 | (0x8 << 12),
            len: 2,
            id: InstID::C_MV,
        }
    }

    pub fn inst_c_sub_code(rd: AddressType, rs2: AddressType) -> InstType {
        InstType {
            data: ((rd & 0x7) << 7) | ((rs2 & 0x7) << 2) | 0x1 | (0x8 << 12),
            len: 2,
            id: InstID::C_SUB,
        }
    }

    pub fn inst_lw_code(rd: AddressType, rs1: AddressType, imm: AddressType) -> InstType {
        InstType {
            data: ((imm & 0xfff) << 20)
                    | ((rd & 0x1f) << 7)
                    | ((rs1 & 0x1f) << 15)
                    | 0x3 | (0x2 << 12),
            len: 4,
            id: InstID::LW,
        }
    }

    pub fn inst_sb_code(rs2: AddressType, rs1: AddressType, imm: AddressType) -> InstType {
        InstType {
            data: (((imm >> 5) & 0x7f) << 25) | ((imm & 0x1f) << 7)
                    | ((rs2 & 0x1f) << 20)
                    | ((rs1 & 0x1f) << 15)
                    | 0x23,
            len: 4,
            id: InstID::SB,
        }
    }
}
