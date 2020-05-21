//Matas Udris, Robertas Povedionok 4 grupe, informatika
use crate::traits::Processor;
use crate::consts::*;

#[derive(Debug)]
pub struct VMProcessor {
    ax: u32,
    bx: u32,
    cx: u32,
    dx: u32,
    ic: u32,
    sr: u16,
    ptr: u32,
    is_finished: bool,
}

impl VMProcessor {
    // Create new instance with default values
    pub fn new(ptr: u32) -> VMProcessor {
        VMProcessor {
            ax: 0,
            bx: 0,
            cx: 0,
            dx: 0,
            ic: 0,
            sr: 0,
            ptr: ptr,
            is_finished: false,
        }
    }
    pub fn is_finished(&self) -> bool {
        self.is_finished
    }
}

impl Processor for VMProcessor{
    fn get_carry_flag(&self) -> bool {
        self.sr & CARRY_FLAG > 0
    }
    fn get_parity_flag(&self) -> bool {
        self.sr & PARITY_FLAG > 0
    }
    fn get_auxiliary_carry_flag(&self) -> bool {
        self.sr & AUXILIARY_CARRY_FLAG > 0
    }
    fn get_zero_flag(&self) -> bool {
        self.sr & ZERO_FLAG > 0
    }
    fn get_sign_flag(&self) -> bool {
        self.sr & SIGN_FLAG > 0
    }
    fn get_trap_flag(&self) -> bool {
        self.sr & TRAP_FLAG > 0
    }
    fn get_interrupt_flag(&self) -> bool {
        self.sr & INTERRUPT_FLAG > 0
    }
    fn get_directional_flag(&self) -> bool {
        self.sr & DIRECTIONAL_FLAG > 0
    }
    fn get_overflow_flag(&self) -> bool {
        self.sr & OVERFLOW_FLAG > 0
    }
    fn get_supervisor_flag(&self) -> bool {
        false
    }
    fn set_carry_flag(&mut self, value: bool) {
        if value {
            self.sr |= CARRY_FLAG;
        }
        else {
            self.sr &= !CARRY_FLAG;
        }
    }
    fn set_parity_flag(&mut self, value: bool) {
        if value {
            self.sr |= PARITY_FLAG;
        }
        else {
            self.sr &= PARITY_FLAG;
        }
    }
    fn set_auxiliary_carry_flag(&mut self, value: bool) {
        if value {
            self.sr |= AUXILIARY_CARRY_FLAG;
        }
        else {
            self.sr &= !AUXILIARY_CARRY_FLAG;
        }
    }
    fn set_zero_flag(&mut self, value: bool) {
        if value {
            self.sr |= ZERO_FLAG;
        }
        else {
            self.sr &= !ZERO_FLAG;
        }
    }
    fn set_sign_flag(&mut self, value: bool) {
        if value {
            self.sr |= SIGN_FLAG;
        }
        else {
            self.sr &= !SIGN_FLAG;
        }
    }
    fn set_trap_flag(&mut self, value: bool) {
        if value {
            self.sr |= TRAP_FLAG;
        }
        else {
            self.sr &= !TRAP_FLAG;
        }
    }
    fn set_interrupt_flag(&mut self, value: bool) {
        if value {
            self.sr |= INTERRUPT_FLAG;
        }
        else {
            self.sr &= !INTERRUPT_FLAG;
        }
    }
    fn set_directional_flag(&mut self, value: bool) {
        if value {
            self.sr |= DIRECTIONAL_FLAG;
        }
        else {
            self.sr &= !DIRECTIONAL_FLAG;
        }
    }
    fn set_overflow_flag(&mut self, value: bool) {
        if value {
            self.sr |= OVERFLOW_FLAG;
        }
        else {
            self.sr &= !OVERFLOW_FLAG;
        }
    }
    fn set_supervisor_flag(&mut self, _value: bool) {}
}


impl VMProcessor{
    pub fn get_ax(&self) -> u32
    {
        self.ax
    }
    
    pub fn get_bx(&self) -> u32
    {
        self.bx
    } 
    
    pub fn get_cx(&self) -> u32
    {
        self.cx
    } 
    
    pub fn get_dx(&self) -> u32
    {
        self.dx
    }

    pub fn get_ic(&self) -> u32
    {
        self.ic
    } 

    pub fn get_sr(&self) -> u16
    {
        self.sr
    }

    pub fn get_ptr(&self) -> u32
    {
        self.ptr
    }

    pub fn set_ax(&mut self,ax: u32)
    {
        self.ax = ax;
    }
    pub fn set_bx(&mut self,bx: u32)
    {
        self.bx = bx;
    }
    pub fn set_cx(&mut self,cx: u32)
    {
        self.cx = cx;
    }
    pub fn set_dx(&mut self,dx: u32)
    {
        self.dx = dx;
    }
    pub fn set_ic(&mut self,ic: u32)
    {
        self.ic = ic;
    }
    pub fn set_sr(&mut self,sr: u16)
    {
        self.sr = sr;
    }
    pub fn set_ptr(&mut self,ptr: u32)
    {
        self.ptr = ptr;
    }

    pub fn stop(&mut self)
    {
        self.is_finished=true;
    }
}



#[cfg(test)]
mod processor_tests {
    use crate::virtual_machine::processor::VMProcessor;
    use crate::traits::Processor;
    #[test]
    pub fn test_carry_flag_true() {
        let mut cpu = VMProcessor::new(0);
        cpu.set_carry_flag(true);
        assert_eq!(cpu.get_carry_flag(), true);
    }
    #[test]
    pub fn test_carry_flag_false() {
        let mut cpu = VMProcessor::new(0);
        cpu.set_carry_flag(false);
        assert_eq!(cpu.get_carry_flag(), false);
    }
    #[test]
    pub fn test_parity_flag_true() {
        let mut cpu = VMProcessor::new(0);
        cpu.set_parity_flag(true);
        assert_eq!(cpu.get_parity_flag(), true);
    }
    #[test]
    pub fn test_parity_flag_false() {
        let mut cpu = VMProcessor::new(0);
        cpu.set_parity_flag(false);
        assert_eq!(cpu.get_parity_flag(), false);
    }
    #[test]
    pub fn test_auxiliary_carry_flag_true() {
        let mut cpu = VMProcessor::new(0);
        cpu.set_auxiliary_carry_flag(true);
        assert_eq!(cpu.get_auxiliary_carry_flag(), true);
    }
    #[test]
    pub fn test_auxiliary_carry_flag_false() {
        let mut cpu = VMProcessor::new(0);
        cpu.set_auxiliary_carry_flag(false);
        assert_eq!(cpu.get_auxiliary_carry_flag(), false);
    }
    #[test]
    pub fn test_zero_flag_true() {
        let mut cpu = VMProcessor::new(0);
        cpu.set_zero_flag(true);
        assert_eq!(cpu.get_zero_flag(), true);
    }
    #[test]
    pub fn test_zero_flag_false() {
        let mut cpu = VMProcessor::new(0);
        cpu.set_zero_flag(false);
        assert_eq!(cpu.get_zero_flag(), false);
    }
    #[test]
    pub fn test_sign_flag_true() {
        let mut cpu = VMProcessor::new(0);
        cpu.set_sign_flag(true);
        assert_eq!(cpu.get_sign_flag(), true);
    }
    #[test]
    pub fn test_sign_flag_false() {
        let mut cpu = VMProcessor::new(0);
        cpu.set_sign_flag(false);
        assert_eq!(cpu.get_sign_flag(), false);
    }
    #[test]
    pub fn test_trap_flag_true() {
        let mut cpu = VMProcessor::new(0);
        cpu.set_trap_flag(true);
        assert_eq!(cpu.get_trap_flag(), true);
    }
    #[test]
    pub fn test_trap_flag_false() {
        let mut cpu = VMProcessor::new(0);
        cpu.set_trap_flag(false);
        assert_eq!(cpu.get_trap_flag(), false);
    }
    #[test]
    pub fn test_interrupt_flag_true() {
        let mut cpu = VMProcessor::new(0);
        cpu.set_interrupt_flag(true);
        assert_eq!(cpu.get_interrupt_flag(), true);
    }
    #[test]
    pub fn test_interrupt_flag_false() {
        let mut cpu = VMProcessor::new(0);
        cpu.set_interrupt_flag(false);
        assert_eq!(cpu.get_interrupt_flag(), false);
    }
    #[test]
    pub fn test_directional_flag_true() {
        let mut cpu = VMProcessor::new(0);
        cpu.set_directional_flag(true);
        assert_eq!(cpu.get_directional_flag(), true);
    }
    #[test]
    pub fn test_directional_flag_false() {
        let mut cpu = VMProcessor::new(0);
        cpu.set_directional_flag(false);
        assert_eq!(cpu.get_directional_flag(), false);
    }
    #[test]
    pub fn test_overflow_flag_true() {
        let mut cpu = VMProcessor::new(0);
        cpu.set_overflow_flag(true);
        assert_eq!(cpu.get_overflow_flag(), true);
    }
    #[test]
    pub fn test_overflow_flag_false() {
        let mut cpu = VMProcessor::new(0);
        cpu.set_overflow_flag(false);
        assert_eq!(cpu.get_overflow_flag(), false);
    }
    #[test]
    pub fn test_supervisor_flag_true() {
        let mut cpu = VMProcessor::new(0);
        cpu.set_supervisor_flag(true);
        assert_eq!(cpu.get_supervisor_flag(), false);
    }
    #[test]
    pub fn test_supervisor_flag_false() {
        let mut cpu = VMProcessor::new(0);
        cpu.set_supervisor_flag(false);
        assert_eq!(cpu.get_supervisor_flag(), false);
    }
}