use crate::rv_core::inst_info::InstID;
use crate::rv_core::inst_type::InstType;

type AddressType = u64;

pub struct InstDecoder {}

impl InstDecoder {
    pub fn new() -> InstDecoder {
        InstDecoder {}
    }

    pub fn decode(&self, inst_bytes: AddressType) -> InstType {
        let mut new_inst = InstType {
            data: inst_bytes,
            len: 0,
            id: InstID::NOP,
        };
        match inst_bytes & 0b11 {
            0 | 1 | 2 => {
                self.decode_inst_compressed(inst_bytes, &mut new_inst);
                new_inst.len = 2;
            }
            _ => {
                self.decode_inst_4byte(inst_bytes, &mut new_inst);
                new_inst.len = 4;
            }
        }

        new_inst
    }

    fn decode_inst_compressed(&self, inst_bytes: AddressType, inst: &mut InstType) {
        let opcode = inst_bytes & 0x3;
        let funct3 = (inst_bytes >> 13) & 7;
        match opcode {
            0x0 => match funct3 {
                0x0 => inst.id = InstID::C_ADDI4SPN,
                0x2 => inst.id = InstID::C_LW,
                0x3 => inst.id = InstID::C_LD,
                0x6 => inst.id = InstID::C_SW,
                0x7 => inst.id = InstID::C_SD,
                _ => self.dump_invalid_inst(inst),
            },
            0x1 => match funct3 {
                0x0 => inst.id = InstID::C_ADDI,
                0x1 => inst.id = InstID::C_ADDIW,
                0x3 => {
                    let rd = inst.get_rd();
                    match rd {
                        0 => self.dump_invalid_inst(inst),
                        2 => inst.id = InstID::C_ADDI16SP,
                        _ => inst.id = InstID::C_LUI,
                    }
                }
                0x2 => inst.id = InstID::C_LI,
                0x4 => {
                    let funct2 = (inst_bytes >> 10) & 3;
                    match funct2 {
                        0x0 => inst.id = InstID::C_SRLI,
                        0x1 => inst.id = InstID::C_SRAI,
                        0x2 => inst.id = InstID::C_ANDI,
                        0x3 => match ((inst_bytes >> 12) & 1, (inst_bytes >> 5) & 0b11) {
                            (0, 0) => inst.id = InstID::C_SUB,
                            (0, 1) => inst.id = InstID::C_XOR,
                            (0, 2) => inst.id = InstID::C_OR,
                            (0, 3) => inst.id = InstID::C_AND,
                            (1, 0) => inst.id = InstID::C_SUBW,
                            (1, 1) => inst.id = InstID::C_ADDW,
                            (_, _) => self.dump_invalid_inst(inst),
                        },
                        _ => self.dump_invalid_inst(inst),
                    }
                }
                0x5 => inst.id = InstID::C_J,
                0x6 => inst.id = InstID::C_BEQZ,
                0x7 => inst.id = InstID::C_BNEZ,
                _ => self.dump_invalid_inst(inst),
            },
            0x2 => match funct3 {
                0x0 => inst.id = InstID::C_SLLI,
                0x2 => inst.id = InstID::C_LWSP,
                0x3 => inst.id = InstID::C_LDSP,
                0x4 => match ((inst_bytes >> 12) & 1, (inst_bytes >> 2) & 0x1f) {
                    (0, 0) => inst.id = InstID::C_JR,
                    (0, _) => inst.id = InstID::C_MV,
                    (1, 0) => inst.id = InstID::C_JALR,
                    (1, _) => inst.id = InstID::C_ADD,
                    (_, _) => self.dump_invalid_inst(inst),
                },
                0x5 => inst.id = InstID::C_FSDSP,
                0x6 => inst.id = InstID::C_SWSP,
                0x7 => inst.id = InstID::C_SDSP,
                _ => self.dump_invalid_inst(inst),
            },
            _ => self.dump_invalid_inst(inst),
        }
    }

    fn decode_inst_4byte(&self, inst_bytes: AddressType, inst: &mut InstType) {
        let opcode = inst_bytes & 0x7f;
        let funct3 = (inst_bytes & 0x00007000) >> 12;
        match opcode {
            0x3 => match funct3 {
                0x0 => inst.id = InstID::LB,
                0x1 => inst.id = InstID::LH,
                0x2 => inst.id = InstID::LW,
                0x3 => inst.id = InstID::LD,
                0x4 => inst.id = InstID::LBU,
                0x5 => inst.id = InstID::LHU,
                0x6 => inst.id = InstID::LWU,
                _ => self.dump_invalid_inst(inst),
            },
            0x7 => match funct3 {
                0x2 => inst.id = InstID::FLW,
                0x3 => inst.id = InstID::FLD,
                _ => self.dump_invalid_inst(inst),
            },
            0x0f => inst.id = InstID::FENCE,
            0x13 => match funct3 {
                0x0 => inst.id = InstID::ADDI,
                0x1 => inst.id = InstID::SLLI,
                0x2 => inst.id = InstID::SLTI,
                0x3 => inst.id = InstID::SLTIU,
                0x4 => inst.id = InstID::XORI,
                0x5 => match (inst_bytes >> 26) & 0x3f {
                    0x0 => inst.id = InstID::SRLI,
                    0x10 => inst.id = InstID::SRAI,
                    _ => self.dump_invalid_inst(inst),
                },
                0x6 => inst.id = InstID::ORI,
                0x7 => inst.id = InstID::ANDI,
                _ => self.dump_invalid_inst(inst),
            },
            0x17 => {
                inst.id = InstID::AUIPC;
            }
            0x1b => match funct3 {
                0x0 => inst.id = InstID::ADDIW,
                0x1 => inst.id = InstID::SLLIW,
                0x5 => {
                    let funct7 = (inst_bytes >> 25) & 0x7f;
                    match funct7 {
                        0x0 => inst.id = InstID::SRLIW,
                        0x20 => inst.id = InstID::SRAIW,
                        _ => self.dump_invalid_inst(inst),
                    }
                }
                _ => self.dump_invalid_inst(inst),
            },
            0x23 => match funct3 {
                0x0 => inst.id = InstID::SB,
                0x1 => inst.id = InstID::SH,
                0x2 => inst.id = InstID::SW,
                0x3 => inst.id = InstID::SD,
                _ => self.dump_invalid_inst(inst),
            },
            0x27 => match funct3 {
                0x2 => inst.id = InstID::FSW,
                0x3 => inst.id = InstID::FSD,
                _ => self.dump_invalid_inst(inst),
            }
            0x33 => match funct3 {
                0x0 => {
                    let funct7 = (inst_bytes >> 25) & 0x7f;
                    match funct7 {
                        0x0 => inst.id = InstID::ADD,
                        0x1 => inst.id = InstID::MUL,
                        0x20 => inst.id = InstID::SUB,
                        _ => self.dump_invalid_inst(inst),
                    }
                }
                0x1 => {
                    let funct7 = (inst_bytes >> 25) & 0x7f;
                    match funct7 {
                        0x0 => inst.id = InstID::SLL,
                        0x1 => inst.id = InstID::MULH,
                        _ => self.dump_invalid_inst(inst),
                    }
                }
                0x2 => {
                    let funct7 = (inst_bytes >> 25) & 0x7f;
                    match funct7 {
                        0x0 => inst.id = InstID::SLT,
                        0x1 => inst.id = InstID::MULHSU,
                        _ => self.dump_invalid_inst(inst),
                    }
                }
                0x3 => {
                    let funct7 = (inst_bytes >> 25) & 0x7f;
                    match funct7 {
                        0x0 => inst.id = InstID::SLTU,
                        0x1 => inst.id = InstID::MULHU,
                        _ => self.dump_invalid_inst(inst),
                    }
                }
                0x4 => {
                    let funct7 = (inst_bytes >> 25) & 0x7f;
                    match funct7 {
                        0x0 => inst.id = InstID::XOR,
                        0x1 => inst.id = InstID::DIV,
                        _ => self.dump_invalid_inst(inst),
                    }
                }
                0x5 => {
                    let funct7 = (inst_bytes >> 25) & 0x7f;
                    match funct7 {
                        0x0 => inst.id = InstID::SRL,
                        0x1 => inst.id = InstID::DIVU,
                        0x20 => inst.id = InstID::SRA,
                        _ => self.dump_invalid_inst(inst),
                    }
                }
                0x6 => {
                    let funct7 = (inst_bytes >> 25) & 0x7f;
                    match funct7 {
                        0x0 => inst.id = InstID::OR,
                        0x1 => inst.id = InstID::REM,
                        _ => self.dump_invalid_inst(inst),
                    }
                }
                0x7 => {
                    let funct7 = (inst_bytes >> 25) & 0x7f;
                    match funct7 {
                        0x0 => inst.id = InstID::AND,
                        0x1 => inst.id = InstID::REMU,
                        _ => self.dump_invalid_inst(inst),
                    }
                }
                _ => self.dump_invalid_inst(inst),
            },
            0x37 => {
                inst.id = InstID::LUI;
            }
            0x3b => match funct3 {
                0x0 => {
                    let funct7 = (inst_bytes >> 25) & 0x7f;
                    match funct7 {
                        0x0 => inst.id = InstID::ADDW,
                        0x1 => inst.id = InstID::MULW,
                        0x20 => inst.id = InstID::SUBW,
                        _ => self.dump_invalid_inst(inst),
                    }
                }
                0x1 => {
                    let funct7 = (inst_bytes >> 25) & 0x7f;
                    match funct7 {
                        0x0 => inst.id = InstID::SLLW,
                        _ => self.dump_invalid_inst(inst),
                    }
                }
                0x4 => inst.id = InstID::DIVW,
                0x5 => {
                    let funct7 = (inst_bytes >> 25) & 0x7f;
                    match funct7 {
                        0x0 => inst.id = InstID::SRLW,
                        0x1 => inst.id = InstID::DIVUW,
                        0x20 => inst.id = InstID::SRAW,
                        _ => self.dump_invalid_inst(inst),
                    }
                }
                0x6 => inst.id = InstID::REMW,
                0x7 => inst.id = InstID::REMUW,
                _ => self.dump_invalid_inst(inst),
            },
            0x43 => inst.id = InstID::FMADD_S,
            0x47 => inst.id = InstID::FMSUB_S,
            0x4b => inst.id = InstID::FNMSUB_S,
            0x4f => inst.id = InstID::FNMADD_S,
            0x53 => {
                let funct7 = (inst_bytes >> 25) & 0x7f;
                match funct7 {
                    0x0 => inst.id = InstID::FADD_S,
                    0x1 => inst.id = InstID::FADD_D,
                    0x4 => inst.id = InstID::FSUB_S,
                    0x5 => inst.id = InstID::FSUB_D,
                    0x8 => inst.id = InstID::FMUL_S,
                    0x9 => inst.id = InstID::FMUL_D,
                    0xc => inst.id = InstID::FDIV_S,
                    0x10 => match funct3 {
                        0x0 => inst.id = InstID::FSGNJ_S,
                        0x1 => inst.id = InstID::FSGNJN_S,
                        0x2 => inst.id = InstID::FSGNJX_S,
                        _ => self.dump_invalid_inst(inst),
                    },
                    0x14 => {
                        if funct3 == 0 {
                            inst.id = InstID::FMIN_S;
                        } else {
                            inst.id = InstID::FMAX_S;
                        }
                    }
                    0x2c => inst.id = InstID::FSQRT_S,
                    0x50 => match funct3 {
                        0x0 => inst.id = InstID::FLE_S,
                        0x1 => inst.id = InstID::FLT_S,
                        0x2 => inst.id = InstID::FEQ_S,
                        _ => self.dump_invalid_inst(inst),
                    },
                    0x51 => match funct3 {
                        0x0 => inst.id = InstID::FLE_D,
                        0x1 => inst.id = InstID::FLT_D,
                        0x2 => inst.id = InstID::FEQ_D,
                        _ => self.dump_invalid_inst(inst),
                    },
                    0x60 => {
                        let rs2 = (inst_bytes >> 20) & 0x1f;
                        match rs2 {
                            0x0 => inst.id = InstID::FCVT_W_S,
                            0x1 => inst.id = InstID::FCVT_WU_S,
                            0x2 => inst.id = InstID::FCVT_L_S,
                            0x3 => inst.id = InstID::FCVT_LU_S,
                            _ => self.dump_invalid_inst(inst),
                        }
                    }
                    0x68 => {
                        let rs2 = (inst_bytes >> 20) & 0x1f;
                        match rs2 {
                            0x0 => inst.id = InstID::FCVT_S_W,
                            0x1 => inst.id = InstID::FCVT_S_WU,
                            0x2 => inst.id = InstID::FCVT_S_L,
                            0x3 => inst.id = InstID::FCVT_S_LU,
                            _ => self.dump_invalid_inst(inst),
                        }
                    }
                    0x70 => match funct3 {
                        0x0 => inst.id = InstID::FMV_X_W,
                        0x1 => inst.id = InstID::FCLASS_S,
                        _ => self.dump_invalid_inst(inst),
                    },
                    0x71 => match (inst.get_rs2_stype(), funct3) {
                        (0, 0) => inst.id = InstID::FMV_X_D,
                        (0, 1) => inst.id = InstID::FCLASS_D,
                        _ => self.dump_invalid_inst(inst),
                    },
                    0x78 => inst.id = InstID::FMV_W_X,
                    0x79 => inst.id = InstID::FMV_D_X,
                    _ => self.dump_invalid_inst(inst),
                }
            }
            0x63 => match funct3 {
                0x0 => inst.id = InstID::BEQ,
                0x1 => inst.id = InstID::BNE,
                0x4 => inst.id = InstID::BLT,
                0x5 => inst.id = InstID::BGE,
                0x6 => inst.id = InstID::BLTU,
                0x7 => inst.id = InstID::BGEU,
                _ => self.dump_invalid_inst(inst),
            },
            0x67 => inst.id = InstID::JALR,
            0x6f => inst.id = InstID::JAL,
            0x73 => match funct3 {
                0x0 => {
                    let funct12 = (inst_bytes >> 20) & 0xfff;
                    match funct12 {
                        0x000 => inst.id = InstID::ECALL,
                        0x302 => inst.id = InstID::MRET,
                        _ => self.dump_invalid_inst(inst),
                    }
                }
                0x1 => inst.id = InstID::CSRRW,
                0x2 => inst.id = InstID::CSRRS,
                0x5 => inst.id = InstID::CSRRWI,
                0x7 => inst.id = InstID::CSRRCI,
                _ => self.dump_invalid_inst(inst),
            },
            _ => self.dump_invalid_inst(inst),
        }
    }

    fn dump_invalid_inst(&self, inst: &mut InstType) {
        inst.id = InstID::INVALID;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rv_core::inst_type::tests::*;

    #[test]
    fn test_decode() {
        let decoder = InstDecoder::new();
        let inst_golden = inst_auipc_code(0, 0);
        let inst = decoder.decode(inst_golden.data);

        assert_eq!(4, inst.len);
        assert_eq!(InstID::AUIPC, inst.id);
        assert_eq!(inst_golden.data, inst.data);
    }
}
