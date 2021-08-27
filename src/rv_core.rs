mod inst_decoder;
mod inst_info;
mod inst_type;
use crate::memory_interface::{MemoryInterface, MemoryOperation, Payload};
use crate::rv_core::inst_info::InstID;

type AddressType = u64;

pub struct XRegisters {
    reg_bank: [AddressType; 32],
}

impl XRegisters {
    fn read(&self, i: usize) -> AddressType {
        self.reg_bank[i]
    }

    pub fn write(&mut self, i: usize, val: AddressType) {
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
            _ => "invalid gpr name",
        }
    }
}

pub struct FRegisters {
    reg_bank: [f64; 32],
}

impl FRegisters {
    fn read(&self, i: usize) -> f64 {
        self.reg_bank[i]
    }

    pub fn write(&mut self, i: usize, val: f64) {
        if i != 0 {
            self.reg_bank[i] = val;
        }
    }

    fn name(i: usize) -> &'static str {
        match i {
            0 => "ft0",
            1 => "ft1",
            2 => "ft2",
            3 => "ft3",
            4 => "ft4",
            5 => "ft5",
            6 => "ft6",
            7 => "ft7",
            8 => "fs0",
            9 => "fs1",
            10 => "fa0",
            11 => "fa1",
            12 => "fa2",
            13 => "fa3",
            14 => "fa4",
            15 => "fa5",
            16 => "fa6",
            17 => "fa7",
            18 => "fs2",
            19 => "fs3",
            20 => "fs4",
            21 => "fs5",
            22 => "fs6",
            23 => "fs7",
            24 => "fs8",
            25 => "fs9",
            26 => "fs10",
            27 => "fs11",
            28 => "ft8",
            29 => "ft9",
            30 => "ft10",
            31 => "ft11",
            _ => "inalid fpr name",
        }
    }
}

pub struct RVCore<'a> {
    pub pc: AddressType,
    pub regs: XRegisters,
    pub fregs: FRegisters,
    id_instance: inst_decoder::InstDecoder,
    mem_if: Option<&'a mut dyn MemoryInterface>,
}

impl<'a> RVCore<'a> {
    pub fn new() -> RVCore<'a> {
        RVCore {
            pc: 0,
            regs: XRegisters { reg_bank: [0; 32] },
            fregs: FRegisters { reg_bank: [0.0; 32] },
            id_instance: inst_decoder::InstDecoder::new(),
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

        print!(
            "PC={:#010x}, {}",
            self.pc,
            inst_info::inst_info_table[inst.id as usize].name
        );
        for i in 0..32 {
            print!(", r{}={:08x}", i, self.regs.read(i));
        }

        self.execute(&inst);
        println!("");
        match inst.id {
            InstID::C_J => {}
            InstID::C_JAL => {}
            InstID::C_JALR => {}
            InstID::C_JR => {}
            InstID::C_BNEZ => {}
            InstID::C_BEQZ => {}
            InstID::JAL => {}
            InstID::JALR => {}
            InstID::BEQ => {}
            InstID::BGE => {}
            InstID::BGEU => {}
            InstID::BLT => {}
            InstID::BLTU => {}
            InstID::BNE => {}
            _ => self.pc += inst.len,
        }
    }

    fn execute(&mut self, inst: &inst_type::InstType) {
        match inst.id {
            InstID::AUIPC => self.inst_auipc(inst),
            InstID::ADD => self.inst_add(inst),
            InstID::ADDI => self.inst_addi(inst),
            InstID::ADDIW => self.inst_addiw(inst),
            InstID::ADDW => self.inst_addw(inst),
            InstID::ANDI => self.inst_andi(inst),
            InstID::BEQ => self.inst_beq(inst),
            InstID::BLT => self.inst_blt(inst),
            InstID::BLTU => self.inst_bltu(inst),
            InstID::BGEU => self.inst_bgeu(inst),
            InstID::BGE => self.inst_bge(inst),
            InstID::BNE => self.inst_bne(inst),
            InstID::C_ADD => self.inst_c_add(inst),
            InstID::C_ADDI => self.inst_c_addi(inst),
            InstID::C_ADDIW => self.inst_c_addiw(inst),
            InstID::C_ADDI16SP => self.inst_c_addi16sp(inst),
            InstID::C_ADDI4SPN => self.inst_c_addi4spn(inst),
            InstID::C_ADDW => self.inst_c_addw(inst),
            InstID::C_AND => self.inst_c_and(inst),
            InstID::C_ANDI => self.inst_c_andi(inst),
            InstID::C_BEQZ => self.inst_c_beqz(inst),
            InstID::C_BNEZ => self.inst_c_bnez(inst),
            InstID::C_FSDSP => self.inst_c_fsdsp(inst),
            InstID::C_J => self.inst_c_j(inst),
            InstID::C_JAL => self.inst_c_jal(inst),
            InstID::C_JALR => self.inst_c_jalr(inst),
            InstID::C_JR => self.inst_c_jr(inst),
            InstID::C_OR => self.inst_c_or(inst),
            InstID::C_SDSP => self.inst_c_sdsp(inst),
            InstID::C_SLLI => self.inst_c_slli(inst),
            InstID::C_SRLI => self.inst_c_srli(inst),
            InstID::C_SW => self.inst_c_sw(inst),
            InstID::C_SWSP => self.inst_c_swsp(inst),
            InstID::C_LD => self.inst_c_ld(inst),
            InstID::C_LDSP => self.inst_c_ldsp(inst),
            InstID::C_LW => self.inst_c_lw(inst),
            InstID::C_LWSP => self.inst_c_lwsp(inst),
            InstID::C_LI => self.inst_c_li(inst),
            InstID::C_LUI => self.inst_c_lui(inst),
            InstID::C_MV => self.inst_c_mv(inst),
            InstID::C_SUB => self.inst_c_sub(inst),
            InstID::C_SD => self.inst_c_sd(inst),
            InstID::C_XOR => self.inst_c_xor(inst),
            InstID::CSRRS => self.inst_csrrs(inst),
            InstID::CSRRW => self.inst_csrrw(inst),
            InstID::ECALL => self.inst_ecall(inst),
            InstID::FMV_W_X => self.inst_fmv_w_x(inst),
            InstID::JAL => self.inst_jal(inst),
            InstID::JALR => self.inst_jalr(inst),
            InstID::LBU => self.inst_lbu(inst),
            InstID::LD => self.inst_ld(inst),
            InstID::LUI => self.inst_lui(inst),
            InstID::LW => self.inst_lw(inst),
            InstID::MULW => self.inst_mulw(inst),
            InstID::OR => self.inst_or(inst),
            InstID::ORI => self.inst_ori(inst),
            InstID::SB => self.inst_sb(inst),
            InstID::SD => self.inst_sd(inst),
            InstID::SH => self.inst_sh(inst),
            InstID::SW => self.inst_sw(inst),
            InstID::SLL => self.inst_sll(inst),
            InstID::SLLI => self.inst_slli(inst),
            InstID::SLLIW => self.inst_slliw(inst),
            InstID::SLLW => self.inst_sllw(inst),
            InstID::SRLI => self.inst_srli(inst),
            InstID::SRAI => self.inst_srai(inst),
            InstID::SRAIW => self.inst_sraiw(inst),
            InstID::SUB => self.inst_sub(inst),
            InstID::NOP => self.inst_nop(inst),
            InstID::XORI => self.inst_xori(inst),
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
        unsafe {
            std::mem::transmute::<[u8; std::mem::size_of::<AddressType>()], AddressType>(*data)
        }
    }

    fn byte_array_to_addr_type_32b(data: &[u8; 4]) -> AddressType {
        (unsafe { std::mem::transmute::<[u8; 4], u32>(*data) }) as AddressType
    }
    fn byte_array_to_addr_type_8b(data: &[u8; 1]) -> AddressType {
        (unsafe { std::mem::transmute::<[u8; 1], u8>(*data) }) as AddressType
    }

    fn inst_auipc(&mut self, inst: &inst_type::InstType) {
        self.regs
            .write(inst.get_rd(), self.pc + inst.get_imm_utype());
    }

    fn inst_add(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1());
        let rs2_val = self.regs.read(inst.get_rs2_rtype());
        self.regs
            .write(inst.get_rd(), rs1_val.wrapping_add( rs2_val));
    }

    fn inst_addi(&mut self, inst: &inst_type::InstType) {
        self.regs.write(
            inst.get_rd(),
            self.regs
                .read(inst.get_rs1())
                .wrapping_add(RVCore::sign_extend(inst.get_imm_itype(), 12)),
        );
    }

    fn inst_addiw(&mut self, inst: &inst_type::InstType) {
        let imm = RVCore::sign_extend(inst.get_imm_itype(), 12);
        let wdata = self.regs.read(inst.get_rs1()).wrapping_add(imm);
        self.regs.write(inst.get_rd(), wdata);
    }

    fn inst_addw(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1());
        let rs2_val = self.regs.read(inst.get_rs2_rtype());
        self.regs.write(inst.get_rd(), rs1_val + rs2_val);
    }

    fn inst_andi(&mut self, inst: &inst_type::InstType) {
        let imm = RVCore::sign_extend(inst.get_imm_itype(), 12);
        self.regs.write(
            inst.get_rd(),
            self.regs.read(inst.get_rs1()) & imm,
        );
    }

    fn inst_beq(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_btype();
        let offset = (((imm >> 11) & 1) << 12)
            | (((imm >> 5) & 0x3f) << 5)
            | (((imm >> 1) & 0xf) << 1)
            | (((imm >> 0) & 1) << 11);
        let new_pc = self.pc.wrapping_add(RVCore::sign_extend(offset, 12));
        //print!(" {},{},{:x}", XRegisters::name(inst.get_rs1()),
        //        XRegisters::name(inst.get_rs2_btype()), new_pc);
        if (self.regs.read(inst.get_rs1()) as i64) == (self.regs.read(inst.get_rs2_btype()) as i64)
        {
            self.pc = new_pc;
        } else {
            self.pc += 4;
        }
    }

    fn inst_blt(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_btype();
        let offset = (((imm >> 11) & 1) << 12)
            | (((imm >> 5) & 0x3f) << 5)
            | (((imm >> 1) & 0xf) << 1)
            | (((imm >> 0) & 1) << 11);
        let new_pc = self.pc.wrapping_add(RVCore::sign_extend(offset, 12));
        //print!(" {},{},{:x}", XRegisters::name(inst.get_rs1()),
        //        XRegisters::name(inst.get_rs2_btype()), new_pc);
        if (self.regs.read(inst.get_rs1()) as i64) < (self.regs.read(inst.get_rs2_btype()) as i64) {
            self.pc = new_pc;
        } else {
            self.pc += 4;
        }
    }

    fn inst_bltu(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_btype();
        let offset = (((imm >> 11) & 1) << 12)
            | (((imm >> 5) & 0x3f) << 5)
            | (((imm >> 1) & 0xf) << 1)
            | (((imm >> 0) & 1) << 11);
        let new_pc = self.pc.wrapping_add(RVCore::sign_extend(offset, 12));
        //print!(" {},{},{:x}", XRegisters::name(inst.get_rs1()),
        //        XRegisters::name(inst.get_rs2_btype()), new_pc);
        if self.regs.read(inst.get_rs1()) < self.regs.read(inst.get_rs2_btype()) {
            self.pc = new_pc;
        } else {
            self.pc += 4;
        }
    }

    fn inst_bge(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_btype();
        let offset = (((imm >> 11) & 1) << 12)
            | (((imm >> 5) & 0x3f) << 5)
            | (((imm >> 1) & 0xf) << 1)
            | (((imm >> 0) & 1) << 11);
        let new_pc = self.pc.wrapping_add(RVCore::sign_extend(offset, 12));
        //print!(" {},{},{:x}", XRegisters::name(inst.get_rs1()),
        //        XRegisters::name(inst.get_rs2_btype()), new_pc);
        if (self.regs.read(inst.get_rs1()) as i64) >= (self.regs.read(inst.get_rs2_btype()) as i64) {
            self.pc = new_pc;
        } else {
            self.pc += 4;
        }
    }

    fn inst_bgeu(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_btype();
        let offset = (((imm >> 11) & 1) << 12)
            | (((imm >> 5) & 0x3f) << 5)
            | (((imm >> 1) & 0xf) << 1)
            | (((imm >> 0) & 1) << 11);
        let new_pc = self.pc.wrapping_add(RVCore::sign_extend(offset, 12));
        //print!(" {},{},{:x}", XRegisters::name(inst.get_rs1()),
        //        XRegisters::name(inst.get_rs2_btype()), new_pc);
        if self.regs.read(inst.get_rs1()) >= self.regs.read(inst.get_rs2_btype()) {
            self.pc = new_pc;
        } else {
            self.pc += 4;
        }
    }

    fn inst_bne(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_btype();
        let offset = (((imm >> 11) & 1) << 12)
            | (((imm >> 5) & 0x3f) << 5)
            | (((imm >> 1) & 0xf) << 1)
            | (((imm >> 0) & 1) << 11);
        let new_pc = self.pc.wrapping_add(RVCore::sign_extend(offset, 12));
        //print!(" {},{},{:x}", XRegisters::name(inst.get_rs1()),
        //        XRegisters::name(inst.get_rs2_btype()), new_pc);
        if (self.regs.read(inst.get_rs1()) as i64) != (self.regs.read(inst.get_rs2_btype()) as i64)
        {
            self.pc = new_pc;
        } else {
            self.pc += 4;
        }
    }

    fn inst_c_add(&mut self, inst: &inst_type::InstType) {
        let result = self
            .regs
            .read(inst.get_rd())
            .wrapping_add(self.regs.read(inst.get_rs2()));
        self.regs.write(inst.get_rd(), result);
    }

    fn inst_c_addi(&mut self, inst: &inst_type::InstType) {
        let imm = RVCore::sign_extend(inst.get_imm_ci(), 6);
        self.regs.write(
            inst.get_rd(),
            self.regs.read(inst.get_rd()).wrapping_add(imm),
        );
    }

    fn inst_c_addiw(&mut self, inst: &inst_type::InstType) {
        let rd_val = self.regs.read(inst.get_rd());
        let imm = RVCore::sign_extend(inst.get_imm_ci(), 6);
        let result = rd_val.wrapping_add(imm);
        self.regs.write(inst.get_rd(), result);
    }

    fn inst_c_addi16sp(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_ci();
        let val = (((imm >> 5) & 1) << 9)
            | (((imm >> 4) & 0x1) << 4)
            | (((imm >> 3) & 0x1) << 6)
            | (((imm >> 1) & 0x3) << 7)
            | (((imm >> 0) & 0x1) << 5);
        let old_sp = self.regs.read(2);
        self.regs
            .write(2, old_sp.wrapping_add(RVCore::sign_extend(val, 10)));
    }

    fn inst_c_addi4spn(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_ciw();
        let val = (((imm >> 6) & 0x3) << 4)
            | (((imm >> 2) & 0xf) << 6)
            | (((imm >> 1) & 0x1) << 2)
            | (((imm >> 0) & 0x1) << 3);
        let old_sp = self.regs.read(2);
        self.regs.write(inst.get_rd_ciw(), old_sp.wrapping_add(val));
    }

    fn inst_c_addw(&mut self, inst: &inst_type::InstType) {
        let a = self.regs.read(inst.get_rd_3b());
        let b = self.regs.read(inst.get_rs2_3b());
        self.regs.write(inst.get_rd_3b(), a.wrapping_add(b));
    }

    fn inst_c_and(&mut self, inst: &inst_type::InstType) {
        let a = self.regs.read(inst.get_rd_3b());
        let b = self.regs.read(inst.get_rs2_3b());
        self.regs.write(inst.get_rd_3b(), a & b);
    }

    fn inst_c_andi(&mut self, inst: &inst_type::InstType) {
        let a = self.regs.read(inst.get_rs1_3b());
        let imm_cb = inst.get_imm_cb();
        let imm = RVCore::sign_extend((imm_cb & 0x1f) | (((imm_cb >> 7) & 1) << 5), 6);
        self.regs.write(inst.get_rs1_3b(), a & imm);
    }

    fn inst_c_beqz(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1_3b());
        let imm = inst.get_imm_cb();
        let offset = (((imm >> 7) & 0x1) << 8)
            | (((imm >> 5) & 0x3) << 3)
            | (((imm >> 3) & 0x3) << 6)
            | (((imm >> 1) & 0x3) << 1)
            | (((imm >> 0) & 0x1) << 5);
        //print!(" {},{:x}", XRegisters::name(inst.get_rs1_3b()), self.pc + offset);
        if rs1_val == 0 {
            self.pc += offset;
        } else {
            self.pc += 2;
        }
    }

    fn inst_c_bnez(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1_3b());
        let imm = inst.get_imm_cb();
        let offset = (((imm >> 7) & 0x1) << 8)
            | (((imm >> 5) & 0x3) << 3)
            | (((imm >> 3) & 0x3) << 6)
            | (((imm >> 1) & 0x3) << 1)
            | (((imm >> 0) & 0x1) << 5);
        //print!(" {},{:x}", XRegisters::name(inst.get_rs1_3b()), self.pc + offset);
        if rs1_val != 0 {
            self.pc += offset;
        } else {
            self.pc += 2;
        }
    }

    fn inst_c_fsdsp(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_css();
        let address = self.regs.read(2) + (((imm & 0x7) << 6) | (imm & 0x38));
        let data = self.fregs.read(inst.get_rs2());
        self.write_memory(address, &data.to_le_bytes());
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

    fn inst_c_jalr(&mut self, inst: &inst_type::InstType) {
        //print!(" {}", XRegisters::name(inst.get_rs1_cr()));
        self.regs.write(1, self.pc + 2);
        self.pc = self.regs.read(inst.get_rs1_cr());
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
        //print!(" {}", XRegisters::name(inst.get_rs1_cr()));
        self.pc = self.regs.read(inst.get_rs1_cr());
    }

    fn inst_c_or(&mut self, inst: &inst_type::InstType) {
        let a = self.regs.read(inst.get_rd_3b());
        let b = self.regs.read(inst.get_rs2_3b());
        self.regs.write(inst.get_rd_3b(), a | b);
    }

    fn inst_c_sdsp(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_css();
        let address = self.regs.read(2) + (((imm & 0x7) << 6) | (imm & 0x38));
        let data = self.regs.read(inst.get_rs2()) as u64;
        self.write_memory(address, &data.to_le_bytes());
    }

    fn inst_c_slli(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_ci();
        let rd_val = self.regs.read(inst.get_rd());
        self.regs.write(inst.get_rd(), rd_val << imm);
    }

    fn inst_c_srli(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_ci();
        let rd_val = self.regs.read(inst.get_rd_3b());
        self.regs.write(inst.get_rd_3b(), rd_val >> imm);
    }

    fn inst_c_sw(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_cl();
        let offset = (((imm >> 2) & 0x7) << 3) | (((imm >> 1) & 1) << 2) | (((imm >> 0) & 1) << 6);
        let address = self.regs.read(inst.get_rs1_3b()) + offset;
        let data = self.regs.read(inst.get_rs2_3b()) as u32;
        self.write_memory(address, &data.to_le_bytes());
    }

    fn inst_c_swsp(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_css();
        let address = self.regs.read(2) + (((imm & 0x3) << 6) | (imm & 0x3c));
        let data = self.regs.read(inst.get_rs2()) as u32;
        self.write_memory(address, &data.to_le_bytes());
    }

    fn inst_c_ld(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_cl();
        let offset = (((imm >> 2) & 0x7) << 3) | (((imm >> 0) & 0x3) << 6);
        let address = self.regs.read(inst.get_rs1_3b()) + offset;
        let mut data = [0; 8];
        self.read_memory(address, &mut data);
        self.regs
            .write(inst.get_rd_cl(), RVCore::byte_array_to_addr_type(&data));
    }

    fn inst_c_ldsp(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_ci();
        let address = self.regs.read(2) + (((imm & 0x7) << 6) | (imm & 0x38));
        let mut data = [0; 8];
        self.read_memory(address, &mut data);
        self.regs
            .write(inst.get_rd(), RVCore::byte_array_to_addr_type(&data));
    }

    fn inst_c_lw(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_cl();
        let offset = (((imm >> 2) & 0x7) << 3) | (((imm >> 1) & 1) << 2) | (((imm >> 0) & 1) << 6);
        let address = self.regs.read(inst.get_rs1_3b()) + offset;
        let mut data = [0; 4];
        self.read_memory(address, &mut data);
        self.regs
            .write(inst.get_rd_cl(), RVCore::byte_array_to_addr_type_32b(&data));
    }

    fn inst_c_lwsp(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_ci();
        let address = self.regs.read(2) + (((imm & 0x3) << 6) | (imm & 0x3c));
        let mut data = [0; 4];
        self.read_memory(address, &mut data);
        self.regs
            .write(inst.get_rd(), RVCore::byte_array_to_addr_type_32b(&data));
    }

    fn inst_c_li(&mut self, inst: &inst_type::InstType) {
        self.regs
            .write(inst.get_rd(), RVCore::sign_extend(inst.get_imm_ci(), 6));
    }

    fn inst_c_lui(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_ci();
        let val = (((imm >> 5) & 1) << 17) | (((imm >> 0) & 0x1f) << 12);
        self.regs.write(inst.get_rd(), RVCore::sign_extend(val, 18));
    }

    fn inst_c_mv(&mut self, inst: &inst_type::InstType) {
        self.regs
            .write(inst.get_rd(), self.regs.read(inst.get_rs2()));
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

    fn inst_c_xor(&mut self, inst: &inst_type::InstType) {
        let a = self.regs.read(inst.get_rd_3b());
        let b = self.regs.read(inst.get_rs2_3b());
        self.regs.write(inst.get_rd_3b(), a ^ b);
    }

    fn inst_csrrs(&mut self, inst: &inst_type::InstType) {
        //let rd = inst.get_rd();
        //let rs1 = inst.get_rs1();
        //let csr = inst.get_csr();
    }

    fn inst_csrrw(&mut self, inst: &inst_type::InstType) {
        //let rd = inst.get_rd();
        //let rs1 = inst.get_rs1();
        //let csr = inst.get_csr();
    }

    fn inst_ecall(&mut self, _inst: &inst_type::InstType) {
        //panic!("ECALL: Exceptions are not supported now");
    }

    fn inst_fmv_w_x(&mut self, inst: &inst_type::InstType) {
        let rs1 = inst.get_rs1();
        let rs1_lower_val = self.regs.read(rs1) as u32;
        self.fregs.write(inst.get_rd(), f32::from_le_bytes(rs1_lower_val.to_le_bytes()).into());
    }

    fn inst_jal(&mut self, inst: &inst_type::InstType) {
        self.regs.write(inst.get_rd(), self.pc + 4);
        let imm = inst.get_imm_jtype();
        let mut offset = (((imm >> 19) & 1) << 20)
            | (((imm >> 9) & 0x3ff) << 1)
            | (((imm >> 8) & 0x1) << 11)
            | (((imm >> 0) & 0xff) << 12);
        offset = RVCore::sign_extend(offset, 21);
        self.pc = self.pc.wrapping_add(offset);
        //print!(" {},{:x}", XRegisters::name(inst.get_rd()), self.pc);
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
        //print!(" {},{:x}({})", XRegisters::name(inst.get_rd()), self.pc, XRegisters::name(inst.get_rs1()));
    }

    fn inst_lbu(&mut self, inst: &inst_type::InstType) {
        let imm = RVCore::sign_extend(inst.get_imm_itype(), 12);
        let address = self.regs.read(inst.get_rs1()).wrapping_add(imm);
        let mut data = [0; 1];
        self.read_memory(address, &mut data);
        let wdata = RVCore::byte_array_to_addr_type_8b(&data);
        self.regs.write(inst.get_rd(), wdata);
    }

    fn inst_ld(&mut self, inst: &inst_type::InstType) {
        let imm = RVCore::sign_extend(inst.get_imm_itype(), 12);
        let address = self.regs.read(inst.get_rs1()).wrapping_add(imm);
        let mut data = [0; 8];
        self.read_memory(address, &mut data);
        let wdata = RVCore::byte_array_to_addr_type(&data) as AddressType;
        self.regs.write(inst.get_rd(), wdata);
    }

    fn inst_lui(&mut self, inst: &inst_type::InstType) {
        self.regs
            .write(inst.get_rd(), RVCore::sign_extend(inst.get_imm_utype(), 32));
    }

    fn inst_lw(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_itype();
        let address = self.regs.read(inst.get_rs1()) + (imm & 0xfff);
        let mut data = [0; 4];
        self.read_memory(address, &mut data);
        self.regs.write(
            inst.get_rd(),
            RVCore::byte_array_to_addr_type_32b(&data) as AddressType,
        );
    }

    fn inst_mulw(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1());
        let rs2_val = self.regs.read(inst.get_rs2_rtype());
        self.regs.write(inst.get_rd(), rs1_val * rs2_val);
    }

    fn inst_or(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1());
        let rs2_val = self.regs.read(inst.get_rs2_rtype());
        self.regs
            .write(inst.get_rd(), rs1_val | rs2_val);
    }

    fn inst_ori(&mut self, inst: &inst_type::InstType) {
        let imm = RVCore::sign_extend(inst.get_imm_itype(), 12);
        self.regs.write(
            inst.get_rd(),
            self.regs.read(inst.get_rs1()) | imm,
        );
    }

    fn inst_sb(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_stype();
        let address = self.regs.read(inst.get_rs1()) + (imm & 0xfff);
        let data = self.regs.read(inst.get_rs2_stype()) as u8;
        self.write_memory(address, &data.to_le_bytes());
    }

    fn inst_sd(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_stype();
        let address = self.regs.read(inst.get_rs1()) + (imm & 0xfff);
        let data = self.regs.read(inst.get_rs2_stype()) as u64;
        self.write_memory(address, &data.to_le_bytes());
    }

    fn inst_sh(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_stype();
        let address = self.regs.read(inst.get_rs1()) + (imm & 0xfff);
        let data = self.regs.read(inst.get_rs2_stype()) as u16;
        self.write_memory(address, &data.to_le_bytes());
    }

    fn inst_sw(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_stype();
        let address = self.regs.read(inst.get_rs1()) + (imm & 0xfff);
        let data = self.regs.read(inst.get_rs2_stype()) as u32;
        self.write_memory(address, &data.to_le_bytes());
    }

    fn inst_sll(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1());
        let rs2_val = self.regs.read(inst.get_rs2_rtype());
        self.regs
            .write(inst.get_rd(), rs1_val << rs2_val);
    }

    fn inst_slli(&mut self, inst: &inst_type::InstType) {
        let shamt = inst.get_shamt_itype();
        let rs1_val = self.regs.read(inst.get_rs1());
        self.regs.write(inst.get_rd(), rs1_val << shamt);
    }

    fn inst_slliw(&mut self, inst: &inst_type::InstType) {
        let shamt = inst.get_shamt_itype();
        let rs1_val = self.regs.read(inst.get_rs1());
        self.regs.write(inst.get_rd(), rs1_val << shamt);
    }

    fn inst_sllw(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1());
        let rs2_val = self.regs.read(inst.get_rs2_rtype());
        self.regs.write(inst.get_rd(), rs1_val << rs2_val);
    }

    fn inst_srli(&mut self, inst: &inst_type::InstType) {
        let shamt = inst.get_shamt_itype();
        let rs1_val = self.regs.read(inst.get_rs1());
        self.regs.write(inst.get_rd(), rs1_val >> shamt);
    }

    fn inst_srai(&mut self, inst: &inst_type::InstType) {
        let shamt = inst.get_shamt_itype();
        let rs1_val = self.regs.read(inst.get_rs1());
        self.regs
            .write(inst.get_rd(), ((rs1_val as i64) >> shamt) as AddressType);
    }

    fn inst_sraiw(&mut self, inst: &inst_type::InstType) {
        let shamt = inst.get_shamt_itype() & 0x1f;
        let rs1_val = self.regs.read(inst.get_rs1());
        self.regs
            .write(inst.get_rd(), ((rs1_val as i64) >> shamt) as AddressType);
    }

    fn inst_sub(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1());
        let rs2_val = self.regs.read(inst.get_rs2_rtype());
        self.regs
            .write(inst.get_rd(), rs1_val.wrapping_sub(rs2_val));
    }

    fn inst_nop(&mut self, _inst: &inst_type::InstType) {}

    fn inst_xori(&mut self, inst: &inst_type::InstType) {
        let imm = RVCore::sign_extend(inst.get_imm_itype(), 12);
        self.regs.write(
            inst.get_rd(),
            self.regs.read(inst.get_rs1()) ^ imm,
        );
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
        core.inst_bgeu(&inst_bgeu_code(2, 3, 0x7fe));
        assert_eq!(0x7fe, core.pc);

        core.pc = 0;
        core.regs.write(2, 0x1230);
        core.regs.write(3, 0x1234);
        core.inst_bgeu(&inst_bgeu_code(2, 3, 0x7fe));
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
    fn test_inst_c_beqz() {
        let mut core: RVCore = RVCore::new();
        core.regs.write(10, 0);
        core.inst_c_beqz(&inst_c_beqz_code(10, 0xfe));
        assert_eq!(0xfe, core.pc);

        core.pc = 0;
        core.regs.write(10, 1);
        core.inst_c_beqz(&inst_c_beqz_code(10, 0xfe));
        assert_eq!(2, core.pc);
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
    fn test_inst_c_lw() {
        let mut core: RVCore = RVCore::new();
        let mut mem_stub: MemoryStub = Default::default();
        core.bind_mem(&mut mem_stub);

        core.regs.write(9, 0x8888); // Address
        core.inst_c_lw(&inst_c_lw_code(8, 9, 0x4));
        assert_eq!(MemoryOperation::READ, mem_stub.buffer.op);
        assert_eq!(0x888c, mem_stub.buffer.addr);
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
        assert_eq!(
            [0x78, 0x56, 0x34, 0x12, 0, 0, 0, 0].to_vec(),
            mem_stub.buffer.data
        );
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
    fn test_inst_ld() {
        let mut core: RVCore = RVCore::new();
        let mut mem_stub: MemoryStub = Default::default();
        core.bind_mem(&mut mem_stub);

        core.regs.write(1, 0x8888); // Address
        core.inst_ld(&inst_ld_code(2, 1, 0xffe));
        assert_eq!(MemoryOperation::READ, mem_stub.buffer.op);
        assert_eq!(0x8888 - 2, mem_stub.buffer.addr);
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
    fn test_inst_sd() {
        let mut core: RVCore = RVCore::new();
        let mut mem_stub: MemoryStub = Default::default();
        core.bind_mem(&mut mem_stub);

        core.regs.write(1, 0xffffff78); // Data
        core.regs.write(2, 0x8888); // Address
        core.inst_sd(&inst_sd_code(1, 2, 0xff));
        assert_eq!(MemoryOperation::WRITE, mem_stub.buffer.op);
        assert_eq!(0x8888 + 0xff, mem_stub.buffer.addr);
        assert_eq!(
            [0x78, 0xff, 0xff, 0xff, 0, 0, 0, 0].to_vec(),
            mem_stub.buffer.data
        );
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
