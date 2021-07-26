#[derive(PartialEq, Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum InstID {
    AUIPC,
    ADDI,
    C_ADDI,
    C_SWSP,
    C_LWSP,
    C_LI,
    SB,
    NOP,
}

pub struct InstInfo<'a> {
    pub name: &'a str,
}

#[allow(non_upper_case_globals)]
pub const inst_info_table: &[InstInfo] = &[
    InstInfo { name: "auipc" },
    InstInfo { name: "addi" },
    InstInfo { name: "c_addi" },
    InstInfo { name: "c_swsp" },
    InstInfo { name: "c_lwsp" },
    InstInfo { name: "c_li" },
    InstInfo { name: "sb" },
    InstInfo { name: "nop" },
];
