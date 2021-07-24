mod inst_decoder;
mod inst_info;
mod inst_type;
use crate::rv_core::inst_info::InstID;

#[derive(Default)]
struct Payload {
    addr: u32,
    data: Vec<u8>,
}

trait MemoryInterface {
    fn access_memory(&mut self, payload: &Payload);
}

#[derive(Default)]
pub struct RVCore<'a> {
    pc: u32,
    regs: [u32; 32],
    id_instance: inst_decoder::InstDecoder,
    mem_if: Option<&'a mut dyn MemoryInterface>,
}

impl<'a> RVCore<'a> {
    fn step(&mut self, inst_bytes: u32) {
        let inst = self.id_instance.decode(inst_bytes);
        print!("{}", inst_info::inst_info_table[inst.id as usize].name);
        self.execute(&inst);
        self.pc += inst.len;
    }

    fn execute(&mut self, inst: &inst_type::InstType) {
        match inst.id {
            InstID::AUIPC => self.inst_auipc(inst),
            InstID::ADDI => self.inst_addi(inst),
            InstID::C_ADDI => self.inst_c_addi(inst),
            InstID::C_SWSP => self.inst_c_swsp(inst),
            InstID::NOP => self.inst_nop(inst),
        }
    }

    pub fn run(&mut self, num_steps: i32) {
        let mut step_count = 0;
        while step_count < num_steps {
            self.step(0x00002197); //AUIPC
            step_count += 1;
        }
    }

    fn write_memory(&mut self, address: u32, data: &[u8]) {
        let payload = Payload {
            addr: address,
            data: data.to_vec(),
        };
        self.mem_if.as_mut().unwrap().access_memory(&payload);
    }

    fn bind_mem(&mut self, mem_if: &'a mut dyn MemoryInterface) {
        self.mem_if = Some(mem_if);
    }

    fn inst_nop(&mut self, _inst: &inst_type::InstType) {}

    fn inst_auipc(&mut self, inst: &inst_type::InstType) {
        self.regs[inst.get_rd()] = self.pc + inst.get_imm_utype();
    }

    fn inst_addi(&mut self, inst: &inst_type::InstType) {
        self.regs[inst.get_rd()] = self.regs[inst.get_rs1()] + inst.get_imm_itype();
    }

    fn inst_c_addi(&mut self, inst: &inst_type::InstType) {
        if inst.get_rd() != 0 {
            self.regs[inst.get_rd()] = self.regs[inst.get_rd()] + inst.get_imm_ci();
        }
    }

    fn inst_c_swsp(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_css();
        let address = self.regs[2] + (((imm & 0x3) << 6) | (imm & 0x3c));
        let data = self.regs[inst.get_rs2()];
        self.regs[inst.get_rd()] = self.regs[inst.get_rd()] + inst.get_imm_ci();
        self.write_memory(address, &data.to_le_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct MemoryStub {
        buffer: Payload,
    }

    impl MemoryInterface for MemoryStub {
        fn access_memory(&mut self, payload: &Payload) {
            self.buffer.addr = payload.addr;
            self.buffer.data = payload.data.clone();
        }
    }

    #[test]
    fn test_core_run() {
        let mut core: RVCore = Default::default();
        assert_eq!(0, core.pc);

        core.run(5);
        assert_eq!(20, core.pc);
    }

    #[test]
    fn test_inst_auipc() {
        let mut core: RVCore = Default::default();
        core.pc = 0x1234;
        core.inst_auipc(&inst_type::inst_auipc_code(1, 0xffff1000));
        assert_eq!(0xffff1000 + 0x1234, core.regs[1]);
    }

    #[test]
    fn test_inst_addi() {
        let mut core: RVCore = Default::default();
        core.regs[2] = 0x1234;
        core.inst_addi(&inst_type::inst_addi_code(1, 2, 0x7ff));
        assert_eq!(0x7ff + 0x1234, core.regs[1]);
    }

    #[test]
    fn test_inst_c_addi() {
        let mut core: RVCore = Default::default();
        core.regs[2] = 0x1234;
        core.inst_c_addi(&inst_type::inst_c_addi_code(2, 0x1));
        assert_eq!(0x1235, core.regs[2]);
    }

    #[test]
    fn test_inst_c_swsp() {
        let mut core: RVCore = Default::default();
        let mut mem_stub: MemoryStub = Default::default();
        core.bind_mem(&mut mem_stub);

        core.regs[1] = 0x12345678; // Data
        core.regs[2] = 0x8888; // Address
        core.inst_c_swsp(&inst_type::inst_c_swsp_code(1, 0x4));
        assert_eq!(0x888c, mem_stub.buffer.addr);
        assert_eq!([0x78, 0x56, 0x34, 0x12].to_vec(), mem_stub.buffer.data);
    }
}
