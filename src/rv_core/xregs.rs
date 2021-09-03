type AddressType = u64;

pub struct XRegisters {
    reg_bank: [AddressType; 32],
}

impl XRegisters {
    pub fn new() -> XRegisters {
        XRegisters {
            reg_bank: [0; 32],
        }
    }

    pub fn read(&self, i: usize) -> AddressType {
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
