pub struct FRegisters {
    reg_bank: [f64; 32],
}

impl FRegisters {
    pub fn new() -> FRegisters {
        FRegisters {
            reg_bank: [0.0; 32],
        }
    }

    pub fn read(&self, i: usize) -> f64 {
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
