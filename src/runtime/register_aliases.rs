use crate::runtime::vm::VM;

const REG_V0: u8 = 2;
const REG_V1: u8 = 3;
const REG_A0: u8 = 4;
const REG_A1: u8 = 5;
const REG_A2: u8 = 6;
const REG_A3: u8 = 7;
const REG_T0: u8 = 8;
const REG_T1: u8 = 9;
const REG_T2: u8 = 10;
const REG_T3: u8 = 11;
const REG_T4: u8 = 12;
const REG_T5: u8 = 13;
const REG_T6: u8 = 14;
const REG_T7: u8 = 15;
const REG_S0: u8 = 16;
const REG_S1: u8 = 17;
const REG_S2: u8 = 18;
const REG_S3: u8 = 19;
const REG_S4: u8 = 20;
const REG_S5: u8 = 21;
const REG_S6: u8 = 22;
const REG_S7: u8 = 23;
const REG_T8: u8 = 24;
const REG_T9: u8 = 25;
const REG_K0: u8 = 26;
const REG_K1: u8 = 27;
const REG_GP: u8 = 28;
const REG_SP: u8 = 29;
const REG_FP: u8 = 30;
const REG_RA: u8 = 31;

impl VM {
    pub fn get_gp(&self) -> u32 {
        self.get_register(28).unwrap()
    }

    pub fn set_gp(&mut self, value: u32) {
        self.set_register(28, value).unwrap();
    }

    pub fn get_sp(&self) -> u32 {
        self.get_register(29).unwrap()
    }

    pub fn set_sp(&mut self, value: u32) {
        self.set_register(29, value).unwrap();
    }

    pub fn get_fp(&self) -> u32 {
        self.get_register(30).unwrap()
    }

    pub fn set_fp(&mut self, value: u32) {
        self.set_register(30, value).unwrap();
    }

    pub fn get_ra(&self) -> u32 {
        self.get_register(31).unwrap()
    }

    pub fn set_ra(&mut self, value: u32) {
        self.set_register(31, value).unwrap();
    }
}
