mod inst_type;
mod inst_info;
mod inst_decoder;
use crate::rv_core::inst_info::InstID;
use crate::memory_interface::{Payload, MemoryInterface, MemoryOperation};

type AddressType = u64;

struct XRegisters {
    reg_bank: [AddressType; 32],
}

impl XRegisters {
    fn read(&self, i: usize) -> AddressType {
        self.reg_bank[i]
    }

    fn write(&mut self, i: usize, val: AddressType) {
        if i != 0 {
            self.reg_bank[i] = val;
        }
    }

	fn name(i: usize) -> &'static str {
		match i {
			0 => "zero",
			1 => "ra",
			2 => "sp",
			3 => "gp",
			4 => "tp",
			5 => "t0",
			6 => "t1",
			7 => "t2",
			8 => "s0",
			9 => "s1",
			10 => "a0",
			11 => "a1",
			12 => "a2",
			13 => "a3",
			14 => "a4",
			15 => "a5",
			16 => "a6",
			17 => "a7",
			18 => "s2",
			19 => "s3",
			20 => "s4",
			21 => "s5",
			22 => "s6",
			23 => "s7",
			24 => "s8",
			25 => "s9",
			26 => "s10",
			27 => "s11",
			28 => "t3",
			29 => "t4",
			30 => "t5",
			31 => "t6",
			_ => "invalid gpr name"
		}
	}
}

pub struct RVCore<'a> {
    pub pc: AddressType,
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
        let mut data = [0; std::mem::size_of::<AddressType>()];
        self.read_memory(self.pc, &mut data);
        let inst_bytes = RVCore::byte_array_to_addr_type(&data);

        let inst = self.id_instance.decode(inst_bytes);
        if inst.id == InstID::INVALID {
            println!("{:#010x} ({:#010x}) INVALID", self.pc, inst_bytes as u32);
            panic!("Invalid instruction");
        }

        print!("{:#010x} ({:#010x}) {}", self.pc, inst_bytes as u32,
                    inst_info::inst_info_table[inst.id as usize].name);

        self.execute(&inst);
		println!("");
        match inst.id {
            InstID::C_J => {}
            InstID::C_JAL => {}
            InstID::C_JR => {}
            InstID::C_BNEZ => {}
            InstID::JAL => {}
            InstID::JALR => {}
            InstID::BGEU => {}
            _ => self.pc += inst.len,
        }
    }

    fn execute(&mut self, inst: &inst_type::InstType) {
        match inst.id {
            InstID::AUIPC => self.inst_auipc(inst),
            InstID::ADDI => self.inst_addi(inst),
            InstID::ANDI => self.inst_andi(inst),
            InstID::BGEU => self.inst_bgeu(inst),
            InstID::C_ADD => self.inst_c_add(inst),
            InstID::C_ADDI => self.inst_c_addi(inst),
            InstID::C_ANDI => self.inst_c_andi(inst),
            InstID::C_BNEZ => self.inst_c_bnez(inst),
            InstID::C_J => self.inst_c_j(inst),
            InstID::C_JAL => self.inst_c_jal(inst),
            InstID::C_JR => self.inst_c_jr(inst),
            InstID::C_SWSP => self.inst_c_swsp(inst),
            InstID::C_LWSP => self.inst_c_lwsp(inst),
            InstID::C_LI => self.inst_c_li(inst),
            InstID::C_MV => self.inst_c_mv(inst),
            InstID::C_SUB => self.inst_c_sub(inst),
            InstID::C_SD => self.inst_c_sd(inst),
            InstID::JAL => self.inst_jal(inst),
            InstID::JALR => self.inst_jalr(inst),
            InstID::LW => self.inst_lw(inst),
            InstID::SB => self.inst_sb(inst),
            InstID::SLLI => self.inst_slli(inst),
            InstID::SRLI => self.inst_srli(inst),
            InstID::SRAI => self.inst_srai(inst),
            InstID::NOP => self.inst_nop(inst),
            InstID::INVALID => panic!("Execute: invalid instruction"),
        }
    }

    pub fn run(&mut self, num_steps: i32) {
        let mut step_count = 0;
        while step_count < num_steps {
            self.step();
            step_count += 1;
        }
    }

    fn write_memory(&mut self, address: AddressType, data: &[u8]) {
        let mut payload = Payload {
            addr: address,
            data: data.to_vec(),
            op: MemoryOperation::WRITE,
        };
        self.mem_if.as_mut().unwrap().access_memory(&mut payload);
    }

    fn read_memory(&mut self, address: AddressType, data: &mut [u8]) {
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

    fn byte_array_to_addr_type(data: &[u8; 8]) -> AddressType {
        unsafe {std::mem::transmute::<[u8; std::mem::size_of::<AddressType>()],
                AddressType>(*data)}
    }

    fn byte_array_to_addr_type_32b(data: &[u8; 4]) -> AddressType {
        (unsafe {std::mem::transmute::<[u8; 4], u32>(*data)}) as AddressType
    }

    fn inst_nop(&mut self, _inst: &inst_type::InstType) {}

    fn inst_auipc(&mut self, inst: &inst_type::InstType) {
        self.regs.write(inst.get_rd(), self.pc + inst.get_imm_utype());
    }

    fn inst_addi(&mut self, inst: &inst_type::InstType) {
        self.regs.write(inst.get_rd(),
            self.regs.read(inst.get_rs1()) + inst.get_imm_itype());
    }

    fn inst_andi(&mut self, inst: &inst_type::InstType) {
        self.regs.write(inst.get_rd(),
            self.regs.read(inst.get_rs1()) & inst.get_imm_itype());
    }

    fn inst_bgeu(&mut self, inst: &inst_type::InstType) {
		let imm = inst.get_imm_btype();
		let offset = (((imm >> 11) & 1) << 12)
						| (((imm >> 5) & 0x3f) << 5)
						| (((imm >> 1) & 0xf) << 1)
						| (((imm >> 0) & 1) << 11);
		print!(" {},{},{:x}", XRegisters::name(inst.get_rs1()), XRegisters::name(inst.get_rs2_btype()), self.pc + offset);
		if self.regs.read(inst.get_rs1()) >= self.regs.read(inst.get_rs2_btype()) {
			self.pc += offset;
		} else {
			self.pc += 4;
		}
    }

    fn inst_c_add(&mut self, inst: &inst_type::InstType) {
        self.regs.write(inst.get_rd(), self.regs.read(inst.get_rd()) + self.regs.read(inst.get_rs2()));
    }

    fn inst_c_addi(&mut self, inst: &inst_type::InstType) {
        self.regs.write(inst.get_rd(),
            self.regs.read(inst.get_rd()) + inst.get_imm_ci());
    }

    fn inst_c_andi(&mut self, inst: &inst_type::InstType) {
        let a = self.regs.read(inst.get_rs1_3b());
        let imm_cb = inst.get_imm_cb();
        let imm = RVCore::sign_extend((imm_cb & 0x1f) | (((imm_cb >> 7) & 1) << 5), 6);
        self.regs.write(inst.get_rs1_3b(), a & imm);
    }

    fn inst_c_bnez(&mut self, inst: &inst_type::InstType) {
		let rs1_val = self.regs.read(inst.get_rs1_3b());
		let imm = inst.get_imm_cb();
		let offset = (((imm >> 7) & 0x1) << 8)
						| (((imm >> 5) & 0x3) << 3)
						| (((imm >> 3) & 0x3) << 6)
						| (((imm >> 1) & 0x3) << 1)
						| (((imm >> 0) & 0x1) << 5);
		print!(" {},{:x}", XRegisters::name(inst.get_rs1_3b()), self.pc + offset);
		if rs1_val != 0 {
			self.pc += offset;
		} else {
			self.pc += 2;
		}
    }

    fn inst_c_j(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_cj();
        let mut offset = ((imm >> 10) & 1) << 11;
        offset |= ((imm >> 9) & 1) << 4;
        offset |= ((imm >> 7) & 3) << 8;
        offset |= ((imm >> 6) & 1) << 10;
        offset |= ((imm >> 5) & 1) << 6;
        offset |= ((imm >> 4) & 1) << 7;
        offset |= ((imm >> 1) & 7) << 1;
        offset |= ((imm >> 0) & 1) << 5;
		let offset_with_sign = RVCore::sign_extend(offset, 12);
        self.pc = self.pc.wrapping_add(offset_with_sign);
    }

    fn inst_c_jal(&mut self, inst: &inst_type::InstType) {
        self.regs.write(1, self.pc + 2);
        let imm = inst.get_imm_cj();
        let mut offset = ((imm >> 10) & 1) << 11;
        offset |= ((imm >> 9) & 1) << 4;
        offset |= ((imm >> 7) & 3) << 8;
        offset |= ((imm >> 6) & 1) << 10;
        offset |= ((imm >> 5) & 1) << 6;
        offset |= ((imm >> 4) & 1) << 7;
        offset |= ((imm >> 1) & 7) << 1;
        offset |= ((imm >> 0) & 1) << 5;
        self.pc += offset;
    }

    fn inst_c_jr(&mut self, inst: &inst_type::InstType) {
		print!(" {}", XRegisters::name(inst.get_rs1_cr()));
		self.pc = self.regs.read(inst.get_rs1_cr());
    }

    fn inst_c_swsp(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_css();
        let address = self.regs.read(2) + (((imm & 0x3) << 6) | (imm & 0x3c));
        let data = self.regs.read(inst.get_rs2()) as u32;
        self.write_memory(address, &data.to_le_bytes());
    }

    fn inst_c_lwsp(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_ci();
        let address = self.regs.read(2) + (((imm & 0x3) << 6) | (imm & 0x3c));
        let mut data = [0; 4];
        self.read_memory(address, &mut data);
        self.regs.write(inst.get_rd(), RVCore::byte_array_to_addr_type_32b(&data));
    }

    fn inst_c_li(&mut self, inst: &inst_type::InstType) {
        self.regs.write(inst.get_rd(), inst.get_imm_ci());
    }

    fn inst_c_mv(&mut self, inst: &inst_type::InstType) {
        self.regs.write(inst.get_rd(), self.regs.read(inst.get_rs2()));
    }

    fn inst_c_sd(&mut self, inst: &inst_type::InstType) {
        let imm_cs = inst.get_imm_cs();
        let offset = (((imm_cs >> 2) & 0x7) << 3) | (((imm_cs >> 0) & 0x3) << 6);
        let address = self.regs.read(inst.get_rs1_3b()) + offset;
        let data = self.regs.read(inst.get_rs2_3b());
        self.write_memory(address, &data.to_le_bytes());
    }

    fn inst_c_sub(&mut self, inst: &inst_type::InstType) {
        let a = self.regs.read(inst.get_rd_3b());
        let b = self.regs.read(inst.get_rs2_3b());
        self.regs.write(inst.get_rd_3b(), a.wrapping_sub(b));
    }

    fn inst_jal(&mut self, inst: &inst_type::InstType) {
        self.regs.write(inst.get_rd(), self.pc + 4);
        let imm = inst.get_imm_jtype();
        self.pc += (((imm >> 19) & 1) << 20)
                    | (((imm >> 9) & 0x3ff) << 1)
                    | (((imm >> 8) & 0x1) << 11)
                    | (((imm >> 0) & 0xff) << 12);
		print!(" {},{:x}", XRegisters::name(inst.get_rd()), self.pc);
    }

	fn sign_extend(mut input: AddressType, input_bit_len: usize) -> AddressType {
		let mask = 1 << (input_bit_len - 1);
		input &= (1 << input_bit_len) - 1;
		(input ^ mask).wrapping_sub(mask)
	}

    fn inst_jalr(&mut self, inst: &inst_type::InstType) {
        self.regs.write(inst.get_rd(), self.pc + 4);
		let offset = RVCore::sign_extend(inst.get_imm_itype(), 12);
        self.pc = (self.regs.read(inst.get_rs1()).wrapping_add(offset)) & !1u64;
		print!(" {},{:x}({})", XRegisters::name(inst.get_rd()), self.pc, XRegisters::name(inst.get_rs1()));
    }

    fn inst_lw(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_itype();
        let address = self.regs.read(inst.get_rs1()) + (imm & 0xfff);
        let mut data = [0; std::mem::size_of::<AddressType>()];
        self.read_memory(address, &mut data);
        self.regs.write(inst.get_rd(), RVCore::byte_array_to_addr_type(&data));
    }

    fn inst_sb(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_stype();
        let address = self.regs.read(inst.get_rs1()) + (imm & 0xfff);
        let data = self.regs.read(inst.get_rs2_stype());
        self.write_memory(address, &data.to_le_bytes()[..1]);
    }

    fn inst_slli(&mut self, inst: &inst_type::InstType) {
        let shamt = inst.get_shamt_itype();
		let rs1_val = self.regs.read(inst.get_rs1());
        self.regs.write(inst.get_rd(), rs1_val << shamt);
    }

    fn inst_srli(&mut self, inst: &inst_type::InstType) {
        let shamt = inst.get_shamt_itype();
		let rs1_val = self.regs.read(inst.get_rs1());
        self.regs.write(inst.get_rd(), rs1_val >> shamt);
    }

    fn inst_srai(&mut self, inst: &inst_type::InstType) {
        let shamt = inst.get_shamt_itype();
		let rs1_val = self.regs.read(inst.get_rs1());
        self.regs.write(inst.get_rd(), ((rs1_val as i64) >> shamt) as AddressType);
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
    fn test_inst_andi() {
        let mut core: RVCore = RVCore::new();
        core.regs.write(2, 0x1234);
        core.inst_andi(&inst_andi_code(1, 2, 0x7ff));
        assert_eq!(0x7ff & 0x1234, core.regs.read(1));
    }

    #[test]
    fn test_inst_bgeu() {
        let mut core: RVCore = RVCore::new();
        core.regs.write(2, 0x1234);
        core.regs.write(3, 0x1234);
        core.inst_bgeu(&inst_bgeu_code(2, 3, 0xffe));
        assert_eq!(0xffe, core.pc);

		core.pc = 0;
        core.regs.write(2, 0x1230);
        core.regs.write(3, 0x1234);
        core.inst_bgeu(&inst_bgeu_code(2, 3, 0xffe));
        assert_eq!(0x4, core.pc);
    }

    #[test]
    fn test_inst_c_add() {
        let mut core: RVCore = RVCore::new();
        core.regs.write(2, 0xff);
        core.regs.write(3, 0xfafafafa);
        core.inst_c_add(&inst_c_add_code(2, 3));
        assert_eq!(0xfafafafa + 0xff, core.regs.read(2));
    }

    #[test]
    fn test_inst_c_addi() {
        let mut core: RVCore = RVCore::new();
        core.regs.write(2, 0x1234);
        core.inst_c_addi(&inst_c_addi_code(2, 0x1));
        assert_eq!(0x1235, core.regs.read(2));
    }

    #[test]
    fn test_inst_c_andi() {
        let mut core: RVCore = RVCore::new();
        core.regs.write(8, 0b111000111);
        core.inst_c_andi(&inst_c_andi_code(8, 0b111100));
        assert_eq!(0b111000100, core.regs.read(8));
    }

    #[test]
    fn test_inst_c_bnez() {
        let mut core: RVCore = RVCore::new();
        core.regs.write(10, 1);
        core.inst_c_bnez(&inst_c_bnez_code(10, 0xfe));
        assert_eq!(0xfe, core.pc);

		core.pc = 0;
        core.regs.write(10, 0);
        core.inst_c_bnez(&inst_c_bnez_code(10, 0xfe));
        assert_eq!(2, core.pc);
    }

    #[test]
    fn test_inst_c_j() {
        let mut core: RVCore = RVCore::new();
        core.pc = 0xfff0;
        core.inst_c_j(&inst_c_j_code(0xfe));
        assert_eq!(0xfff0 + 0xfe, core.pc);
    }

    #[test]
    fn test_inst_c_jal() {
        let mut core: RVCore = RVCore::new();
        core.pc = 0xfff0;
        core.inst_c_jal(&inst_c_jal_code(0xffe));
        assert_eq!(0xfff2, core.regs.read(1));
        assert_eq!(0xfff0 + 0xffe, core.pc);
    }

    #[test]
    fn test_inst_c_jr() {
        let mut core: RVCore = RVCore::new();
        core.pc = 0x0;
		core.regs.write(8, 0x6666);
        core.inst_c_jr(&inst_c_jr_code(8));
        assert_eq!(0x6666, core.pc);
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
    fn test_inst_c_sd() {
        let mut core: RVCore = RVCore::new();
        let mut mem_stub: MemoryStub = Default::default();
        core.bind_mem(&mut mem_stub);

        core.regs.write(8, 0x12345678); // Data
        core.regs.write(9, 0x8888); // Address
        core.inst_c_sd(&inst_c_sd_code(8, 9, 0x18));
        assert_eq!(MemoryOperation::WRITE, mem_stub.buffer.op);
        assert_eq!(0x8888 + 0x18, mem_stub.buffer.addr);
        assert_eq!([0x78, 0x56, 0x34, 0x12, 0,0,0,0].to_vec(), mem_stub.buffer.data);
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
    fn test_inst_jal() {
        let mut core: RVCore = RVCore::new();
        core.regs.write(8, 0xffffffff);
        core.inst_jal(&inst_jal_code(8, 0xff00));
        assert_eq!(4, core.regs.read(8));
        assert_eq!(0xff00, core.pc);
    }

    #[test]
    fn test_inst_jalr() {
        let mut core: RVCore = RVCore::new();
        core.regs.write(8, 0xffffffff);
        core.regs.write(9, 0x66);
        core.inst_jalr(&inst_jalr_code(8, 9, 0xfff));
        assert_eq!(4, core.regs.read(8));
        assert_eq!(0x66 - 2, core.pc);
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

    #[test]
    fn test_inst_slli() {
        let mut core: RVCore = RVCore::new();

        core.regs.write(1, 0x0); // rd
        core.regs.write(2, 0xff); // rs1
        core.inst_slli(&inst_slli_code(1, 2, 0x10));
		assert_eq!(0xff << 16, core.regs.read(1));
    }

    #[test]
    fn test_inst_srli() {
        let mut core: RVCore = RVCore::new();

        core.regs.write(1, 0x0); // rd
        core.regs.write(2, 0xff0000); // rs1
        core.inst_srli(&inst_srli_code(1, 2, 0x10));
		assert_eq!(0xff, core.regs.read(1));
    }

    #[test]
    fn test_inst_srai() {
        let mut core: RVCore = RVCore::new();

        core.regs.write(1, 0x0); // rd
        core.regs.write(2, AddressType::MAX); // rs1
        core.inst_srai(&inst_srai_code(1, 2, 0x10));
		assert_eq!(AddressType::MAX, core.regs.read(1));
    }
}
