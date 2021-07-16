use std::collections::HashMap;

struct MemoryModel {
    data: HashMap<u32, [u8; 32]>,
}

impl MemoryModel {
    fn read_byte(&mut self, addr: u32) -> u8 {
        let block_base = addr & 0xffffffe0;
        let block_offset = addr - block_base;
        if !self.data.contains_key(&block_base) {
            self.data.insert(block_base, [0; 32]);
        }

        let block = self.data.get(&block_base).unwrap();
        block[block_offset as usize]
    }

    fn write_byte(&mut self, addr: u32, value: u8) {
        let block_base = addr & 0xffffffe0;
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
}
