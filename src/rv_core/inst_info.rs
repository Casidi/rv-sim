use crate::rv_core::inst_type::InstType;
use crate::rv_core::RVCore;

#[derive(PartialEq, Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum InstID {
    AUIPC,
    ADDI,
    C_ADDI,
    NOP,
}

pub struct InstInfo<'a> {
    pub name: &'a str,
    pub operate: fn(&mut RVCore, &InstType),
}

#[allow(non_upper_case_globals)]
pub const inst_info_table: &[InstInfo] = &[
    InstInfo {
        name: "auipc",
        operate: RVCore::inst_auipc,
    },
    InstInfo {
        name: "addi",
        operate: RVCore::inst_addi,
    },
    InstInfo {
        name: "c_addi",
        operate: RVCore::inst_c_addi,
    },
    InstInfo {
        name: "nop",
        operate: RVCore::inst_nop,
    },
];
