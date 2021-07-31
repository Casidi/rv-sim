#[derive(PartialEq, Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum InstID {
    AUIPC,
    ADDI,
    BGEU,
    C_ADDI,
    C_JAL,
    C_SWSP,
    C_LWSP,
    C_LI,
    C_MV,
    C_SUB,
    JAL,
    LW,
    SB,
    NOP,
    INVALID,
}

pub struct InstInfo<'a> {
    pub name: &'a str,
}

#[allow(non_upper_case_globals)]
pub const inst_info_table: &[InstInfo] = &[
    InstInfo { name: "auipc" },
    InstInfo { name: "addi" },
    InstInfo { name: "bgeu" },
    InstInfo { name: "c.addi" },
    InstInfo { name: "c.jal" },
    InstInfo { name: "c.swsp" },
    InstInfo { name: "c.lwsp" },
    InstInfo { name: "c.li" },
    InstInfo { name: "c.mv" },
    InstInfo { name: "c.sub" },
    InstInfo { name: "jal" },
    InstInfo { name: "lw" },
    InstInfo { name: "sb" },
    InstInfo { name: "nop" },
    InstInfo { name: "invalid" },
];
