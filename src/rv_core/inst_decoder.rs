use crate::rv_core::inst_info::InstID;
use crate::rv_core::inst_type::InstType;

#[derive(Default)]
pub struct InstDecoder {}

impl InstDecoder {
    pub fn decode(&self, inst_bytes: u32) -> InstType {
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

    fn decode_inst_compressed(&self, inst_bytes: u32, inst: &mut InstType) {
        let opcode = inst_bytes & 0x3;
        let funct3 = (inst_bytes >> 13) & 7;
        match opcode {
            0x1 => match funct3 {
                0x0 => inst.id = InstID::ADDI,
                _ => panic!("Invalid instruction"),
            },
            0x2 => match funct3 {
                0x6 => inst.id = InstID::C_SWSP,
                _ => panic!("Invalid instruction"),
            },
            _ => panic!("Invalid instruction"),
        }
    }

    fn decode_inst_4byte(&self, inst_bytes: u32, inst: &mut InstType) {
        let opcode = inst_bytes & 0x7f;
        let funct3 = (inst_bytes & 0x00007000) >> 12;
        match opcode {
            0x13 => match funct3 {
                0x0 => {
                    inst.id = InstID::ADDI;
                }
                _ => panic!("Invalid instruction"),
            },
            0x17 => {
                inst.id = InstID::AUIPC;
            }
            _ => panic!("Invalid instruction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rv_core::inst_type::*;

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
