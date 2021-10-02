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

    pub fn get_rd_cl(&self) -> usize {
        (((self.data >> 2) & 0x7) + 8) as usize
    }

    pub fn get_rd_ciw(&self) -> usize {
        (((self.data >> 2) & 0x7) + 8) as usize
    }

    pub fn get_rs1(&self) -> usize {
        ((self.data >> 15) & 0x1f) as usize
    }

    pub fn get_rs1_3b(&self) -> usize {
        (((self.data >> 7) & 0x7) + 8) as usize
    }

    pub fn get_rs1_cr(&self) -> usize {
        ((self.data >> 7) & 0x1f) as usize
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

    pub fn get_rs2_btype(&self) -> usize {
        ((self.data >> 20) & 0x1f) as usize
    }

    pub fn get_rs2_rtype(&self) -> usize {
        ((self.data >> 20) & 0x1f) as usize
    }

    pub fn get_rs3(&self) -> usize {
        ((self.data >> 27) & 0x1f) as usize
    }

    pub fn get_shamt_itype(&self) -> AddressType {
        (self.data >> 20) & 0x1f
    }

    pub fn get_imm_itype(&self) -> AddressType {
        (self.data >> 20) & 0xfff
    }

    pub fn get_imm_jtype(&self) -> AddressType {
        (self.data >> 12) & 0xfffff
    }

    pub fn get_imm_utype(&self) -> AddressType {
        self.data & 0xfffff000
    }

    pub fn get_imm_btype(&self) -> AddressType {
        (((self.data >> 25) & 0x7f) << 5) | ((self.data >> 7) & 0x1f)
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

    pub fn get_imm_cb(&self) -> AddressType {
        (((self.data >> 10) & 0x7) << 5) | ((self.data >> 2) & 0x1f)
    }

    pub fn get_imm_css(&self) -> AddressType {
        (self.data >> 7) & 0x3f
    }

    pub fn get_imm_cs(&self) -> AddressType {
        (((self.data >> 10) & 0x7) << 2) | ((self.data >> 5) & 0x3)
    }

    pub fn get_imm_cl(&self) -> AddressType {
        (((self.data >> 10) & 0x7) << 2) | ((self.data >> 5) & 0x3)
    }

    pub fn get_imm_ciw(&self) -> AddressType {
        (self.data >> 5) & 0xff
    }

    pub fn get_csr(&self) -> AddressType {
        (self.data >> 20) & 0xfff
    }
}

#[cfg(test)]
pub mod tests {
    use super::AddressType;
    use super::InstType;
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
            data: (imm << 20) | (rs1 << 15) | (rd << 7) | 0x13 | (0x0 << 12),
            len: 4,
            id: InstID::ADDI,
        }
    }

    pub fn inst_andi_code(rd: AddressType, rs1: AddressType, imm: AddressType) -> InstType {
        InstType {
            data: (imm << 20) | (rs1 << 15) | (rd << 7) | 0x13 | (0x7 << 12),
            len: 4,
            id: InstID::ANDI,
        }
    }

    pub fn inst_bgeu_code(rs1: AddressType, rs2: AddressType, imm: AddressType) -> InstType {
        InstType {
            data: (rs1 << 15)
                | (rs2 << 20)
                | 0x63
                | (0x7 << 12)
                | (((imm >> 12) & 1) << 31)
                | (((imm >> 5) & 0x3f) << 25)
                | (((imm >> 1) & 0xf) << 8)
                | (((imm >> 11) & 1) << 7),
            len: 4,
            id: InstID::BGEU,
        }
    }

    pub fn inst_c_add_code(rd: AddressType, rs2: AddressType) -> InstType {
        InstType {
            data: (rd << 7) | (rs2 << 2) | 0x2 | (0x9 << 12),
            len: 2,
            id: InstID::C_ADD,
        }
    }

    pub fn inst_c_addi_code(rd: AddressType, imm: AddressType) -> InstType {
        InstType {
            data: (((imm >> 5) & 1) << 12) | (rd << 7) | ((imm & 0x1f) << 2) | 0x1,
            len: 2,
            id: InstID::C_ADDI,
        }
    }

    pub fn inst_c_andi_code(rd: AddressType, imm: AddressType) -> InstType {
        InstType {
            data: (((rd - 8) & 0x7) << 7)
                | ((imm & 0x1f) << 2)
                | ((imm >> 5) << 12)
                | 0x1
                | (4 << 13)
                | (0b10 << 10),
            len: 2,
            id: InstID::C_ANDI,
        }
    }

    pub fn inst_c_beqz_code(rs1: AddressType, offset: AddressType) -> InstType {
        InstType {
            data: ((rs1 - 8) << 7)
                | 0x1
                | (0x6 << 13)
                | (((offset >> 8) & 1) << 12)
                | (((offset >> 3) & 3) << 10)
                | (((offset >> 6) & 3) << 5)
                | (((offset >> 1) & 3) << 3)
                | (((offset >> 5) & 1) << 2),
            len: 2,
            id: InstID::C_BEQZ,
        }
    }

    pub fn inst_c_bnez_code(rs1: AddressType, offset: AddressType) -> InstType {
        InstType {
            data: ((rs1 - 8) << 7)
                | 0x1
                | (0x7 << 13)
                | (((offset >> 8) & 1) << 12)
                | (((offset >> 3) & 3) << 10)
                | (((offset >> 6) & 3) << 5)
                | (((offset >> 1) & 3) << 3)
                | (((offset >> 5) & 1) << 2),
            len: 2,
            id: InstID::C_BNEZ,
        }
    }

    pub fn inst_c_j_code(imm: AddressType) -> InstType {
        InstType {
            data: 0x1
                | (5 << 13)
                | (((imm >> 11) & 1) << 12)
                | (((imm >> 4) & 1) << 11)
                | (((imm >> 8) & 3) << 9)
                | (((imm >> 10) & 1) << 8)
                | (((imm >> 6) & 1) << 7)
                | (((imm >> 7) & 1) << 6)
                | (((imm >> 1) & 7) << 3)
                | (((imm >> 5) & 1) << 2),
            len: 2,
            id: InstID::C_J,
        }
    }

    pub fn inst_c_jr_code(rs1: AddressType) -> InstType {
        InstType {
            data: 0x2 | (0b100 << 13) | (rs1 << 7),
            len: 2,
            id: InstID::C_JR,
        }
    }

    pub fn inst_c_swsp_code(rs2: AddressType, imm: AddressType) -> InstType {
        InstType {
            data: (((imm >> 2) & 0xf) << 9)
                | (((imm >> 6) & 0x3) << 7)
                | ((rs2 & 0x1f) << 2)
                | 0x2
                | (0x6 << 13),
            len: 2,
            id: InstID::C_SWSP,
        }
    }

    pub fn inst_c_lw_code(
        rd_3b: AddressType,
        rs1_3b: AddressType,
        offset: AddressType,
    ) -> InstType {
        InstType {
            data: (((rs1_3b - 8) & 0x7) << 7)
                | (((rd_3b - 8) & 0x7) << 2)
                | (((offset >> 3) & 0x7) << 10)
                | (((offset >> 2) & 0x1) << 6)
                | (((offset >> 6) & 0x1) << 5)
                | 0x0
                | (0x2 << 13),
            len: 2,
            id: InstID::C_LW,
        }
    }

    pub fn inst_c_lwsp_code(rd: AddressType, offset: AddressType) -> InstType {
        InstType {
            data: (((offset >> 2) & 0x7) << 4)
                | (((offset >> 6) & 0x3) << 2)
                | (((offset >> 5) & 0x1) << 12)
                | ((rd & 0x1f) << 7)
                | 0x2
                | (0x2 << 13),
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

    pub fn inst_c_sd_code(rs2: AddressType, rs1: AddressType, offset: AddressType) -> InstType {
        InstType {
            data: (((rs1 - 8) & 0x7) << 7)
                | (((rs2 - 8) & 0x7) << 2)
                | (((offset >> 3) & 0x7) << 10)
                | (((offset >> 6) & 0x3) << 5)
                | 0x0
                | (0x7 << 13),
            len: 2,
            id: InstID::C_SD,
        }
    }

    pub fn inst_c_sub_code(rd: AddressType, rs2: AddressType) -> InstType {
        InstType {
            data: ((rd & 0x7) << 7) | ((rs2 & 0x7) << 2) | 0x1 | (0x8 << 12),
            len: 2,
            id: InstID::C_SUB,
        }
    }

    pub fn inst_jal_code(rd: AddressType, imm: AddressType) -> InstType {
        InstType {
            data: ((rd & 0x1f) << 7)
                | (((imm >> 20) & 1) << 31)
                | (((imm >> 1) & 0x3ff) << 21)
                | (((imm >> 11) & 1) << 20)
                | (((imm >> 12) & 0xff) << 12)
                | 0b1101111,
            len: 4,
            id: InstID::JAL,
        }
    }

    pub fn inst_jalr_code(rd: AddressType, rs1: AddressType, offset: AddressType) -> InstType {
        InstType {
            data: ((rd & 0x1f) << 7) | ((rs1 & 0x1f) << 15) | ((offset & 0xfff) << 20) | 0x67,
            len: 4,
            id: InstID::JALR,
        }
    }

    pub fn inst_ld_code(rd: AddressType, rs1: AddressType, imm: AddressType) -> InstType {
        InstType {
            data: ((imm & 0xfff) << 20)
                | ((rd & 0x1f) << 7)
                | ((rs1 & 0x1f) << 15)
                | 0x3
                | (0x3 << 12),
            len: 4,
            id: InstID::LD,
        }
    }

    pub fn inst_lw_code(rd: AddressType, rs1: AddressType, imm: AddressType) -> InstType {
        InstType {
            data: ((imm & 0xfff) << 20)
                | ((rd & 0x1f) << 7)
                | ((rs1 & 0x1f) << 15)
                | 0x3
                | (0x2 << 12),
            len: 4,
            id: InstID::LW,
        }
    }

    pub fn inst_sb_code(rs2: AddressType, rs1: AddressType, imm: AddressType) -> InstType {
        InstType {
            data: (((imm >> 5) & 0x7f) << 25)
                | ((imm & 0x1f) << 7)
                | ((rs2 & 0x1f) << 20)
                | ((rs1 & 0x1f) << 15)
                | 0x23
                | (0 << 12),
            len: 4,
            id: InstID::SB,
        }
    }

    pub fn inst_sd_code(rs2: AddressType, rs1: AddressType, imm: AddressType) -> InstType {
        InstType {
            data: (((imm >> 5) & 0x7f) << 25)
                | ((imm & 0x1f) << 7)
                | ((rs2 & 0x1f) << 20)
                | ((rs1 & 0x1f) << 15)
                | 0x23
                | (0x3 << 12),
            len: 4,
            id: InstID::SD,
        }
    }

    pub fn inst_slli_code(rd: AddressType, rs1: AddressType, shamt: AddressType) -> InstType {
        InstType {
            data: (shamt << 20) | (rs1 << 15) | (rd << 7) | 0x13 | (0x0 << 25) | (1 << 12),
            len: 4,
            id: InstID::SLLI,
        }
    }

    pub fn inst_srli_code(rd: AddressType, rs1: AddressType, shamt: AddressType) -> InstType {
        InstType {
            data: (shamt << 20) | (rs1 << 15) | (rd << 7) | 0x13 | (0x0 << 25) | (0b101 << 12),
            len: 4,
            id: InstID::SRLI,
        }
    }

    pub fn inst_srai_code(rd: AddressType, rs1: AddressType, shamt: AddressType) -> InstType {
        InstType {
            data: (shamt << 20) | (rs1 << 15) | (rd << 7) | 0x13 | (0x20 << 25) | (0b101 << 12),
            len: 4,
            id: InstID::SRAI,
        }
    }
}
