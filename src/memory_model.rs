use crate::memory_interface::{MemoryInterface, MemoryOperation, Payload};
use std::collections::HashMap;
type AddressType = u64;

trait MemoryModelConfig {
    const BLOCK_SIZE: usize = 32;
}

impl MemoryModelConfig for MemoryModel {}
pub struct MemoryModel {
    data: HashMap<AddressType, [u8; MemoryModel::BLOCK_SIZE]>,
}


impl MemoryInterface for MemoryModel {
    fn access_memory(&mut self, payload: &mut Payload) {
        self.do_access(payload.addr, &mut payload.data, payload.op);
    }
}

impl MemoryModel {
    pub fn new() -> MemoryModel {
        MemoryModel {
            data: HashMap::new(),
        }
    }

    pub fn read_byte(&mut self, addr: AddressType) -> u8 {
        let mut value = vec![0; 1];
        self.do_access(addr, &mut value, MemoryOperation::READ);
        value[0]
    }

    pub fn write_byte(&mut self, addr: AddressType, value: u8) {
        self.do_access(addr, &mut value.to_le_bytes().to_vec(), MemoryOperation::WRITE);
    }

    pub fn write_word(&mut self, addr: AddressType, value: u32) {
        self.do_access(addr, &mut value.to_le_bytes().to_vec(), MemoryOperation::WRITE);
    }

    fn do_access(&mut self, addr: AddressType, data: &mut Vec<u8>, op: MemoryOperation) {
        match op {
            MemoryOperation::READ => {
                let mut data_offset: usize = 0;
                while data_offset < data.len() {
                    let block_base = (addr + data_offset as AddressType)
                                        & (!(0x1f as AddressType));
                    let mut block_offset: usize = (addr
                                        + data_offset as AddressType - block_base) as usize;
                    if !self.data.contains_key(&block_base) {
                        self.data.insert(block_base, [0; MemoryModel::BLOCK_SIZE]);
                    }

                    let block = self.data.get(&block_base).unwrap();
                    while (block_offset < block.len()) && (data_offset < data.len()) {
                        data[data_offset] = block[block_offset as usize];
                        block_offset += 1;
                        data_offset += 1;
                    }
                }
            }
            MemoryOperation::WRITE => {
                let mut data_offset: usize = 0;
                while data_offset < data.len() {
                    let block_base = (addr + data_offset as AddressType)
                                        & (!(0x1f as AddressType));
                    let mut block_offset: usize = (addr
                                        + data_offset as AddressType - block_base) as usize;
                    if !self.data.contains_key(&block_base) {
                        self.data.insert(block_base, [0; MemoryModel::BLOCK_SIZE]);
                    }

                    let block = self.data.get_mut(&block_base).unwrap();
                    while (block_offset < block.len()) && (data_offset < data.len()) {
                        block[block_offset as usize] = data[data_offset];
                        block_offset += 1;
                        data_offset += 1;
                    }
                }
            }
            MemoryOperation::INVALID => panic!("Invalid mem op"),
        }
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
        assert_eq!([0, 0, 0, 0, 0].to_vec(), payload.data);

        for i in 0..payload.data.len() {
            payload.data[i] = (i as u8 + 1) * 11;
        }
        payload.op = MemoryOperation::WRITE;
        mem.access_memory(&mut payload);
        payload.op = MemoryOperation::READ;
        mem.access_memory(&mut payload);
        assert_eq!([11, 22, 33, 44, 55].to_vec(), payload.data);
    }
}
