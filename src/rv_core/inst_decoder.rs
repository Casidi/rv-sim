use crate::rv_core::inst_info::InstID;
use crate::rv_core::inst_type::InstType;

type AddressType = u64;

#[derive(Default)]
pub struct InstDecoder {}

impl InstDecoder {
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
            0x1 => match funct3 {
                0x0 => inst.id = InstID::C_ADDI,
                0x1 => inst.id = InstID::C_JAL,
                0x2 => inst.id = InstID::C_LI,
                0x4 => {
                    match ((inst_bytes >> 12) & 1, (inst_bytes >> 5) & 0b11) {
                        (0, 0) => inst.id = InstID::C_SUB,
                        (_, _) => self.dump_invalid_inst(inst),
                    }
                }
                0x7 => inst.id = InstID::C_BNEZ,
                _ => self.dump_invalid_inst(inst),
            },
            0x2 => match funct3 {
                0x2 => inst.id = InstID::C_LWSP,
                0x4 => {
                    match ((inst_bytes >> 12) & 1, (inst_bytes >> 2) & 0x1f) {
                        (0, _) => inst.id = InstID::C_MV,
                        (1, _) => inst.id = InstID::C_ADD,
                        (_, _) => self.dump_invalid_inst(inst),
                    }
                }
                0x6 => inst.id = InstID::C_SWSP,
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
            0x23 => match funct3 {
                0x0 => inst.id = InstID::SB,
                _ => self.dump_invalid_inst(inst),
            }
            0x63 => inst.id = InstID::BGEU,
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
        let decoder: InstDecoder = Default::default();
        let inst_golden = inst_auipc_code(0, 0);
        let inst = decoder.decode(inst_golden.data);

        assert_eq!(4, inst.len);
        assert_eq!(InstID::AUIPC, inst.id);
        assert_eq!(inst_golden.data, inst.data);
    }
}
