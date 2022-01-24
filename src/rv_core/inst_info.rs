#[derive(PartialEq, Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum InstID {
    AUIPC,
    ADD,
    ADDI,
    ADDIW,
    ADDW,
    AMOADD_D,
    AMOADD_W,
    AMOAND_D,
    AMOAND_W,
    AMOMAX_D,
    AMOMAX_W,
    AMOMAXU_D,
    AMOMAXU_W,
    AMOMIN_D,
    AMOMIN_W,
    AMOMINU_D,
    AMOMINU_W,
    AMOOR_D,
    AMOOR_W,
    AMOSWAP_D,
    AMOSWAP_W,
    AMOXOR_D,
    AMOXOR_W,
    AND,
    ANDI,
    BEQ,
    BGE,
    BGEU,
    BLT,
    BLTU,
    BNE,
    C_ADD,
    C_ADDI,
    C_ADDIW,
    C_ADDI16SP,
    C_ADDI4SPN,
    C_ADDW,
    C_AND,
    C_ANDI,
    C_BEQZ,
    C_BNEZ,
    C_FSDSP,
    C_J,
    C_JALR,
    C_JR,
    C_SDSP,
    C_SLLI,
    C_SRAI,
    C_SRLI,
    C_SW,
    C_SWSP,
    C_LD,
    C_LDSP,
    C_LW,
    C_LWSP,
    C_LI,
    C_LUI,
    C_MV,
    C_OR,
    C_SD,
    C_SUB,
    C_SUBW,
    C_XOR,
    CSRRCI,
    CSRRS,
    CSRRW,
    CSRRWI,
    DIV,
    DIVU,
    DIVUW,
    DIVW,
    ECALL,
    FADD_D,
    FADD_S,
    FCLASS_D,
    FCLASS_S,
    FCVT_D_L,
    FCVT_D_LU,
    FCVT_D_S,
    FCVT_D_W,
    FCVT_D_WU,
    FCVT_L_D,
    FCVT_L_S,
    FCVT_LU_D,
    FCVT_LU_S,
    FCVT_S_D,
    FCVT_S_L,
    FCVT_S_LU,
    FCVT_S_W,
    FCVT_S_WU,
    FCVT_W_D,
    FCVT_W_S,
    FCVT_WU_D,
    FCVT_WU_S,
    FDIV_D,
    FDIV_S,
    FMADD_D,
    FMADD_S,
    FMAX_D,
    FMAX_S,
    FMIN_D,
    FMIN_S,
    FMSUB_D,
    FMSUB_S,
    FNMADD_D,
    FNMADD_S,
    FNMSUB_D,
    FNMSUB_S,
    FSQRT_D,
    FSQRT_S,
    FSGNJ_D,
    FSGNJ_S,
    FSGNJN_D,
    FSGNJN_S,
    FSGNJX_D,
    FSGNJX_S,
    FENCE,
    FEQ_D,
    FEQ_S,
    FLD,
    FLE_D,
    FLE_S,
    FLT_D,
    FLT_S,
    FLW,
    FSD,
    FSW,
    FMUL_D,
    FMUL_S,
    FMV_D_X,
    FMV_W_X,
    FMV_X_D,
    FMV_X_W,
    FSUB_D,
    FSUB_S,
    JAL,
    JALR,
    LB,
    LBU,
    LD,
    LH,
    LHU,
    LR_D,
    LR_W,
    LUI,
    LW,
    LWU,
    MUL,
    MULH,
    MULHSU,
    MULHU,
    MULW,
    MRET,
    OR,
    ORI,
    REM,
    REMU,
    REMUW,
    REMW,
    SB,
    SC_D,
    SC_W,
    SD,
    SH,
    SW,
    SLL,
    SLLI,
    SLLIW,
    SLLW,
    SLT,
    SLTI,
    SLTIU,
    SLTU,
    SRL,
    SRLI,
    SRLIW,
    SRLW,
    SRA,
    SRAI,
    SRAIW,
    SRAW,
    SUB,
    SUBW,
    NOP,
    XOR,
    XORI,
    INVALID,
}

pub struct InstInfo<'a> {
    pub name: &'a str,
}

#[allow(non_upper_case_globals)]
pub const inst_info_table: &[InstInfo] = &[
    InstInfo { name: "auipc" },
    InstInfo { name: "add" },
    InstInfo { name: "addi" },
    InstInfo { name: "addiw" },
    InstInfo { name: "addw" },
    InstInfo { name: "amoadd.d" },
    InstInfo { name: "amoadd.w" },
    InstInfo { name: "amoand.d" },
    InstInfo { name: "amoand.w" },
    InstInfo { name: "amomax.d" },
    InstInfo { name: "amomax.w" },
    InstInfo { name: "amomaxu.d" },
    InstInfo { name: "amomaxu.w" },
    InstInfo { name: "amomin.d" },
    InstInfo { name: "amomin.w" },
    InstInfo { name: "amominu.d" },
    InstInfo { name: "amominu.w" },
    InstInfo { name: "amoor.d" },
    InstInfo { name: "amoor.w" },
    InstInfo { name: "amoswap.d" },
    InstInfo { name: "amoswap.w" },
    InstInfo { name: "amoxor.d" },
    InstInfo { name: "amoxor.w" },
    InstInfo { name: "and" },
    InstInfo { name: "andi" },
    InstInfo { name: "beq" },
    InstInfo { name: "bge" },
    InstInfo { name: "bgeu" },
    InstInfo { name: "blt" },
    InstInfo { name: "bltu" },
    InstInfo { name: "bne" },
    InstInfo { name: "c.add" },
    InstInfo { name: "c.addi" },
    InstInfo { name: "c.addiw" },
    InstInfo { name: "c.addi16sp" },
    InstInfo { name: "c.addi4spn" },
    InstInfo { name: "c.addw" },
    InstInfo { name: "c.and" },
    InstInfo { name: "c.andi" },
    InstInfo { name: "c.beqz" },
    InstInfo { name: "c.bnez" },
    InstInfo { name: "c.fsdsp" },
    InstInfo { name: "c.j" },
    InstInfo { name: "c.jalr" },
    InstInfo { name: "c.jr" },
    InstInfo { name: "c.sdsp" },
    InstInfo { name: "c.slli" },
    InstInfo { name: "c.srai" },
    InstInfo { name: "c.srli" },
    InstInfo { name: "c.sw" },
    InstInfo { name: "c.swsp" },
    InstInfo { name: "c.ld" },
    InstInfo { name: "c.ldsp" },
    InstInfo { name: "c.lw" },
    InstInfo { name: "c.lwsp" },
    InstInfo { name: "c.li" },
    InstInfo { name: "c.lui" },
    InstInfo { name: "c.mv" },
    InstInfo { name: "c.or" },
    InstInfo { name: "c.sd" },
    InstInfo { name: "c.sub" },
    InstInfo { name: "c.subw" },
    InstInfo { name: "c.xor" },
    InstInfo { name: "csrrci" },
    InstInfo { name: "csrrs" },
    InstInfo { name: "csrrw" },
    InstInfo { name: "csrrwi" },
    InstInfo { name: "div" },
    InstInfo { name: "divu" },
    InstInfo { name: "divuw" },
    InstInfo { name: "divw" },
    InstInfo { name: "ecall" },
    InstInfo { name: "fadd.d" },
    InstInfo { name: "fadd.s" },
    InstInfo { name: "fclass.d" },
    InstInfo { name: "fclass.s" },
    InstInfo { name: "fcvt.d.l" },
    InstInfo { name: "fcvt.d.lu" },
    InstInfo { name: "fcvt.d.s" },
    InstInfo { name: "fcvt.d.w" },
    InstInfo { name: "fcvt.d.wu" },
    InstInfo { name: "fcvt.l.d" },
    InstInfo { name: "fcvt.l.s" },
    InstInfo { name: "fcvt.lu.d" },
    InstInfo { name: "fcvt.lu.s" },
    InstInfo { name: "fcvt.s.d" },
    InstInfo { name: "fcvt.s.l" },
    InstInfo { name: "fcvt.s.lu" },
    InstInfo { name: "fcvt.s.w" },
    InstInfo { name: "fcvt.s.wu" },
    InstInfo { name: "fcvt.w.d" },
    InstInfo { name: "fcvt.w.s" },
    InstInfo { name: "fcvt.wu.d" },
    InstInfo { name: "fcvt.wu.s" },
    InstInfo { name: "fdiv.d" },
    InstInfo { name: "fdiv.s" },
    InstInfo { name: "fmadd.d" },
    InstInfo { name: "fmadd.s" },
    InstInfo { name: "fmax.d" },
    InstInfo { name: "fmax.s" },
    InstInfo { name: "fmin.d" },
    InstInfo { name: "fmin.s" },
    InstInfo { name: "fmsub.d" },
    InstInfo { name: "fmsub.s" },
    InstInfo { name: "fnmadd.d" },
    InstInfo { name: "fnmadd.s" },
    InstInfo { name: "fnmsub.d" },
    InstInfo { name: "fnmsub.s" },
    InstInfo { name: "fsqrt.d" },
    InstInfo { name: "fsqrt.s" },
    InstInfo { name: "fsgnj.d" },
    InstInfo { name: "fsgnj.s" },
    InstInfo { name: "fsgnjn.d" },
    InstInfo { name: "fsgnjn.s" },
    InstInfo { name: "fsgnjx.d" },
    InstInfo { name: "fsgnjx.s" },
    InstInfo { name: "fence" },
    InstInfo { name: "feq.d" },
    InstInfo { name: "feq.s" },
    InstInfo { name: "fld" },
    InstInfo { name: "fle.d" },
    InstInfo { name: "fle.s" },
    InstInfo { name: "flt.d" },
    InstInfo { name: "flt.s" },
    InstInfo { name: "flw" },
    InstInfo { name: "fsd" },
    InstInfo { name: "fsw" },
    InstInfo { name: "fmul.d" },
    InstInfo { name: "fmul.s" },
    InstInfo { name: "fmv.d.x" },
    InstInfo { name: "fmv.w.x" },
    InstInfo { name: "fmv.x.d" },
    InstInfo { name: "fmv.x.w" },
    InstInfo { name: "fsub.d" },
    InstInfo { name: "fsub.s" },
    InstInfo { name: "jal" },
    InstInfo { name: "jalr" },
    InstInfo { name: "lb" },
    InstInfo { name: "lbu" },
    InstInfo { name: "ld" },
    InstInfo { name: "lh" },
    InstInfo { name: "lhu" },
    InstInfo { name: "lr.d" },
    InstInfo { name: "lr.w" },
    InstInfo { name: "lui" },
    InstInfo { name: "lw" },
    InstInfo { name: "lwu" },
    InstInfo { name: "mul" },
    InstInfo { name: "mulh" },
    InstInfo { name: "mulhsu" },
    InstInfo { name: "mulhu" },
    InstInfo { name: "mulw" },
    InstInfo { name: "mret" },
    InstInfo { name: "or" },
    InstInfo { name: "ori" },
    InstInfo { name: "rem" },
    InstInfo { name: "remu" },
    InstInfo { name: "remuw" },
    InstInfo { name: "remw" },
    InstInfo { name: "sb" },
    InstInfo { name: "sc.c" },
    InstInfo { name: "sc.d" },
    InstInfo { name: "sd" },
    InstInfo { name: "sh" },
    InstInfo { name: "sw" },
    InstInfo { name: "sll" },
    InstInfo { name: "slli" },
    InstInfo { name: "slliw" },
    InstInfo { name: "sllw" },
    InstInfo { name: "slt" },
    InstInfo { name: "slti" },
    InstInfo { name: "sltiu" },
    InstInfo { name: "sltu" },
    InstInfo { name: "srl" },
    InstInfo { name: "srli" },
    InstInfo { name: "srliw" },
    InstInfo { name: "srlw" },
    InstInfo { name: "sra" },
    InstInfo { name: "srai" },
    InstInfo { name: "sraiw" },
    InstInfo { name: "sraw" },
    InstInfo { name: "sub" },
    InstInfo { name: "subw" },
    InstInfo { name: "nop" },
    InstInfo { name: "xor" },
    InstInfo { name: "xori" },
    InstInfo { name: "invalid" },
];
