use crate::rv_core::RVCore;
use crate::rv_core::inst_type::InstType;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum InstID {
	AUIPC,
	ADDI,
	NOP,
}

pub struct InstInfo<'a> {
	name: &'a str,
    pub operate: fn(&mut RVCore, &InstType),
}

pub const inst_info_table: &[InstInfo] = &[
	InstInfo {name: "auipc", operate: RVCore::inst_auipc},
	InstInfo {name: "addi", operate: RVCore::inst_addi},
	InstInfo {name: "nop", operate: RVCore::inst_nop},
];
