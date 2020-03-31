use crate::traits::Processor;

#[derive(Debug)]
pub struct Virtual_Processor {
    r1: u32,
    r2: u32,
    r3: u32,
    r4: u32,
    ic: u32,
    sr: u16,
    ptr: u32,
}

impl Virtual_Processor {
    // Create new instance with default values
    pub fn new() -> Virtual_Processor {
        Virtual_Processor {
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            ic: 0,
            sr: 0,
            ptr: 0,
        }
    }
}


impl Processor for Virtual_Processor{
    fn get_carry_flag(self) -> bool {
        self.sr & 0b0000_0000_0000_0001 > 0
    }
    fn get_parity_flag(self) -> bool {
        self.sr & 0b0000_0000_0000_0100 > 0
    }
    fn get_auxiliary_carry_flag(self) -> bool {
        self.sr & 0b0000_0000_0001_0000 > 0
    }
    fn get_zero_flag(self) -> bool {
        self.sr & 0b0000_0000_0100_0000 > 0
    }
    fn get_sign_flag(self) -> bool {
        self.sr & 0b0000_0000_1000_0000 > 0
    }
    fn get_trap_flag(self) -> bool {
        self.sr & 0b0000_0001_0000_0000 > 0
    }
    fn get_interrupt_flag(self) -> bool {
        self.sr & 0b0000_0010_0000_0000 > 0
    }
    fn get_directional_flag(self) -> bool {
        self.sr & 0b0000_0100_0000_0000 > 0
    }
    fn get_overflow_flag(self) -> bool {
        self.sr & 0b0000_1000_0000_0000 > 0
    }
    fn get_supervisor_flag(&self) -> bool {
        false
    }
} 