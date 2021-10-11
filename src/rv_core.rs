mod csregs;
mod fregs;
mod inst_decoder;
mod inst_info;
mod inst_type;
mod xregs;
use crate::memory_interface::{MemoryInterface, MemoryOperation, Payload};
use crate::rv_core::inst_info::InstID;
use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;

use softfloat_wrapper::{ExceptionFlags, Float, RoundingMode, F32, F64};

type AddressType = u64;

enum PrivilegeMode {
    U,
    S,
    M,
}

pub struct RVCore {
    pub pc: AddressType,
    pub regs: xregs::XRegisters,
    pub fregs: fregs::FRegisters,
    pub csregs: csregs::CSRegisters,
    id_instance: inst_decoder::InstDecoder,
    mem_if: Option<Rc<RefCell<dyn MemoryInterface>>>,
    mode: PrivilegeMode,
}

impl RVCore {
    pub fn new() -> RVCore {
        RVCore {
            pc: 0,
            regs: xregs::XRegisters::new(),
            fregs: fregs::FRegisters::new(),
            csregs: csregs::CSRegisters::new(),
            id_instance: inst_decoder::InstDecoder::new(),
            mem_if: None,
            mode: PrivilegeMode::M,
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
            //print!(", {}={:08x}", self.regs.name(i), self.regs.read(i));
            print!(", r{}={:08x}", i, self.regs.read(i));
        }
        println!("");

        self.execute(&inst);
        self.pc += inst.len;

        self.csregs
            .write(csregs::MCYCLE, self.csregs.read(csregs::MCYCLE) + 1);
        self.csregs
            .write(csregs::MINSTRET, self.csregs.read(csregs::MINSTRET) + 1);
    }

    fn execute(&mut self, inst: &inst_type::InstType) {
        match inst.id {
            InstID::AUIPC => self.inst_auipc(inst),
            InstID::ADD => self.inst_add(inst),
            InstID::ADDI => self.inst_addi(inst),
            InstID::ADDIW => self.inst_addiw(inst),
            InstID::ADDW => self.inst_addw(inst),
            InstID::AND => self.inst_and(inst),
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
            InstID::C_JALR => self.inst_c_jalr(inst),
            InstID::C_JR => self.inst_c_jr(inst),
            InstID::C_OR => self.inst_c_or(inst),
            InstID::C_SDSP => self.inst_c_sdsp(inst),
            InstID::C_SLLI => self.inst_c_slli(inst),
            InstID::C_SRAI => self.inst_c_srai(inst),
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
            InstID::C_SUBW => self.inst_c_subw(inst),
            InstID::C_SD => self.inst_c_sd(inst),
            InstID::C_XOR => self.inst_c_xor(inst),
            InstID::CSRRCI => self.inst_csrrci(inst),
            InstID::CSRRS => self.inst_csrrs(inst),
            InstID::CSRRW => self.inst_csrrw(inst),
            InstID::CSRRWI => self.inst_csrrwi(inst),
            InstID::DIV => self.inst_div(inst),
            InstID::DIVU => self.inst_divu(inst),
            InstID::DIVUW => self.inst_divuw(inst),
            InstID::DIVW => self.inst_divw(inst),
            InstID::ECALL => self.inst_ecall(inst),
            InstID::FADD_D => self.inst_fadd_d(inst),
            InstID::FADD_S => self.inst_fadd_s(inst),
            InstID::FCLASS_S => self.inst_fclass_s(inst),
            InstID::FCVT_L_S => self.inst_fcvt_l_s(inst),
            InstID::FCVT_LU_S => self.inst_fcvt_lu_s(inst),
            InstID::FCVT_S_L => self.inst_fcvt_s_l(inst),
            InstID::FCVT_S_LU => self.inst_fcvt_s_lu(inst),
            InstID::FCVT_S_W => self.inst_fcvt_s_w(inst),
            InstID::FCVT_S_WU => self.inst_fcvt_s_wu(inst),
            InstID::FCVT_W_S => self.inst_fcvt_w_s(inst),
            InstID::FCVT_WU_S => self.inst_fcvt_wu_s(inst),
            InstID::FDIV_S => self.inst_fdiv_s(inst),
            InstID::FMADD_S => self.inst_fmadd_s(inst),
            InstID::FMAX_S => self.inst_fmax_s(inst),
            InstID::FMIN_S => self.inst_fmin_s(inst),
            InstID::FMSUB_S => self.inst_fmsub_s(inst),
            InstID::FNMADD_S => self.inst_fnmadd_s(inst),
            InstID::FNMSUB_S => self.inst_fnmsub_s(inst),
            InstID::FSQRT_S => self.inst_fsqrt_s(inst),
            InstID::FSGNJ_S => self.inst_fsgnj_s(inst),
            InstID::FSGNJN_S => self.inst_fsgnjn_s(inst),
            InstID::FSGNJX_S => self.inst_fsgnjx_s(inst),
            InstID::FENCE => self.inst_fence(inst),
            InstID::FEQ_S => self.inst_feq_s(inst),
            InstID::FLD => self.inst_fld(inst),
            InstID::FLE_S => self.inst_fle_s(inst),
            InstID::FLT_S => self.inst_flt_s(inst),
            InstID::FLW => self.inst_flw(inst),
            InstID::FSD => self.inst_fsd(inst),
            InstID::FSW => self.inst_fsw(inst),
            InstID::FMUL_D => self.inst_fmul_d(inst),
            InstID::FMUL_S => self.inst_fmul_s(inst),
            InstID::FMV_W_X => self.inst_fmv_w_x(inst),
            InstID::FMV_X_D => self.inst_fmv_x_d(inst),
            InstID::FMV_X_W => self.inst_fmv_x_w(inst),
            InstID::FSUB_D => self.inst_fsub_d(inst),
            InstID::FSUB_S => self.inst_fsub_s(inst),
            InstID::JAL => self.inst_jal(inst),
            InstID::JALR => self.inst_jalr(inst),
            InstID::LB => self.inst_lb(inst),
            InstID::LBU => self.inst_lbu(inst),
            InstID::LD => self.inst_ld(inst),
            InstID::LH => self.inst_lh(inst),
            InstID::LHU => self.inst_lhu(inst),
            InstID::LUI => self.inst_lui(inst),
            InstID::LW => self.inst_lw(inst),
            InstID::LWU => self.inst_lwu(inst),
            InstID::MUL => self.inst_mul(inst),
            InstID::MULH => self.inst_mulh(inst),
            InstID::MULHSU => self.inst_mulhsu(inst),
            InstID::MULHU => self.inst_mulhu(inst),
            InstID::MULW => self.inst_mulw(inst),
            InstID::MRET => self.inst_mret(inst),
            InstID::OR => self.inst_or(inst),
            InstID::ORI => self.inst_ori(inst),
            InstID::REM => self.inst_rem(inst),
            InstID::REMU => self.inst_remu(inst),
            InstID::REMUW => self.inst_remuw(inst),
            InstID::REMW => self.inst_remw(inst),
            InstID::SB => self.inst_sb(inst),
            InstID::SD => self.inst_sd(inst),
            InstID::SH => self.inst_sh(inst),
            InstID::SW => self.inst_sw(inst),
            InstID::SLL => self.inst_sll(inst),
            InstID::SLLI => self.inst_slli(inst),
            InstID::SLLIW => self.inst_slliw(inst),
            InstID::SLLW => self.inst_sllw(inst),
            InstID::SLT => self.inst_slt(inst),
            InstID::SLTI => self.inst_slti(inst),
            InstID::SLTIU => self.inst_sltiu(inst),
            InstID::SLTU => self.inst_sltu(inst),
            InstID::SRL => self.inst_srl(inst),
            InstID::SRLI => self.inst_srli(inst),
            InstID::SRLIW => self.inst_srliw(inst),
            InstID::SRLW => self.inst_srlw(inst),
            InstID::SRA => self.inst_sra(inst),
            InstID::SRAI => self.inst_srai(inst),
            InstID::SRAIW => self.inst_sraiw(inst),
            InstID::SRAW => self.inst_sraw(inst),
            InstID::SUB => self.inst_sub(inst),
            InstID::SUBW => self.inst_subw(inst),
            InstID::NOP => self.inst_nop(inst),
            InstID::XOR => self.inst_xor(inst),
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
        self.mem_if
            .as_mut()
            .unwrap()
            .borrow_mut()
            .access_memory(&mut payload);
    }

    fn read_memory(&mut self, address: AddressType, data: &mut [u8]) {
        let mut payload = Payload {
            addr: address,
            data: data.to_vec(),
            op: MemoryOperation::READ,
        };
        self.mem_if
            .as_mut()
            .unwrap()
            .borrow_mut()
            .access_memory(&mut payload);

        for i in 0..data.len() {
            data[i] = payload.data[i];
        }
    }

    pub fn bind_mem(&mut self, mem_if: Rc<RefCell<dyn MemoryInterface>>) {
        self.mem_if = Some(mem_if);
    }

    fn byte_array_to_addr_type(data: &[u8]) -> AddressType {
        match data.len() {
            8 => u64::from_le_bytes(data.try_into().unwrap()) as AddressType,
            4 => u32::from_le_bytes(data.try_into().unwrap()) as AddressType,
            2 => u16::from_le_bytes(data.try_into().unwrap()) as AddressType,
            1 => u8::from_le_bytes(data.try_into().unwrap()) as AddressType,
            _ => panic!("bad data length"),
        }
    }

    fn update_fflags(&mut self, flags: &ExceptionFlags) {
        let mut val = 0;
        if flags.is_inexact() {
            val |= 1;
        }
        if flags.is_underflow() {
            val |= 2;
        }
        if flags.is_invalid() {
            val |= 0x10;
        }

        self.csregs.write(csregs::FFLAGS, val);
    }

    fn inst_auipc(&mut self, inst: &inst_type::InstType) {
        let result = (self.pc + inst.get_imm_utype()) as u32;
        self.regs.write(inst.get_rd(), result as AddressType);
    }

    fn inst_add(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1());
        let rs2_val = self.regs.read(inst.get_rs2_rtype());
        self.regs
            .write(inst.get_rd(), rs1_val.wrapping_add(rs2_val));
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
        let mut wdata = self.regs.read(inst.get_rs1()).wrapping_add(imm);
        wdata = wdata as u32 as AddressType;
        wdata = RVCore::sign_extend(wdata, 32);
        self.regs.write(inst.get_rd(), wdata);
    }

    fn inst_addw(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1());
        let rs2_val = self.regs.read(inst.get_rs2_rtype());
        let result = RVCore::sign_extend(rs1_val.wrapping_add(rs2_val) as u32 as u64, 32);
        self.regs.write(inst.get_rd(), result);
    }

    fn inst_and(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1());
        let rs2_val = self.regs.read(inst.get_rs2_rtype());
        self.regs.write(inst.get_rd(), rs1_val & rs2_val);
    }

    fn inst_andi(&mut self, inst: &inst_type::InstType) {
        let imm = RVCore::sign_extend(inst.get_imm_itype(), 12);
        self.regs
            .write(inst.get_rd(), self.regs.read(inst.get_rs1()) & imm);
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
            self.pc = new_pc - inst.len;
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
            self.pc = new_pc - inst.len;
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
            self.pc = new_pc - inst.len;
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
        if (self.regs.read(inst.get_rs1()) as i64) >= (self.regs.read(inst.get_rs2_btype()) as i64)
        {
            self.pc = new_pc - inst.len;
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
            self.pc = new_pc - inst.len;
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
            self.pc = new_pc - inst.len;
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
        let result = rd_val.wrapping_add(imm) as u32 as u64;
        self.regs
            .write(inst.get_rd(), RVCore::sign_extend(result, 32));
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
        let result = a.wrapping_add(b) as u32 as u64;
        self.regs
            .write(inst.get_rd_3b(), RVCore::sign_extend(result, 32));
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
        let mut offset = (((imm >> 7) & 0x1) << 8)
            | (((imm >> 5) & 0x3) << 3)
            | (((imm >> 3) & 0x3) << 6)
            | (((imm >> 1) & 0x3) << 1)
            | (((imm >> 0) & 0x1) << 5);
        offset = RVCore::sign_extend(offset, 9);
        //print!(" {},{:x}", XRegisters::name(inst.get_rs1_3b()), self.pc + offset);
        if rs1_val == 0 {
            self.pc = self.pc.wrapping_add(offset) - inst.len;
        }
    }

    fn inst_c_bnez(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1_3b());
        let imm = inst.get_imm_cb();
        let mut offset = (((imm >> 7) & 0x1) << 8)
            | (((imm >> 5) & 0x3) << 3)
            | (((imm >> 3) & 0x3) << 6)
            | (((imm >> 1) & 0x3) << 1)
            | (((imm >> 0) & 0x1) << 5);
        offset = RVCore::sign_extend(offset, 9);
        //print!(" {},{:x}", XRegisters::name(inst.get_rs1_3b()), self.pc + offset);
        if rs1_val != 0 {
            self.pc = self.pc.wrapping_add(offset) - inst.len;
        }
    }

    fn inst_c_fsdsp(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_css();
        let address = self.regs.read(2) + (((imm & 0x7) << 6) | (imm & 0x38));
        let data = self.fregs.read(inst.get_rs2());
        self.write_memory(address, &(data.to_bits().to_le_bytes()));
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
        self.pc = self.pc.wrapping_add(offset_with_sign) - inst.len;
    }

    fn inst_c_jalr(&mut self, inst: &inst_type::InstType) {
        //print!(" {}", XRegisters::name(inst.get_rs1_cr()));
        self.regs.write(1, self.pc + 2);
        self.pc = self.regs.read(inst.get_rs1_cr()) - inst.len;
    }

    /*fn inst_c_jal(&mut self, inst: &inst_type::InstType) {
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
        self.pc += offset - inst.len;
    }*/

    fn inst_c_jr(&mut self, inst: &inst_type::InstType) {
        //print!(" {}", XRegisters::name(inst.get_rs1_cr()));
        self.pc = self.regs.read(inst.get_rs1_cr()) - inst.len;
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

    fn inst_c_srai(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_ci();
        let rd_val = self.regs.read(inst.get_rd_3b()) as i64;
        self.regs
            .write(inst.get_rd_3b(), (rd_val >> imm) as AddressType);
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

        let result = RVCore::sign_extend(RVCore::byte_array_to_addr_type(&data), 32);
        self.regs.write(inst.get_rd_cl(), result);
    }

    fn inst_c_lwsp(&mut self, inst: &inst_type::InstType) {
        let imm = inst.get_imm_ci();
        let address = self.regs.read(2) + (((imm & 0x3) << 6) | (imm & 0x3c));
        let mut data = [0; 4];
        self.read_memory(address, &mut data);

        let result = RVCore::sign_extend(RVCore::byte_array_to_addr_type(&data), 32);
        self.regs.write(inst.get_rd(), result);
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

    fn inst_c_subw(&mut self, inst: &inst_type::InstType) {
        let a = self.regs.read(inst.get_rd_3b());
        let b = self.regs.read(inst.get_rs2_3b());
        let result = a.wrapping_sub(b) as u32 as u64;
        self.regs
            .write(inst.get_rd_3b(), RVCore::sign_extend(result, 32));
    }

    fn inst_c_xor(&mut self, inst: &inst_type::InstType) {
        let a = self.regs.read(inst.get_rd_3b());
        let b = self.regs.read(inst.get_rs2_3b());
        self.regs.write(inst.get_rd_3b(), a ^ b);
    }

    fn inst_csrrci(&mut self, inst: &inst_type::InstType) {
        let rd = inst.get_rd();
        let imm = inst.get_rs1() as AddressType;
        let csr = inst.get_csr();
        self.regs.write(rd, self.csregs.read(csr));
        self.csregs.write(csr, !imm & self.csregs.read(csr));
        //println!("JC_DEBUG: csrrs: writing csr {}, val {:#x}, rs1={:#x}", csr
        //            , self.regs.read(rs1) | self.csregs.read(csr), self.regs.read(rs1));
    }

    fn inst_csrrs(&mut self, inst: &inst_type::InstType) {
        let rd = inst.get_rd();
        let rs1 = inst.get_rs1();
        let csr = inst.get_csr();
        self.regs.write(rd, self.csregs.read(csr));
        self.csregs
            .write(csr, self.regs.read(rs1) | self.csregs.read(csr));
        //println!("JC_DEBUG: csrrs: writing csr {}, val {:#x}, rs1={:#x}", csr
        //            , self.regs.read(rs1) | self.csregs.read(csr), self.regs.read(rs1));
    }

    fn inst_csrrw(&mut self, inst: &inst_type::InstType) {
        let rd = inst.get_rd();
        let rs1 = inst.get_rs1();
        let csr = inst.get_csr();
        self.regs.write(rd, self.csregs.read(csr));
        self.csregs.write(csr, self.regs.read(rs1));
    }

    fn inst_csrrwi(&mut self, inst: &inst_type::InstType) {
        let rd = inst.get_rd();
        let imm = inst.get_rs1() as AddressType;
        let csr = inst.get_csr();

        // Prevent csr read when rd == 0
        if rd != 0 {
            self.regs.write(rd, self.csregs.read(csr));
        }

        //println!("JC_DEBUG: csrrwi: read csr {}, val = {:#x}, write val={:#x}"
        //            , csr, self.csregs.read(csr), imm);
        self.csregs.write(csr, imm);
    }

    fn inst_div(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1()) as i64;
        let rs2_val = self.regs.read(inst.get_rs2_rtype()) as i64;
        if rs2_val == 0 {
            self.regs.write(inst.get_rd(), AddressType::MAX);
        } else {
            self.regs
                .write(inst.get_rd(), rs1_val.wrapping_div(rs2_val) as AddressType);
        }
    }

    fn inst_divu(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1());
        let rs2_val = self.regs.read(inst.get_rs2_rtype());
        if rs2_val == 0 {
            self.regs.write(inst.get_rd(), AddressType::MAX);
        } else {
            self.regs
                .write(inst.get_rd(), rs1_val.wrapping_div(rs2_val));
        }
    }

    fn inst_divuw(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1()) as u32;
        let rs2_val = self.regs.read(inst.get_rs2_rtype()) as u32;
        if rs2_val == 0 {
            self.regs.write(inst.get_rd(), AddressType::MAX);
        } else {
            let result = rs1_val.wrapping_div(rs2_val) as AddressType;
            self.regs
                .write(inst.get_rd(), RVCore::sign_extend(result, 32));
        }
    }

    fn inst_divw(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1()) as i32;
        let rs2_val = self.regs.read(inst.get_rs2_rtype()) as i32;
        if rs2_val == 0 {
            self.regs.write(inst.get_rd(), AddressType::MAX);
        } else {
            let result = rs1_val.wrapping_div(rs2_val) as i64 as u64;
            self.regs.write(inst.get_rd(), result);
        }
    }

    fn inst_ecall(&mut self, _inst: &inst_type::InstType) {
        match self.mode {
            PrivilegeMode::U => {
                self.mode = PrivilegeMode::S;
                self.csregs.write(csregs::MCAUSE, csregs::EXC_ECALL_FROM_U);
            }
            PrivilegeMode::S => {
                self.mode = PrivilegeMode::M;
                self.csregs.write(csregs::MCAUSE, csregs::EXC_ECALL_FROM_S);
            }
            PrivilegeMode::M => {
                self.mode = PrivilegeMode::M;
                self.csregs.write(csregs::MCAUSE, csregs::EXC_ECALL_FROM_M);
            }
        }

        self.pc = self.csregs.read(csregs::MTVEC);
        self.pc -= 4;
        //panic!("ECALL: Exceptions are not supported now");
    }

    fn inst_fadd_d(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self
            .fregs
            .read(inst.get_rs1());
        let rs2_val = self
            .fregs
            .read(inst.get_rs2_stype());

        let mut flag = ExceptionFlags::default();
        flag.set();
        let result = rs1_val.add(rs2_val, RoundingMode::TiesToEven);
        flag.get();
        self.update_fflags(&flag);

        self.fregs.write(inst.get_rd(), result);
    }

    fn inst_fadd_s(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self
            .fregs
            .read(inst.get_rs1())
            .to_f32(RoundingMode::TiesToEven);
        let rs2_val = self
            .fregs
            .read(inst.get_rs2_stype())
            .to_f32(RoundingMode::TiesToEven);

        let mut flag = ExceptionFlags::default();
        flag.set();
        let result = rs1_val.add(rs2_val, RoundingMode::TiesToEven);
        flag.get();
        self.update_fflags(&flag);

        self.fregs
            .write(inst.get_rd(), result.to_f64(RoundingMode::TiesToEven));
    }

    fn inst_fclass_s(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self
            .fregs
            .read(inst.get_rs1())
            .to_f32(RoundingMode::TiesToEven);
        let f64_val = self.fregs.read(inst.get_rs1());
        if rs1_val.is_negative_infinity() {
            self.regs.write(inst.get_rd(), 1 << 0);
        } else if rs1_val.is_negative_normal() {
            self.regs.write(inst.get_rd(), 1 << 1);
        } else if rs1_val.is_negative_subnormal() {
            self.regs.write(inst.get_rd(), 1 << 2);
        } else if rs1_val.is_negative_zero() {
            self.regs.write(inst.get_rd(), 1 << 3);
        } else if rs1_val.is_positive_zero() {
            self.regs.write(inst.get_rd(), 1 << 4);
        } else if rs1_val.is_positive_subnormal() {
            self.regs.write(inst.get_rd(), 1 << 5);
        } else if rs1_val.is_positive_normal() {
            self.regs.write(inst.get_rd(), 1 << 6);
        } else if rs1_val.is_positive_infinity() {
            self.regs.write(inst.get_rd(), 1 << 7);
        } else if f64_val.is_signaling_nan() {
            self.regs.write(inst.get_rd(), 1 << 8);
        } else if rs1_val.is_nan() {
            self.regs.write(inst.get_rd(), 1 << 9);
        }
    }

    fn inst_fcvt_l_s(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.fregs.read(inst.get_rs1());
        let mut flag = ExceptionFlags::default();
        flag.set();
        let result = rs1_val.to_i64(RoundingMode::TowardZero, true);
        flag.get();
        self.update_fflags(&flag);

        if rs1_val.is_nan() || rs1_val.is_positive_infinity() {
            self.regs.write(inst.get_rd(), i64::MAX as AddressType);
        } else {
            self.regs.write(inst.get_rd(), result as AddressType);
        }
    }

    fn inst_fcvt_lu_s(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.fregs.read(inst.get_rs1());
        let mut flag = ExceptionFlags::default();
        flag.set();
        let result = rs1_val.to_u64(RoundingMode::TowardZero, true);
        flag.get();
        self.update_fflags(&flag);

        if rs1_val.is_nan() {
            self.regs.write(inst.get_rd(), AddressType::MAX);
        } else if rs1_val.is_negative() {
            self.regs.write(inst.get_rd(), 0);
        } else {
            self.regs.write(inst.get_rd(), result as AddressType);
        }
    }

    fn inst_fcvt_s_l(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1());
        let result = Float::from_i64(rs1_val as i64, RoundingMode::TiesToEven);
        self.fregs.write(inst.get_rd(), result);
    }

    fn inst_fcvt_s_lu(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1());
        let result = Float::from_u64(rs1_val, RoundingMode::TiesToEven);
        self.fregs.write(inst.get_rd(), result);
    }

    fn inst_fcvt_s_w(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1()) & 0xffffffff;
        let result = Float::from_i32(rs1_val as i32, RoundingMode::TiesToEven);
        self.fregs.write(inst.get_rd(), result);
    }

    fn inst_fcvt_s_wu(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1()) & 0xffffffff;
        let result = Float::from_u32(rs1_val as u32, RoundingMode::TiesToEven);
        self.fregs.write(inst.get_rd(), result);
    }

    fn inst_fcvt_w_s(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.fregs.read(inst.get_rs1());

        let mut flag = ExceptionFlags::default();
        flag.set();
        let result = rs1_val.to_i32(RoundingMode::TowardZero, true);
        flag.get();
        self.update_fflags(&flag);

        if (result.is_negative() && rs1_val.is_positive())
            || (rs1_val.is_nan() && rs1_val.is_negative())
        {
            self.regs.write(inst.get_rd(), i32::MAX as AddressType);
        } else {
            self.regs.write(inst.get_rd(), result as AddressType);
        }
    }

    fn inst_fcvt_wu_s(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.fregs.read(inst.get_rs1());

        let mut flag = ExceptionFlags::default();
        flag.set();
        let result = rs1_val.to_u32(RoundingMode::TowardZero, true);
        flag.get();
        self.update_fflags(&flag);

        if rs1_val.is_nan() {
            self.regs.write(inst.get_rd(), AddressType::MAX);
        } else if rs1_val.is_negative() {
            self.regs.write(inst.get_rd(), 0 as AddressType);
        } else {
            self.regs.write(
                inst.get_rd(),
                RVCore::sign_extend(result as AddressType, 32),
            );
        }
    }

    fn inst_fdiv_s(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.fregs.read(inst.get_rs1());
        let rs2_val = self.fregs.read(inst.get_rs2_stype());

        let mut flag = ExceptionFlags::default();
        flag.set();
        let result = rs1_val.div(rs2_val, RoundingMode::TiesToEven);
        flag.get();
        self.update_fflags(&flag);

        self.fregs.write(inst.get_rd(), result);
    }

    fn inst_fmadd_s(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self
            .fregs
            .read(inst.get_rs1())
            .to_f32(RoundingMode::TiesToEven);
        let rs2_val = self
            .fregs
            .read(inst.get_rs2_stype())
            .to_f32(RoundingMode::TiesToEven);
        let rs3_val = self
            .fregs
            .read(inst.get_rs3())
            .to_f32(RoundingMode::TiesToEven);

        let mut flag = ExceptionFlags::default();
        flag.set();
        let mul_result = rs1_val.mul(rs2_val, RoundingMode::TiesToEven);
        let result = mul_result.add(rs3_val, RoundingMode::TiesToEven);
        flag.get();
        self.update_fflags(&flag);

        self.fregs
            .write(inst.get_rd(), result.to_f64(RoundingMode::TiesToEven));
    }

    fn inst_fmax_s(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self
            .fregs
            .read(inst.get_rs1())
            .to_f32(RoundingMode::TiesToEven);
        let rs1_val_f64 = self.fregs.read(inst.get_rs1());
        let rs2_val = self
            .fregs
            .read(inst.get_rs2_stype())
            .to_f32(RoundingMode::TiesToEven);

        if rs1_val.is_nan() {
            if rs2_val.is_nan() {
                self.fregs.write(inst.get_rd(), F64::quiet_nan());
            } else {
                self.fregs
                    .write(inst.get_rd(), rs2_val.to_f64(RoundingMode::TiesToEven));
                if rs1_val_f64.is_signaling_nan() {
                    self.csregs.write(csregs::FFLAGS, 0x10);
                }
            }
        } else if rs2_val.is_nan() {
            self.fregs
                .write(inst.get_rd(), rs1_val.to_f64(RoundingMode::TiesToEven));
        } else {
            if (rs1_val.is_positive_zero() && rs2_val.is_negative_zero())
                || (rs1_val.is_negative_zero() && rs2_val.is_positive_zero())
            {
                self.fregs.write(inst.get_rd(), F64::positive_zero());
            } else if rs1_val.lt(rs2_val) {
                self.fregs
                    .write(inst.get_rd(), rs2_val.to_f64(RoundingMode::TiesToEven));
            } else {
                self.fregs
                    .write(inst.get_rd(), rs1_val.to_f64(RoundingMode::TiesToEven));
            }
        }
    }

    fn inst_fmin_s(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self
            .fregs
            .read(inst.get_rs1())
            .to_f32(RoundingMode::TiesToEven);
        let rs2_val = self
            .fregs
            .read(inst.get_rs2_stype())
            .to_f32(RoundingMode::TiesToEven);

        if rs1_val.is_nan() {
            if rs2_val.is_nan() {
                self.fregs.write(inst.get_rd(), F64::quiet_nan());
            } else {
                self.fregs
                    .write(inst.get_rd(), rs2_val.to_f64(RoundingMode::TiesToEven));
            }
        } else if rs2_val.is_nan() {
            self.fregs
                .write(inst.get_rd(), rs1_val.to_f64(RoundingMode::TiesToEven));
        } else {
            if (rs1_val.is_positive_zero() && rs2_val.is_negative_zero())
                || (rs1_val.is_negative_zero() && rs2_val.is_positive_zero())
            {
                self.fregs.write(inst.get_rd(), F64::negative_zero());
            } else if rs1_val.lt(rs2_val) {
                self.fregs
                    .write(inst.get_rd(), rs1_val.to_f64(RoundingMode::TiesToEven));
            } else {
                self.fregs
                    .write(inst.get_rd(), rs2_val.to_f64(RoundingMode::TiesToEven));
            }
        }
    }

    fn inst_fmsub_s(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self
            .fregs
            .read(inst.get_rs1())
            .to_f32(RoundingMode::TiesToEven);
        let rs2_val = self
            .fregs
            .read(inst.get_rs2_stype())
            .to_f32(RoundingMode::TiesToEven);
        let rs3_val = self
            .fregs
            .read(inst.get_rs3())
            .to_f32(RoundingMode::TiesToEven);

        let mut flag = ExceptionFlags::default();
        flag.set();
        let mul_result = rs1_val.mul(rs2_val, RoundingMode::TiesToEven);
        let result = mul_result.sub(rs3_val, RoundingMode::TiesToEven);
        flag.get();
        self.update_fflags(&flag);

        self.fregs
            .write(inst.get_rd(), result.to_f64(RoundingMode::TiesToEven));
    }

    fn inst_fnmadd_s(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self
            .fregs
            .read(inst.get_rs1())
            .to_f32(RoundingMode::TiesToEven);
        let rs2_val = self
            .fregs
            .read(inst.get_rs2_stype())
            .to_f32(RoundingMode::TiesToEven);
        let rs3_val = self
            .fregs
            .read(inst.get_rs3())
            .to_f32(RoundingMode::TiesToEven);

        let mut flag = ExceptionFlags::default();
        flag.set();
        let mul_result = rs1_val.mul(rs2_val, RoundingMode::TiesToEven).neg();
        let result = mul_result.sub(rs3_val, RoundingMode::TiesToEven);
        flag.get();
        self.update_fflags(&flag);

        self.fregs
            .write(inst.get_rd(), result.to_f64(RoundingMode::TiesToEven));
    }

    fn inst_fnmsub_s(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self
            .fregs
            .read(inst.get_rs1())
            .to_f32(RoundingMode::TiesToEven);
        let rs2_val = self
            .fregs
            .read(inst.get_rs2_stype())
            .to_f32(RoundingMode::TiesToEven);
        let rs3_val = self
            .fregs
            .read(inst.get_rs3())
            .to_f32(RoundingMode::TiesToEven);

        let mut flag = ExceptionFlags::default();
        flag.set();
        let mul_result = rs1_val.mul(rs2_val, RoundingMode::TiesToEven).neg();
        let result = mul_result.add(rs3_val, RoundingMode::TiesToEven);
        flag.get();
        self.update_fflags(&flag);

        self.fregs
            .write(inst.get_rd(), result.to_f64(RoundingMode::TiesToEven));
    }

    fn inst_fsqrt_s(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.fregs.read(inst.get_rs1());

        let mut flag = ExceptionFlags::default();
        flag.set();
        let result = rs1_val.sqrt(RoundingMode::TiesToEven);
        flag.get();
        self.update_fflags(&flag);

        if rs1_val.is_negative() {
            self.fregs.write(inst.get_rd(), F64::quiet_nan());
        } else {
            self.fregs.write(inst.get_rd(), result);
        }
    }

    fn inst_fsgnj_s(&mut self, inst: &inst_type::InstType) {
        let mut rs1_val = self.fregs.read(inst.get_rs1());
        let rs2_val = self.fregs.read(inst.get_rs2_stype());
        rs1_val.set_sign(rs2_val.sign());
        self.fregs.write(inst.get_rd(), rs1_val);
    }

    fn inst_fsgnjn_s(&mut self, inst: &inst_type::InstType) {
        let mut rs1_val = self.fregs.read(inst.get_rs1());
        let rs2_val = self.fregs.read(inst.get_rs2_stype());
        rs1_val.set_sign(rs2_val.sign() ^ 1);
        self.fregs.write(inst.get_rd(), rs1_val);
    }

    fn inst_fsgnjx_s(&mut self, inst: &inst_type::InstType) {
        let mut rs1_val = self.fregs.read(inst.get_rs1());
        let rs2_val = self.fregs.read(inst.get_rs2_stype());
        rs1_val.set_sign(rs2_val.sign() ^ rs1_val.sign());
        self.fregs.write(inst.get_rd(), rs1_val);
    }

    fn inst_fence(&mut self, _inst: &inst_type::InstType) {}

    fn inst_feq_s(&mut self, inst: &inst_type::InstType) {
        //let rs1_val = self.fregs.read(inst.get_rs1()).to_f32(RoundingMode::TiesToEven);
        //let rs2_val = self.fregs.read(inst.get_rs2_stype()).to_f32(RoundingMode::TiesToEven);
        let rs1_val = self.fregs.read(inst.get_rs1());
        let rs2_val = self.fregs.read(inst.get_rs2_stype());

        if rs1_val.is_signaling_nan() || rs2_val.is_signaling_nan() {
            self.csregs.write(0x1, 0x10);
            self.regs.write(inst.get_rd(), 0);
        } else {
            if rs1_val.eq(rs2_val) {
                self.regs.write(inst.get_rd(), 1);
            } else {
                self.regs.write(inst.get_rd(), 0);
            }
        }
    }

    fn inst_fld(&mut self, inst: &inst_type::InstType) {
        let base = self.regs.read(inst.get_rs1());
        let offset = RVCore::sign_extend(inst.get_imm_itype(), 12);
        let addr = base.wrapping_add(offset);
        let mut data = [0; 8];
        self.read_memory(addr, &mut data);

        let f64_val = F64::from_bits(u64::from_le_bytes(data));
        if f64_val.is_signaling_nan() || u64::from_le_bytes(data) == 0x7f800001 {
            self.fregs
                .write(inst.get_rd(), F64::from_bits(0x7ff0000000000001));
        } else {
            self.fregs
                .write(inst.get_rd(), f64_val);
        }
    }

    fn inst_fle_s(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self
            .fregs
            .read(inst.get_rs1())
            .to_f32(RoundingMode::TiesToEven);
        let rs2_val = self
            .fregs
            .read(inst.get_rs2_stype())
            .to_f32(RoundingMode::TiesToEven);

        if rs1_val.is_nan() || rs2_val.is_nan() {
            self.csregs.write(0x1, 0x10);
            self.regs.write(inst.get_rd(), 0);
        } else {
            if rs1_val.le(rs2_val) {
                self.regs.write(inst.get_rd(), 1);
            } else {
                self.regs.write(inst.get_rd(), 0);
            }
        }
    }

    fn inst_flt_s(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self
            .fregs
            .read(inst.get_rs1())
            .to_f32(RoundingMode::TiesToEven);
        let rs2_val = self
            .fregs
            .read(inst.get_rs2_stype())
            .to_f32(RoundingMode::TiesToEven);

        if rs1_val.is_nan() || rs2_val.is_nan() {
            self.csregs.write(0x1, 0x10);
            self.regs.write(inst.get_rd(), 0);
        } else {
            if rs1_val.lt(rs2_val) {
                self.regs.write(inst.get_rd(), 1);
            } else {
                self.regs.write(inst.get_rd(), 0);
            }
        }
    }

    fn inst_flw(&mut self, inst: &inst_type::InstType) {
        let base = self.regs.read(inst.get_rs1());
        let offset = RVCore::sign_extend(inst.get_imm_itype(), 12);
        let addr = base.wrapping_add(offset);
        let mut data = [0; 4];
        self.read_memory(addr, &mut data);

        let f32_val = F32::from_bits(u32::from_le_bytes(data));
        if f32_val.is_signaling_nan() || u32::from_le_bytes(data) == 0x7f800001 {
            self.fregs
                .write(inst.get_rd(), F64::from_bits(0x7ff0000000000001));
        } else {
            self.fregs
                .write(inst.get_rd(), f32_val.to_f64(RoundingMode::TiesToEven));
        }
    }

    fn inst_fsd(&mut self, inst: &inst_type::InstType) {
        let base = self.regs.read(inst.get_rs1());
        let offset = RVCore::sign_extend(inst.get_imm_btype(), 12);
        let addr = base.wrapping_add(offset);
        let data = self
            .fregs
            .read(inst.get_rs2_stype())
            .to_bits();
        self.write_memory(addr, &data.to_le_bytes());
    }

    fn inst_fsw(&mut self, inst: &inst_type::InstType) {
        let base = self.regs.read(inst.get_rs1());
        let offset = RVCore::sign_extend(inst.get_imm_btype(), 12);
        let addr = base.wrapping_add(offset);
        let data = self
            .fregs
            .read(inst.get_rs2_stype())
            .to_f32(RoundingMode::TiesToEven)
            .to_bits();
        self.write_memory(addr, &data.to_le_bytes());
    }

    fn inst_fmul_d(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self
            .fregs
            .read(inst.get_rs1());
        let rs2_val = self
            .fregs
            .read(inst.get_rs2_stype());

        let mut flag = ExceptionFlags::default();
        flag.set();
        let result = rs1_val.mul(rs2_val, RoundingMode::TiesToEven);
        flag.get();

        if flag.is_inexact() {
            self.csregs.write(csregs::FFLAGS, 1);
        }

        self.fregs.write(inst.get_rd(), result);
    }

    fn inst_fmul_s(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self
            .fregs
            .read(inst.get_rs1())
            .to_f32(RoundingMode::TiesToEven);
        let rs2_val = self
            .fregs
            .read(inst.get_rs2_stype())
            .to_f32(RoundingMode::TiesToEven);

        let mut flag = ExceptionFlags::default();
        flag.set();
        let result = rs1_val.mul(rs2_val, RoundingMode::TiesToEven);
        flag.get();

        if flag.is_inexact() {
            self.csregs.write(csregs::FFLAGS, 1);
        }

        self.fregs
            .write(inst.get_rd(), result.to_f64(RoundingMode::TiesToEven));
    }

    fn inst_fmv_w_x(&mut self, inst: &inst_type::InstType) {
        let rs1 = inst.get_rs1();
        let rs1_lower_val = self.regs.read(rs1) as u32;

        let f32_val = F32::from_bits(rs1_lower_val);
        if f32_val.is_signaling_nan() {
            self.fregs
                .write(inst.get_rd(), F64::from_bits(0x7ff0000000000001));
        } else {
            self.fregs
                .write(inst.get_rd(), f32_val.to_f64(RoundingMode::TiesToEven));
        }
    }

    fn inst_fmv_x_d(&mut self, inst: &inst_type::InstType) {
        let rs1 = inst.get_rs1();
        let rs1_val = self.fregs.read(rs1);
        self.regs.write(inst.get_rd(), rs1_val.to_bits());
    }

    fn inst_fmv_x_w(&mut self, inst: &inst_type::InstType) {
        let rs1 = inst.get_rs1();
        let rs1_val = self.fregs.read(rs1);
        self.regs.write(
            inst.get_rd(),
            RVCore::sign_extend(
                (rs1_val.to_f32(RoundingMode::TiesToEven).to_bits() & 0xffffffff) as AddressType,
                32,
            ),
        );
    }

    fn inst_fsub_d(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self
            .fregs
            .read(inst.get_rs1());
        let rs2_val = self
            .fregs
            .read(inst.get_rs2_stype());

        let mut flag = ExceptionFlags::default();
        flag.set();
        let result = rs1_val.sub(rs2_val, RoundingMode::TiesToEven);
        flag.get();
        self.update_fflags(&flag);

        if rs1_val.is_positive_infinity() && rs2_val.is_positive_infinity() {
            self.fregs.write(inst.get_rd(), F64::quiet_nan());
        } else {
            self.fregs.write(inst.get_rd(), result);
        }
    }

    fn inst_fsub_s(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self
            .fregs
            .read(inst.get_rs1())
            .to_f32(RoundingMode::TiesToEven);
        let rs2_val = self
            .fregs
            .read(inst.get_rs2_stype())
            .to_f32(RoundingMode::TiesToEven);

        let mut flag = ExceptionFlags::default();
        flag.set();
        let result = rs1_val.sub(rs2_val, RoundingMode::TiesToEven);
        flag.get();
        self.update_fflags(&flag);

        if rs1_val.is_positive_infinity() && rs2_val.is_positive_infinity() {
            self.fregs.write(inst.get_rd(), F64::quiet_nan());
        } else {
            self.fregs
                .write(inst.get_rd(), result.to_f64(RoundingMode::TiesToEven));
        }
    }

    fn inst_jal(&mut self, inst: &inst_type::InstType) {
        self.regs.write(inst.get_rd(), self.pc + 4);
        let imm = inst.get_imm_jtype();
        let mut offset = (((imm >> 19) & 1) << 20)
            | (((imm >> 9) & 0x3ff) << 1)
            | (((imm >> 8) & 0x1) << 11)
            | (((imm >> 0) & 0xff) << 12);
        offset = RVCore::sign_extend(offset, 21);
        self.pc = self.pc.wrapping_add(offset) - inst.len;
        //print!(" {},{:x}", XRegisters::name(inst.get_rd()), self.pc);
    }

    fn sign_extend(mut input: AddressType, input_bit_len: usize) -> AddressType {
        let mask = 1 << (input_bit_len - 1);
        input &= (1 << input_bit_len) - 1;
        (input ^ mask).wrapping_sub(mask)
    }

    fn inst_jalr(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1());
        self.regs.write(inst.get_rd(), self.pc + 4);
        let offset = RVCore::sign_extend(inst.get_imm_itype(), 12);
        self.pc = ((rs1_val.wrapping_add(offset)) & (AddressType::max_value() - 1)) - inst.len;
    }

    fn inst_lb(&mut self, inst: &inst_type::InstType) {
        let imm = RVCore::sign_extend(inst.get_imm_itype(), 12);
        let address = self.regs.read(inst.get_rs1()).wrapping_add(imm);
        let mut data = [0; 1];
        self.read_memory(address, &mut data);
        self.regs.write(
            inst.get_rd(),
            RVCore::sign_extend(RVCore::byte_array_to_addr_type(&data), 8),
        );
    }

    fn inst_lbu(&mut self, inst: &inst_type::InstType) {
        let imm = RVCore::sign_extend(inst.get_imm_itype(), 12);
        let address = self.regs.read(inst.get_rs1()).wrapping_add(imm);
        let mut data = [0; 1];
        self.read_memory(address, &mut data);
        let wdata = RVCore::byte_array_to_addr_type(&data);
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

    fn inst_lh(&mut self, inst: &inst_type::InstType) {
        let imm = RVCore::sign_extend(inst.get_imm_itype(), 12);
        let address = self.regs.read(inst.get_rs1()).wrapping_add(imm);
        let mut data = [0; 2];
        self.read_memory(address, &mut data);
        self.regs.write(
            inst.get_rd(),
            RVCore::sign_extend(RVCore::byte_array_to_addr_type(&data), 16),
        );
    }

    fn inst_lhu(&mut self, inst: &inst_type::InstType) {
        let imm = RVCore::sign_extend(inst.get_imm_itype(), 12);
        let address = self.regs.read(inst.get_rs1()).wrapping_add(imm);
        let mut data = [0; 2];
        self.read_memory(address, &mut data);
        let wdata = RVCore::byte_array_to_addr_type(&data);
        self.regs.write(inst.get_rd(), wdata);
    }

    fn inst_lui(&mut self, inst: &inst_type::InstType) {
        self.regs
            .write(inst.get_rd(), RVCore::sign_extend(inst.get_imm_utype(), 32));
    }

    fn inst_lw(&mut self, inst: &inst_type::InstType) {
        let imm = RVCore::sign_extend(inst.get_imm_itype(), 12);
        let address = self.regs.read(inst.get_rs1()).wrapping_add(imm);
        let mut data = [0; 4];
        self.read_memory(address, &mut data);
        self.regs.write(
            inst.get_rd(),
            RVCore::sign_extend(RVCore::byte_array_to_addr_type(&data), 32),
        );
    }

    fn inst_lwu(&mut self, inst: &inst_type::InstType) {
        let imm = RVCore::sign_extend(inst.get_imm_itype(), 12);
        let address = self.regs.read(inst.get_rs1()).wrapping_add(imm);
        let mut data = [0; 4];
        self.read_memory(address, &mut data);
        self.regs
            .write(inst.get_rd(), RVCore::byte_array_to_addr_type(&data));
    }

    fn inst_mul(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1()) as i64;
        let rs2_val = self.regs.read(inst.get_rs2_rtype()) as i64;
        self.regs
            .write(inst.get_rd(), rs1_val.wrapping_mul(rs2_val) as u64);
    }

    fn inst_mulh(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1()) as i64 as i128;
        let rs2_val = self.regs.read(inst.get_rs2_rtype()) as i64 as i128;
        self.regs.write(
            inst.get_rd(),
            (rs1_val.wrapping_mul(rs2_val) >> 64) as AddressType,
        );
    }

    fn inst_mulhsu(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1()) as i64 as i128 as u128;
        let rs2_val = self.regs.read(inst.get_rs2_rtype()) as u128;
        self.regs.write(
            inst.get_rd(),
            (rs1_val.wrapping_mul(rs2_val) >> 64) as AddressType,
        );
    }

    fn inst_mulhu(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1()) as u128;
        let rs2_val = self.regs.read(inst.get_rs2_rtype()) as u128;
        self.regs.write(
            inst.get_rd(),
            (rs1_val.wrapping_mul(rs2_val) >> 64) as AddressType,
        );
    }

    fn inst_mulw(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1()) as i32;
        let rs2_val = self.regs.read(inst.get_rs2_rtype()) as i32;
        self.regs.write(
            inst.get_rd(),
            rs1_val.wrapping_mul(rs2_val) as i64 as AddressType,
        );
    }

    fn inst_mret(&mut self, _inst: &inst_type::InstType) {
        self.mode = PrivilegeMode::U;
    }

    fn inst_or(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1());
        let rs2_val = self.regs.read(inst.get_rs2_rtype());
        self.regs.write(inst.get_rd(), rs1_val | rs2_val);
    }

    fn inst_ori(&mut self, inst: &inst_type::InstType) {
        let imm = RVCore::sign_extend(inst.get_imm_itype(), 12);
        self.regs
            .write(inst.get_rd(), self.regs.read(inst.get_rs1()) | imm);
    }

    fn inst_rem(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1()) as i64;
        let rs2_val = self.regs.read(inst.get_rs2_rtype()) as i64;
        if rs2_val == 0 {
            self.regs.write(inst.get_rd(), rs1_val as AddressType);
        } else {
            self.regs
                .write(inst.get_rd(), rs1_val.wrapping_rem(rs2_val) as AddressType);
        }
    }

    fn inst_remu(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1());
        let rs2_val = self.regs.read(inst.get_rs2_rtype());
        if rs2_val == 0 {
            self.regs.write(inst.get_rd(), rs1_val);
        } else {
            self.regs
                .write(inst.get_rd(), rs1_val.wrapping_rem(rs2_val));
        }
    }

    fn inst_remuw(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1()) as u32;
        let rs2_val = self.regs.read(inst.get_rs2_rtype()) as u32;
        if rs2_val == 0 {
            self.regs.write(
                inst.get_rd(),
                RVCore::sign_extend(rs1_val as AddressType, 32),
            );
        } else {
            let result = rs1_val.wrapping_rem(rs2_val) as AddressType;
            self.regs
                .write(inst.get_rd(), RVCore::sign_extend(result, 32));
        }
    }

    fn inst_remw(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1()) as i32;
        let rs2_val = self.regs.read(inst.get_rs2_rtype()) as i32;
        if rs2_val == 0 {
            self.regs.write(inst.get_rd(), rs1_val as AddressType);
        } else {
            self.regs
                .write(inst.get_rd(), rs1_val.wrapping_rem(rs2_val) as AddressType);
        }
    }

    fn inst_sb(&mut self, inst: &inst_type::InstType) {
        let imm = RVCore::sign_extend(inst.get_imm_stype(), 12);
        let address = self.regs.read(inst.get_rs1()).wrapping_add(imm);
        let data = self.regs.read(inst.get_rs2_stype()) as u8;
        self.write_memory(address, &data.to_le_bytes());
    }

    fn inst_sd(&mut self, inst: &inst_type::InstType) {
        let imm = RVCore::sign_extend(inst.get_imm_stype(), 12);
        let address = self.regs.read(inst.get_rs1()).wrapping_add(imm);
        let data = self.regs.read(inst.get_rs2_stype()) as u64;
        self.write_memory(address, &data.to_le_bytes());
    }

    fn inst_sh(&mut self, inst: &inst_type::InstType) {
        let imm = RVCore::sign_extend(inst.get_imm_stype(), 12);
        let address = self.regs.read(inst.get_rs1()).wrapping_add(imm);
        let data = self.regs.read(inst.get_rs2_stype()) as u16;
        self.write_memory(address, &data.to_le_bytes());
    }

    fn inst_sw(&mut self, inst: &inst_type::InstType) {
        let imm = RVCore::sign_extend(inst.get_imm_stype(), 12);
        let address = self.regs.read(inst.get_rs1()).wrapping_add(imm);
        let data = self.regs.read(inst.get_rs2_stype()) as u32;
        self.write_memory(address, &data.to_le_bytes());
    }

    fn inst_sll(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1());
        let rs2_val = self.regs.read(inst.get_rs2_rtype());
        self.regs.write(inst.get_rd(), rs1_val << (rs2_val & 0x3f));
    }

    fn inst_slli(&mut self, inst: &inst_type::InstType) {
        let shamt = inst.get_imm_itype();
        let rs1_val = self.regs.read(inst.get_rs1());
        self.regs.write(inst.get_rd(), rs1_val << shamt);
    }

    fn inst_slliw(&mut self, inst: &inst_type::InstType) {
        let shamt = inst.get_shamt_itype() & 0x3f;
        let rs1_val = self.regs.read(inst.get_rs1());
        self.regs
            .write(inst.get_rd(), RVCore::sign_extend(rs1_val << shamt, 32));
    }

    fn inst_sllw(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1()) as u32;
        let rs2_val = self.regs.read(inst.get_rs2_rtype()) & 0x1f;
        let result = rs1_val << rs2_val;
        self.regs.write(
            inst.get_rd(),
            RVCore::sign_extend(result as AddressType, 32),
        );
    }

    fn inst_slt(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1()) as i64;
        let rs2_val = self.regs.read(inst.get_rs2_rtype()) as i64;
        if rs1_val < rs2_val {
            self.regs.write(inst.get_rd(), 1);
        } else {
            self.regs.write(inst.get_rd(), 0);
        }
    }

    fn inst_slti(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1()) as i64;
        let imm = RVCore::sign_extend(inst.get_imm_itype(), 12) as i64;
        if rs1_val < imm {
            self.regs.write(inst.get_rd(), 1);
        } else {
            self.regs.write(inst.get_rd(), 0);
        }
    }

    fn inst_sltiu(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1());
        let imm = RVCore::sign_extend(inst.get_imm_itype(), 12);
        if rs1_val < imm {
            self.regs.write(inst.get_rd(), 1);
        } else {
            self.regs.write(inst.get_rd(), 0);
        }
    }

    fn inst_sltu(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1());
        let rs2_val = self.regs.read(inst.get_rs2_rtype());
        if rs1_val < rs2_val {
            self.regs.write(inst.get_rd(), 1);
        } else {
            self.regs.write(inst.get_rd(), 0);
        }
    }

    fn inst_srli(&mut self, inst: &inst_type::InstType) {
        let shamt = inst.get_shamt_itype() & 0x3f;
        let rs1_val = self.regs.read(inst.get_rs1());
        self.regs.write(inst.get_rd(), rs1_val >> shamt);
    }

    fn inst_sra(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1()) as i64;
        let rs2_val = self.regs.read(inst.get_rs2_rtype()) & 0x3f;
        self.regs
            .write(inst.get_rd(), (rs1_val >> rs2_val) as AddressType);
    }

    fn inst_srai(&mut self, inst: &inst_type::InstType) {
        let shamt = inst.get_shamt_itype();
        let rs1_val = self.regs.read(inst.get_rs1());
        self.regs
            .write(inst.get_rd(), ((rs1_val as i64) >> shamt) as AddressType);
    }

    fn inst_sraiw(&mut self, inst: &inst_type::InstType) {
        let shamt = inst.get_shamt_itype() & 0x1f;
        let rs1_val = self.regs.read(inst.get_rs1()) as i32;
        let result = rs1_val >> shamt;
        self.regs.write(
            inst.get_rd(),
            RVCore::sign_extend(result as AddressType, 32),
        );
    }

    fn inst_sraw(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1()) as i32;
        let rs2_val = self.regs.read(inst.get_rs2_rtype()) & 0x1f;
        let result = (rs1_val >> rs2_val) as AddressType;
        self.regs
            .write(inst.get_rd(), RVCore::sign_extend(result, 32));
    }

    fn inst_srl(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1());
        let rs2_val = self.regs.read(inst.get_rs2_rtype()) & 0x3f;
        self.regs.write(inst.get_rd(), rs1_val >> rs2_val);
    }

    fn inst_srliw(&mut self, inst: &inst_type::InstType) {
        let shamt = inst.get_shamt_itype() & 0x1f;
        let rs1_val = self.regs.read(inst.get_rs1()) as u32;
        let result = rs1_val >> shamt;
        self.regs.write(
            inst.get_rd(),
            RVCore::sign_extend(result as AddressType, 32),
        );
    }

    fn inst_srlw(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1()) as u32;
        let rs2_val = self.regs.read(inst.get_rs2_rtype()) & 0x1f;
        let result = (rs1_val >> rs2_val) as AddressType;
        self.regs
            .write(inst.get_rd(), RVCore::sign_extend(result, 32));
    }

    fn inst_sub(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1());
        let rs2_val = self.regs.read(inst.get_rs2_rtype());
        self.regs
            .write(inst.get_rd(), rs1_val.wrapping_sub(rs2_val));
    }

    fn inst_subw(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1()) as u32;
        let rs2_val = self.regs.read(inst.get_rs2_rtype()) as u32;
        let result = rs1_val.wrapping_sub(rs2_val) as AddressType;
        self.regs
            .write(inst.get_rd(), RVCore::sign_extend(result, 32));
    }

    fn inst_nop(&mut self, _inst: &inst_type::InstType) {}

    fn inst_xor(&mut self, inst: &inst_type::InstType) {
        let rs1_val = self.regs.read(inst.get_rs1());
        let rs2_val = self.regs.read(inst.get_rs2_rtype());
        self.regs.write(inst.get_rd(), rs1_val ^ rs2_val);
    }

    fn inst_xori(&mut self, inst: &inst_type::InstType) {
        let imm = RVCore::sign_extend(inst.get_imm_itype(), 12);
        self.regs
            .write(inst.get_rd(), self.regs.read(inst.get_rs1()) ^ imm);
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

    struct Fixture {
        core: RVCore,
        mem_stub: Rc<RefCell<MemoryStub>>,
    }

    impl Fixture {
        fn new() -> Fixture {
            let mut new_fixture = Fixture {
                core: RVCore::new(),
                mem_stub: Rc::new(RefCell::new(MemoryStub::default())),
            };

            let mem_if: Rc<RefCell<dyn MemoryInterface>> = new_fixture.mem_stub.clone();
            new_fixture.core.bind_mem(mem_if);
            new_fixture
        }
    }

    #[test]
    fn test_inst_auipc() {
        let mut fixture = Fixture::new();
        fixture.core.pc = 0x1234;
        fixture.core.inst_auipc(&inst_auipc_code(1, 0xffff1000));
        assert_eq!(0xffff1000 + 0x1234, fixture.core.regs.read(1));
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
        assert_eq!(0x7fe - 4, core.pc);

        core.pc = 0;
        core.regs.write(2, 0x1230);
        core.regs.write(3, 0x1234);
        core.inst_bgeu(&inst_bgeu_code(2, 3, 0x7fe));
        assert_eq!(0, core.pc);
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
        assert_eq!(0xfe - 2, core.pc);

        core.pc = 0;
        core.regs.write(10, 1);
        core.inst_c_beqz(&inst_c_beqz_code(10, 0xfe));
        assert_eq!(0, core.pc);
    }

    #[test]
    fn test_inst_c_bnez() {
        let mut core: RVCore = RVCore::new();
        core.regs.write(10, 1);
        core.inst_c_bnez(&inst_c_bnez_code(10, 0xfe));
        assert_eq!(0xfe - 2, core.pc);

        core.pc = 0;
        core.regs.write(10, 0);
        core.inst_c_bnez(&inst_c_bnez_code(10, 0xfe));
        assert_eq!(0, core.pc);
    }

    #[test]
    fn test_inst_c_j() {
        let mut core: RVCore = RVCore::new();
        core.pc = 0xfff0;
        core.inst_c_j(&inst_c_j_code(0xfe));
        assert_eq!(0xfff0 + 0xfe - 2, core.pc);
    }

    #[test]
    fn test_inst_c_jr() {
        let mut core: RVCore = RVCore::new();
        core.pc = 0x0;
        core.regs.write(8, 0x6666);
        core.inst_c_jr(&inst_c_jr_code(8));
        assert_eq!(0x6664, core.pc);
    }

    #[test]
    fn test_inst_c_swsp() {
        let mut ft = Fixture::new();

        ft.core.regs.write(1, 0x12345678); // Data
        ft.core.regs.write(2, 0x8888); // Address
        ft.core.inst_c_swsp(&inst_c_swsp_code(1, 0x4));

        let result = &ft.mem_stub.borrow().buffer;
        assert_eq!(MemoryOperation::WRITE, result.op);
        assert_eq!(0x888c, result.addr);
        assert_eq!([0x78, 0x56, 0x34, 0x12].to_vec(), result.data);
    }

    #[test]
    fn test_inst_c_lw() {
        let mut ft = Fixture::new();

        ft.core.regs.write(9, 0x8888); // Address
        ft.core.inst_c_lw(&inst_c_lw_code(8, 9, 0x4));

        let result = &ft.mem_stub.borrow().buffer;
        assert_eq!(MemoryOperation::READ, result.op);
        assert_eq!(0x888c, result.addr);
    }

    #[test]
    fn test_inst_c_lwsp() {
        let mut ft = Fixture::new();

        ft.core.regs.write(2, 0x8888); // Address
        ft.core.inst_c_lwsp(&inst_c_lwsp_code(1, 0x4));

        let result = &ft.mem_stub.borrow().buffer;
        assert_eq!(MemoryOperation::READ, result.op);
        assert_eq!(0x888c, result.addr);
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
        let mut ft = Fixture::new();

        ft.core.regs.write(8, 0x12345678); // Data
        ft.core.regs.write(9, 0x8888); // Address
        ft.core.inst_c_sd(&inst_c_sd_code(8, 9, 0x18));
        assert_eq!(MemoryOperation::WRITE, ft.mem_stub.borrow().buffer.op);
        assert_eq!(0x8888 + 0x18, ft.mem_stub.borrow().buffer.addr);
        assert_eq!(
            [0x78, 0x56, 0x34, 0x12, 0, 0, 0, 0].to_vec(),
            ft.mem_stub.borrow().buffer.data
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
        assert_eq!(0xff00 - 4, core.pc);
    }

    #[test]
    fn test_inst_jalr() {
        let mut core: RVCore = RVCore::new();
        core.regs.write(8, 0xffffffff);
        core.regs.write(9, 0x66);

        //0xfff will be masked into 0xffe (-2)
        core.inst_jalr(&inst_jalr_code(8, 9, 0xfff));
        assert_eq!(4, core.regs.read(8));
        assert_eq!(0x66 - 4 - 2, core.pc);
    }

    #[test]
    fn test_inst_ld() {
        let mut ft = Fixture::new();

        ft.core.regs.write(1, 0x8888); // Address
        ft.core.inst_ld(&inst_ld_code(2, 1, 0xffe));
        assert_eq!(MemoryOperation::READ, ft.mem_stub.borrow().buffer.op);
        assert_eq!(0x8888 - 2, ft.mem_stub.borrow().buffer.addr);
    }

    #[test]
    fn test_inst_lw() {
        let mut fixture = Fixture::new();
        fixture.core.regs.write(1, 0x8888); // Address
        fixture.core.inst_lw(&inst_lw_code(2, 1, 0x7f0));
        assert_eq!(MemoryOperation::READ, fixture.mem_stub.borrow().buffer.op);
        assert_eq!(0x8888 + 0x7f0, fixture.mem_stub.borrow().buffer.addr);
    }

    #[test]
    fn test_inst_sb() {
        let mut ft = Fixture::new();

        ft.core.regs.write(1, 0xffffff78); // Data
        ft.core.regs.write(2, 0x8888); // Address
        ft.core.inst_sb(&inst_sb_code(1, 2, 0xff));
        assert_eq!(MemoryOperation::WRITE, ft.mem_stub.borrow().buffer.op);
        assert_eq!(0x8888 + 0xff, ft.mem_stub.borrow().buffer.addr);
        assert_eq!([0x78].to_vec(), ft.mem_stub.borrow().buffer.data);
    }

    #[test]
    fn test_inst_sd() {
        let mut fixture = Fixture::new();
        fixture.core.regs.write(1, 0x8888); // Address
        fixture.core.inst_lw(&inst_lw_code(2, 1, 0x7f0));

        fixture.core.regs.write(1, 0xffffff78); // Data
        fixture.core.regs.write(2, 0x8888); // Address
        fixture.core.inst_sd(&inst_sd_code(1, 2, 0xff));
        assert_eq!(MemoryOperation::WRITE, fixture.mem_stub.borrow().buffer.op);
        assert_eq!(0x8888 + 0xff, fixture.mem_stub.borrow().buffer.addr);
        assert_eq!(
            [0x78, 0xff, 0xff, 0xff, 0, 0, 0, 0].to_vec(),
            fixture.mem_stub.borrow().buffer.data
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
        let mut fixture = Fixture::new();

        fixture.core.regs.write(1, 0x0); // rd
        fixture.core.regs.write(2, AddressType::MAX); // rs1
        fixture.core.inst_srai(&inst_srai_code(1, 2, 0x10));
        assert_eq!(AddressType::MAX, fixture.core.regs.read(1));
    }
}
