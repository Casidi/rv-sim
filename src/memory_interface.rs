type AddressType = u64;

#[derive(PartialEq, Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum MemoryOperation {
    READ,
    WRITE,
    INVALID,
}

impl Default for MemoryOperation {
    fn default() -> MemoryOperation {
        MemoryOperation::INVALID
    }
}

#[derive(Default)]
pub struct Payload {
    pub addr: AddressType,
    pub data: Vec<u8>,
    pub op: MemoryOperation,
}

pub trait MemoryInterface {
    fn access_memory(&mut self, payload: &mut Payload);
}
