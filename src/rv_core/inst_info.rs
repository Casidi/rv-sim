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
    DIVW,
    ECALL,
    FENCE,
    FMV_W_X,
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
    MULW,
    MRET,
    OR,
    ORI,
    REMU,
    SB,
    SD,
    SH,
    SW,
    SLL,
    SLLI,
    SLLIW,
    SLLW,
    SLTIU,
    SRLI,
    SRAI,
    SRAIW,
    SUB,
    SUBW,
    NOP,
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
    InstInfo { name: "divw" },
    InstInfo { name: "ecall" },
    InstInfo { name: "fence" },
    InstInfo { name: "fmv.w.x" },
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
    InstInfo { name: "mulw" },
    InstInfo { name: "mret" },
    InstInfo { name: "or" },
    InstInfo { name: "ori" },
    InstInfo { name: "remu" },
    InstInfo { name: "sb" },
    InstInfo { name: "sd" },
    InstInfo { name: "sh" },
    InstInfo { name: "sw" },
    InstInfo { name: "sll" },
    InstInfo { name: "slli" },
    InstInfo { name: "slliw" },
    InstInfo { name: "sllw" },
    InstInfo { name: "sltiu" },
    InstInfo { name: "srli" },
    InstInfo { name: "srai" },
    InstInfo { name: "sraiw" },
    InstInfo { name: "sub" },
    InstInfo { name: "subw" },
    InstInfo { name: "nop" },
    InstInfo { name: "xori" },
    InstInfo { name: "invalid" },
];
