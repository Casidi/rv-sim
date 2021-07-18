use crate::rv_core::inst_type::InstType;
use crate::rv_core::RVCore;

#[derive(Default)]
pub struct InstDecoder {
}

impl InstDecoder {
	pub fn decode(&self, inst_bytes: u32) -> InstType {
        let mut new_inst = InstType {
            data: inst_bytes,
            len: 0,
            operate: RVCore::inst_nop,
        };
        match inst_bytes & 0b11 {
            0 | 1 | 2 => {
                new_inst.len = 2;
            }
            _ => {
                self.decode_inst_4byte(inst_bytes, &mut new_inst);
                new_inst.len = 4;
            }
        }

        new_inst
	}

    fn decode_inst_4byte(&self, inst_bytes: u32, inst: &mut InstType) {
        let opcode = inst_bytes & 0x7f;
        let funct3 = (inst_bytes & 0x00007000) >> 12;
        match opcode {
            0x13 => match funct3 {
                0x0 => {
                    inst.operate = RVCore::inst_addi;
                }
                _ => panic!("Invalid instruction"),
            },
            0x17 => {
                inst.operate = RVCore::inst_auipc;
            }
            _ => panic!("Invalid instruction"),
        }
    }
}
