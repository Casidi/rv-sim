#[derive(PartialEq, Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum InstID {
    AUIPC,
    ADD,
    ADDI,
    ADDIW,
    ADDW,
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
    C_JAL,
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
    CSRRS,
    CSRRW,
    CSRRWI,
    DIV,
    DIVU,
    DIVUW,
    DIVW,
    ECALL,
    FADD_S,
    FCLASS_S,
    FCVT_L_S,
    FCVT_LU_S,
    FCVT_S_L,
    FCVT_S_LU,
    FCVT_S_W,
    FCVT_S_WU,
    FCVT_W_S,
    FCVT_WU_S,
    FENCE,
    FEQ_S,
    FLE_S,
    FLT_S,
    FLW,
    FMUL_S,
    FMV_W_X,
    FMV_X_W,
    FSUB_S,
    JAL,
    JALR,
    LB,
    LBU,
    LD,
    LH,
    LHU,
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
    InstInfo { name: "c.jal" },
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
    InstInfo { name: "csrrs" },
    InstInfo { name: "csrrw" },
    InstInfo { name: "csrrwi" },
    InstInfo { name: "div" },
    InstInfo { name: "divu" },
    InstInfo { name: "divuw" },
    InstInfo { name: "divw" },
    InstInfo { name: "ecall" },
    InstInfo { name: "fadd.s" },
    InstInfo { name: "fclass.s" },
    InstInfo { name: "fcvt.l.s" },
    InstInfo { name: "fcvt.lu.s" },
    InstInfo { name: "fcvt.s.l" },
    InstInfo { name: "fcvt.s.lu" },
    InstInfo { name: "fcvt.s.w" },
    InstInfo { name: "fcvt.s.wu" },
    InstInfo { name: "fcvt.w.s" },
    InstInfo { name: "fcvt.wu.s" },
    InstInfo { name: "fence" },
    InstInfo { name: "feq.s" },
    InstInfo { name: "fle.s" },
    InstInfo { name: "flt.s" },
    InstInfo { name: "flw" },
    InstInfo { name: "fmul.s" },
    InstInfo { name: "fmv.w.x" },
    InstInfo { name: "fmv.x.w" },
    InstInfo { name: "fsub.s" },
    InstInfo { name: "jal" },
    InstInfo { name: "jalr" },
    InstInfo { name: "lb" },
    InstInfo { name: "lbu" },
    InstInfo { name: "ld" },
    InstInfo { name: "lh" },
    InstInfo { name: "lhu" },
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
