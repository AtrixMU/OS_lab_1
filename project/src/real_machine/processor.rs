use crate::traits::Processor;
// The processor struct for our real machine.
// Debug allows us to print the struct using println!("{:?}", struct_name)
#[derive(Debug)]
pub struct RMProcessor {
    ax: u32,
    bx: u32,
    cx: u32,
    dx: u32,
    pi: u8,
    ki: u8,
    ti: u8,
    sr: u16,
    ip: u32,
    ptr: u32,
}

// Methods implemented for the real machine Processor struct
impl RMProcessor {
    // Create new instance with default values
    pub fn new() -> RMProcessor {
        RMProcessor {
            ax: 0,
            bx: 0,
            cx: 0,
            dx: 0,
            pi: 0,
            ki: 0,
            ti: 0,
            sr: 0,
            ip: 0,
            ptr: 0,
        }
    }
}

impl Processor for RMProcessor {
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
        self.sr & 0b1000_0000_0000_0000 > 0
    }
}