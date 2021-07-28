mod inst_type;
mod inst_info;
mod inst_decoder;
use crate::rv_core::inst_info::InstID;
use crate::memory_interface::{Payload, MemoryInterface, MemoryOperation};

struct XRegisters {
    reg_bank: [u32; 32],
}

impl XRegisters {
    fn read(&self, i: usize) -> u32 {
        self.reg_bank[i]
    }

    fn write(&mut self, i: usize, val: u32) {
        if i != 0 {
            self.reg_bank[i] = val;
        }
    }
}

pub struct RVCore<'a> {
    pub pc: u32,
    regs: XRegisters,
    id_instance: inst_decoder::InstDecoder,
    mem_if: Option<&'a mut dyn MemoryInterface>,
}

impl<'a> RVCore<'a> {
    pub fn new() -> RVCore<'a> {
        RVCore {
            pc: 0,
            regs: XRegisters { reg_bank: [0; 32] },
            id_instance: inst_decoder::InstDecoder {},
            mem_if: None,
        }
    }

    fn step(&mut self) {
        let mut data = [0; 4];
        self.read_memory(self.pc, &mut data);
        let inst_bytes = unsafe {std::mem::transmute::<[u8; 4], u32>(data)};

        let inst = self.id_instance.decode(inst_bytes);
        println!("{:#010x} ({:#010x}) {}", self.pc, inst_bytes,
                    inst_info::inst_info_table[inst.id as usize].name);

        self.execute(&inst);
        self.pc += inst.len;
    }

    fn execute(&mut self, inst: &inst_type::InstType) {
        match inst.id {
            InstID::AUIPC => self.inst_auipc(inst),
            InstID::ADDI => self.inst_addi(inst),
            InstID::C_ADDI => self.inst_c_addi(inst),
            InstID::C_SWSP => self.inst_c_swsp(inst),
            InstID::C_LWSP => self.inst_c_lwsp(inst),
            InstID::C_LI => self.inst_c_li(inst),
            InstID::C_MV => self.inst_c_mv(inst),
            InstID::C_SUB => self.inst_c_sub(inst),
            InstID::LW => self.inst_lw(inst),
            InstID::SB => self.inst_sb(inst),
            InstID::NOP => self.inst_nop(inst),
        }
    }

    pub fn run(&mut self, num_steps: i32) {
        let mut step_count = 0;
        while step_count < num_steps {
            self.step();
            step_count += 1;
        }
    }

    fn write_memory(&mut self, address: u32, data: &[u8]) {
        let mut payload = Payload {
            addr: address,
            data: data.to_vec(),
            op: MemoryOperation::WRITE,
        };
        self.mem_if.as_mut().unwrap().access_memory(&mut payload);
    }

    fn read_memory(&mut self, address: u32, data: &mut [u8]) {
        let mut payload = Payload {
            addr: address,
            data: data.to_vec(),
            op: MemoryOperation::READ,
        };
        self.mem_if.as_mut().unwrap().access_memory(&mut payload);

        for i in 0..data.len() {
            data[i] = payload.data[i];
        }
    }

    pub fn bind_mem(&mut self, mem_if: &'a mut dyn MemoryInterface) {
        self.mem_if = Some(mem_if);
    }

    fn inst_nop(&mut self, _inst: &inst_type::InstType) {}

    fn inst_auipc(&mut self, inst: &inst_type::InstType) {
        self.regs.write(inst.get_rd(), self.pc + inst.get_imm_utype());
    }

    fn inst_addi(&mut self, inst: &inst_type::InstType) {
        self.regs.write(inst.get_rd(),
            self.regs.read(inst.get_rs1()) + inst.get_imm_itype());
    }

    fn inst_c_addi(&mut self, inst: &inst_type::InstType) {
        self.regs.write(inst.get_rd(),
            self.regs.read(inst.get_rd()) + inst.get_imm_ci());
    }

    fn inst_c_swsp(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_css();
        let address = self.regs.read(2) + (((imm & 0x3) << 6) | (imm & 0x3c));
        let data = self.regs.read(inst.get_rs2());
        self.write_memory(address, &data.to_le_bytes());
    }

    fn inst_c_lwsp(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_ci();
        let address = self.regs.read(2) + (((imm & 0x3) << 6) | (imm & 0x3c));
        let mut data = [0; 4];
        self.read_memory(address, &mut data);
        self.regs.write(inst.get_rd(), unsafe {std::mem::transmute::<[u8; 4], u32>(data)});
    }

    fn inst_c_li(&mut self, inst: &inst_type::InstType) {
        self.regs.write(inst.get_rd(), inst.get_imm_ci());
    }

    fn inst_c_mv(&mut self, inst: &inst_type::InstType) {
        self.regs.write(inst.get_rd(), self.regs.read(inst.get_rs2()));
    }

    fn inst_c_sub(&mut self, inst: &inst_type::InstType) {
        let a = self.regs.read(inst.get_rd_3b());
        let b = self.regs.read(inst.get_rs2_3b());
        self.regs.write(inst.get_rd_3b(), a - b);
    }

    fn inst_lw(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_itype();
        let address = self.regs.read(inst.get_rs1()) + (imm & 0xfff);
        let mut data = [0; 4];
        self.read_memory(address, &mut data);
        self.regs.write(inst.get_rd(), unsafe {std::mem::transmute::<[u8; 4], u32>(data)});
    }

    fn inst_sb(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_stype();
        let address = self.regs.read(inst.get_rs1()) + (imm & 0xfff);
        let data = self.regs.read(inst.get_rs2_stype());
        self.write_memory(address, &data.to_le_bytes()[..1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inst_type::tests::*;

    #[derive(Default)]
    struct MemoryStub {
        buffer: Payload,
    }

    impl MemoryInterface for MemoryStub {
        fn access_memory(&mut self, payload: &mut Payload) {
            self.buffer.addr = payload.addr;
            self.buffer.data = payload.data.clone();
            self.buffer.op = payload.op;
        }
    }

    #[test]
    fn test_inst_auipc() {
        let mut core: RVCore = RVCore::new();
        core.pc = 0x1234;
        core.inst_auipc(&inst_auipc_code(1, 0xffff1000));
        assert_eq!(0xffff1000 + 0x1234, core.regs.read(1));
    }

    #[test]
    fn test_inst_addi() {
        let mut core: RVCore = RVCore::new();
        core.regs.write(2, 0x1234);
        core.inst_addi(&inst_addi_code(1, 2, 0x7ff));
        assert_eq!(0x7ff + 0x1234, core.regs.read(1));
    }

    #[test]
    fn test_inst_c_addi() {
        let mut core: RVCore = RVCore::new();
        core.regs.write(2, 0x1234);
        core.inst_c_addi(&inst_c_addi_code(2, 0x1));
        assert_eq!(0x1235, core.regs.read(2));
    }

    #[test]
    fn test_inst_c_swsp() {
        let mut core: RVCore = RVCore::new();
        let mut mem_stub: MemoryStub = Default::default();
        core.bind_mem(&mut mem_stub);

        core.regs.write(1, 0x12345678); // Data
        core.regs.write(2, 0x8888); // Address
        core.inst_c_swsp(&inst_c_swsp_code(1, 0x4));
        assert_eq!(MemoryOperation::WRITE, mem_stub.buffer.op);
        assert_eq!(0x888c, mem_stub.buffer.addr);
        assert_eq!([0x78, 0x56, 0x34, 0x12].to_vec(), mem_stub.buffer.data);
    }

    #[test]
    fn test_inst_c_lwsp() {
        let mut core: RVCore = RVCore::new();
        let mut mem_stub: MemoryStub = Default::default();
        core.bind_mem(&mut mem_stub);

        core.regs.write(2, 0x8888); // Address
        core.inst_c_lwsp(&inst_c_lwsp_code(1, 0x4));
        assert_eq!(MemoryOperation::READ, mem_stub.buffer.op);
        assert_eq!(0x888c, mem_stub.buffer.addr);
    }

    #[test]
    fn test_inst_c_li() {
        let mut core: RVCore = RVCore::new();
        core.regs.write(2, 0x0);
        core.inst_c_li(&inst_c_li_code(2, 0x1f));
        assert_eq!(0x1f, core.regs.read(2));
    }

    #[test]
    fn test_inst_c_mv() {
        let mut core: RVCore = RVCore::new();
        core.regs.write(2, 0x0);
        core.regs.write(3, 0xfafafafa);
        core.inst_c_mv(&inst_c_mv_code(2, 3));
        assert_eq!(0xfafafafa, core.regs.read(2));
    }

    #[test]
    fn test_inst_c_sub() {
        let mut core: RVCore = RVCore::new();
        core.regs.write(8, 0xffffffff);
        core.regs.write(9, 0xf0f00f0f);
        core.inst_c_sub(&inst_c_sub_code(8, 9));
        assert_eq!(0x0f0ff0f0, core.regs.read(8));
    }

    #[test]
    fn test_inst_lw() {
        let mut core: RVCore = RVCore::new();
        let mut mem_stub: MemoryStub = Default::default();
        core.bind_mem(&mut mem_stub);

        core.regs.write(1, 0x8888); // Address
        core.inst_lw(&inst_lw_code(2, 1, 0xff0));
        assert_eq!(MemoryOperation::READ, mem_stub.buffer.op);
        assert_eq!(0x8888 + 0xff0, mem_stub.buffer.addr);
    }

    #[test]
    fn test_inst_sb() {
        let mut core: RVCore = RVCore::new();
        let mut mem_stub: MemoryStub = Default::default();
        core.bind_mem(&mut mem_stub);

        core.regs.write(1, 0xffffff78); // Data
        core.regs.write(2, 0x8888); // Address
        core.inst_sb(&inst_sb_code(1, 2, 0xff));
        assert_eq!(MemoryOperation::WRITE, mem_stub.buffer.op);
        assert_eq!(0x8888 + 0xff, mem_stub.buffer.addr);
        assert_eq!([0x78].to_vec(), mem_stub.buffer.data);
    }
}
