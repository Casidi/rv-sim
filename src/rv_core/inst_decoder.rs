use crate::rv_core::inst_info::InstID;
use crate::rv_core::inst_type::InstType;

type AddressType = u64;
enum ArchType {
    RV32,
    RV64,
}

pub struct InstDecoder {
    arch: ArchType,
}

impl InstDecoder {
    pub fn new() -> InstDecoder {
        InstDecoder {
            arch: ArchType::RV64
        }
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
            }
            0x1 => match funct3 {
                0x0 => inst.id = InstID::C_ADDI,
                0x1 => match self.arch {
                    ArchType::RV32 => inst.id = InstID::C_JAL,
                    ArchType::RV64 => inst.id = InstID::C_ADDIW,
                    _ => panic!("Invalid arch type"),
                }
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
                        0x2 => inst.id = InstID::C_ANDI,
                        0x3 =>  {
                            match ((inst_bytes >> 12) & 1, (inst_bytes >> 5) & 0b11) {
                                (0, 0) => inst.id = InstID::C_SUB,
                                (_, _) => self.dump_invalid_inst(inst),
                            }
                        }
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
                0x4 => {
                    match ((inst_bytes >> 12) & 1, (inst_bytes >> 2) & 0x1f) {
                        (0, 0) => inst.id = InstID::C_JR,
                        (0, _) => inst.id = InstID::C_MV,
                        (1, 0) => inst.id = InstID::C_JALR,
                        (1, _) => inst.id = InstID::C_ADD,
                        (_, _) => self.dump_invalid_inst(inst),
                    }
                }
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
                0x2 => inst.id = InstID::LW,
                0x3 => inst.id = InstID::LD,
                _ => self.dump_invalid_inst(inst),
            }
            0x13 => match funct3 {
                0x0 => inst.id = InstID::ADDI,
                0x1 => inst.id = InstID::SLLI,
                0x5 => match (inst_bytes >> 25) & 0x7f {
					0x0 => inst.id = InstID::SRLI,
					0x20 => inst.id = InstID::SRAI,
					_ => self.dump_invalid_inst(inst),
				}
                0x7 => inst.id = InstID::ANDI,
                _ => self.dump_invalid_inst(inst),
            }
            0x17 => {
                inst.id = InstID::AUIPC;
            }
            0x1b => match funct3 {
                0x0 => inst.id = InstID::ADDIW,
                _ => self.dump_invalid_inst(inst),
            }
            0x23 => match funct3 {
                0x0 => inst.id = InstID::SB,
                0x2 => inst.id = InstID::SW,
                0x3 => inst.id = InstID::SD,
                _ => self.dump_invalid_inst(inst),
            }
            0x33 => match funct3 {
                0x0 => {
                    let funct7 = (inst_bytes >> 25) & 0x7f;
                    match funct7 {
                        0x20 => inst.id = InstID::SUB,
                        _ => self.dump_invalid_inst(inst),
                    }
                }
                _ => self.dump_invalid_inst(inst),
            }
            0x3b => match funct3 {
                0x1 => {
                    let funct7 = (inst_bytes >> 25) & 0x7f;
                    match funct7 {
                        0x0 => inst.id = InstID::SLLW,
                        _ => self.dump_invalid_inst(inst),
                    }
                }
                _ => self.dump_invalid_inst(inst),
            }
            0x63 => match funct3 {
                0x0 => inst.id = InstID::BEQ,
                0x1 => inst.id = InstID::BNE,
                0x4 => inst.id = InstID::BLT,
                0x6 => inst.id = InstID::BLTU,
                0x7 => inst.id = InstID::BGEU,
                _ => self.dump_invalid_inst(inst),
            }
            0x67 => inst.id = InstID::JALR,
            0x6f => inst.id = InstID::JAL,
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
