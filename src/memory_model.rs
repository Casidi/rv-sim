use std::collections::HashMap;
use crate::memory_interface::{Payload, MemoryInterface, MemoryOperation};
type AddressType = u64;

pub struct MemoryModel {
    data: HashMap<AddressType, [u8; 32]>,
}

impl MemoryInterface for MemoryModel {
    fn access_memory(&mut self, payload: &mut Payload) {
        match payload.op {
            MemoryOperation::READ => {
                for i in 0..payload.data.len() {
                    payload.data[i] = self.read_byte(payload.addr + i as AddressType);
                }
            }
            MemoryOperation::WRITE => {
                for i in 0..payload.data.len() {
                    self.write_byte(payload.addr + i as AddressType, payload.data[i]);
                }
            }
            MemoryOperation::INVALID => panic!("Invalid mem op"),
        }
    }
}

impl MemoryModel {
    pub fn new() -> MemoryModel {
        MemoryModel {
            data: HashMap::new(),
        }
    }

    pub fn read_byte(&mut self, addr: AddressType) -> u8 {
        let block_base = addr & 0xffffffe0;
        let block_offset = addr - block_base;
        if !self.data.contains_key(&block_base) {
            self.data.insert(block_base, [0; 32]);
        }

        let block = self.data.get(&block_base).unwrap();
        block[block_offset as usize]
    }

    pub fn write_byte(&mut self, addr: AddressType, value: u8) {
        let block_base = addr & (AddressType::MAX & !(0x1f as AddressType));
        let block_offset = addr - block_base;
        if !self.data.contains_key(&block_base) {
            self.data.insert(block_base, [0; 32]);
        }

        let block = self.data.get_mut(&block_base).unwrap();
        block[block_offset as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem() {
        let mut mem = MemoryModel {
            data: HashMap::new(),
        };
        mem.write_byte(0x12345678, 0);
        assert_eq!(mem.read_byte(0x12345678), 0);
        mem.write_byte(0x12345678, 0xff);
        assert_eq!(mem.read_byte(0x12345678), 0xff);
    }

    #[test]
    fn test_access_mem() {
        let mut mem = MemoryModel {
            data: HashMap::new(),
        };
        let mut payload = Payload {
            addr: 0x66666666,
            data: [0, 0, 0, 0, 0].to_vec(),
            op: MemoryOperation::WRITE,
        };

        mem.access_memory(&mut payload);
        payload.op = MemoryOperation::READ;
        mem.access_memory(&mut payload);
        assert_eq!([0,0,0,0,0].to_vec(), payload.data);

        for i in 0..payload.data.len() {
            payload.data[i] = (i as u8 + 1) * 11;
        }
        payload.op = MemoryOperation::WRITE;
        mem.access_memory(&mut payload);
        payload.op = MemoryOperation::READ;
        mem.access_memory(&mut payload);
        assert_eq!([11,22,33,44,55].to_vec(), payload.data);
    }
}
