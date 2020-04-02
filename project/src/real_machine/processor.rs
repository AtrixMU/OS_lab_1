#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]

use crate::virtual_machine::processor::VMProcessor;
use super::memory_management_unit::MemoryManagementUnit;
use crate::traits::Processor;
use crate::consts::*;
use crate::types::Word;
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
    vm_list: Vec<VMProcessor>,
    mmu: MemoryManagementUnit,
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
            vm_list: Vec::new(),
            mmu: MemoryManagementUnit::new(),
        }
    }
    pub fn process_interrupt(self, process_id: u32) {
        match self.pi {
            0 => println!("oopsie"),
            _ => println!("all good"),
        }
    }

    pub fn get_command() -> Word {
        self.mmu.get_word(self.ptr, self.ip)
      }
}

impl Processor for RMProcessor {
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
        self.sr & SUPERVISOR_FLAG > 0
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
    fn set_supervisor_flag(&mut self, value: bool) {
        if value {
            self.sr |= SUPERVISOR_FLAG;
        }
        else {
            self.sr &= !SUPERVISOR_FLAG;
        }
    }
}

impl RMProcessor{
    pub fn instruction_loop(&mut self) {
        while true {
          for vm in self.vm_list.iter_mut() {
            self.process_command(&mut vm);
            }
        }
    }

    pub fn process_command(&mut self, vm: &mut VMProcessor) {
        self.set_vars(vm);
        let cmd: &str= self.get_command().as_text(); // not implemented yet, vyks per mmu
        match cmd {
          "ADDR" => self.process_addr(vm),
          //"ADDV" => self.process_addv(vm),
          //"SUBR" => self.process_subr(vm),
          //"SUBV" => self.process_subv(vm),
          //"MULR" => self.process_mulr(vm),
          //"MULV" => self.process_mulv(vm),
          //"DIVR" => self.process_divr(vm),
          //"DIVV" => self.process_divv(vm),
          //"ANDR" => self.process_andr(vm),
          //"ANDV" => self.process_andv(vm),
          //"ORR" => self.process_orr(vm),
          //"ORV" => self.process_orv(vm),
          //"XORR" => self.process_xorr(vm),
          //"XORV" => self.process_xorv(vm),
          //"CMPR" => self.process_cmpr(vm),
          //"CMPV" => self.process_cmpv(vm),
          //"JUMP" => self.process_jump(vm),
          //"JPEQ" => self.process_jpeq(vm),
          //"JPOF" => self.process_jpof(vm),
          //"JPGE" => self.process_jpge(vm),
          //"JPBE" => self.process_jpbe(vm),
          //"JMPG" => self.process_jmpg(vm),
          //"JMPB" => self.process_jmpb(vm),
          //"LOOP" => self.process_loop(vm),
          //"PRTN" => self.process_prtn(vm),
          //"GETN" => self.process_getn(vm),
          //"PRTS" => self.process_prts(vm),
          //"GETS" => self.process_gets(vm),
          //"MOVR" => self.process_movr(vm),
          //"MOVN" => self.process_movn(vm),
          //"LOAD" => self.process_load(vm),
          //"STOR" => self.process_stor(vm),
          //"OPEN" => self.process_open(vm),
          //"READ" => self.process_read(vm),
          //"WRT" => self.process_wrt(vm),
          //"CLS" => self.process_cls(vm),
          //"DEL" => self.process_del(vm),
          //"ACTV" => self.process_actv(vm),
          //"GTST" => self.process_gtst(vm),
          //"STST" => self.process_stst(vm),
          //"HALT" => self.process_halt(vm),
          
          _ => println!("NOT IMPLEMENTED"),
        }
    }
      
    pub fn get_vars(&mut self, vm: &VMProcessor) {
        self.ax = vm.get_ax();
        self.bx = vm.get_bx();
        self.cx = vm.get_cx();
        self.dx = vm.get_dx();
        self.sr = vm.get_sr();
        self.ptr = vm.get_ptr();
        self.ip = vm.get_ic();
    }

    pub fn set_vars(self, vm: &mut VMProcessor) {
        vm.set_ax(self.ax);
        vm.set_bx(self.bx);
        vm.set_cx(self.cx);
        vm.set_dx(self.dx);
        vm.set_ic(self.ip);
        vm.set_sr(self.sr);
        vm.set_ptr(self.ptr);
      }
      


}

impl RMProcessor{

    pub fn process_addr(&mut self,vm: &VMProcessor){
        self.get_vars(vm);
        self.ip+=self.ip+1;
        self.ptr+=self.ptr+1;
        let cmd: &str=self.get_command().as_text();
        let reg;
        match cmd {
            "REGA" => reg = &mut self.ax,
            "REGB" => reg = &mut self.bx,
            "REGC" => reg = &mut self.cx,
            "REGD" => reg = &mut self.dx,
        }
        self.ip+=self.ip+1;
        self.ptr+=self.ptr+1;
        let cmd: &str =self.get_command().as_text();
        match cmd{
            "REGA" => *reg += self.ax,
            "REGB" => *reg += self.bx,
            "REGC" => *reg += self.cx,
            "REGD" => *reg += self.dx,
        }
        self.ip+=self.ip+1;
        self.ptr+=self.ptr+1;

    }
}






#[cfg(test)]
mod processor_tests {
    use crate::real_machine::processor::RMProcessor;
    use crate::traits::Processor;
    #[test]
    pub fn test_carry_flag_true() {
        let mut cpu = RMProcessor::new();
        cpu.set_carry_flag(true);
        assert_eq!(cpu.get_carry_flag(), true);
    }
    #[test]
    pub fn test_carry_flag_false() {
        let mut cpu = RMProcessor::new();
        cpu.set_carry_flag(false);
        assert_eq!(cpu.get_carry_flag(), false);
    }
    #[test]
    pub fn test_parity_flag_true() {
        let mut cpu = RMProcessor::new();
        cpu.set_parity_flag(true);
        assert_eq!(cpu.get_parity_flag(), true);
    }
    #[test]
    pub fn test_parity_flag_false() {
        let mut cpu = RMProcessor::new();
        cpu.set_parity_flag(false);
        assert_eq!(cpu.get_parity_flag(), false);
    }
    #[test]
    pub fn test_auxiliary_carry_flag_true() {
        let mut cpu = RMProcessor::new();
        cpu.set_auxiliary_carry_flag(true);
        assert_eq!(cpu.get_auxiliary_carry_flag(), true);
    }
    #[test]
    pub fn test_auxiliary_carry_flag_false() {
        let mut cpu = RMProcessor::new();
        cpu.set_auxiliary_carry_flag(false);
        assert_eq!(cpu.get_auxiliary_carry_flag(), false);
    }
    #[test]
    pub fn test_zero_flag_true() {
        let mut cpu = RMProcessor::new();
        cpu.set_zero_flag(true);
        assert_eq!(cpu.get_zero_flag(), true);
    }
    #[test]
    pub fn test_zero_flag_false() {
        let mut cpu = RMProcessor::new();
        cpu.set_zero_flag(false);
        assert_eq!(cpu.get_zero_flag(), false);
    }
    #[test]
    pub fn test_sign_flag_true() {
        let mut cpu = RMProcessor::new();
        cpu.set_sign_flag(true);
        assert_eq!(cpu.get_sign_flag(), true);
    }
    #[test]
    pub fn test_sign_flag_false() {
        let mut cpu = RMProcessor::new();
        cpu.set_sign_flag(false);
        assert_eq!(cpu.get_sign_flag(), false);
    }
    #[test]
    pub fn test_trap_flag_true() {
        let mut cpu = RMProcessor::new();
        cpu.set_trap_flag(true);
        assert_eq!(cpu.get_trap_flag(), true);
    }
    #[test]
    pub fn test_trap_flag_false() {
        let mut cpu = RMProcessor::new();
        cpu.set_trap_flag(false);
        assert_eq!(cpu.get_trap_flag(), false);
    }
    #[test]
    pub fn test_interrupt_flag_true() {
        let mut cpu = RMProcessor::new();
        cpu.set_interrupt_flag(true);
        assert_eq!(cpu.get_interrupt_flag(), true);
    }
    #[test]
    pub fn test_interrupt_flag_false() {
        let mut cpu = RMProcessor::new();
        cpu.set_interrupt_flag(false);
        assert_eq!(cpu.get_interrupt_flag(), false);
    }
    #[test]
    pub fn test_directional_flag_true() {
        let mut cpu = RMProcessor::new();
        cpu.set_directional_flag(true);
        assert_eq!(cpu.get_directional_flag(), true);
    }
    #[test]
    pub fn test_directional_flag_false() {
        let mut cpu = RMProcessor::new();
        cpu.set_directional_flag(false);
        assert_eq!(cpu.get_directional_flag(), false);
    }
    #[test]
    pub fn test_overflow_flag_true() {
        let mut cpu = RMProcessor::new();
        cpu.set_overflow_flag(true);
        assert_eq!(cpu.get_overflow_flag(), true);
    }
    #[test]
    pub fn test_overflow_flag_false() {
        let mut cpu = RMProcessor::new();
        cpu.set_overflow_flag(false);
        assert_eq!(cpu.get_overflow_flag(), false);
    }
    #[test]
    pub fn test_supervisor_flag_true() {
        let mut cpu = RMProcessor::new();
        cpu.set_supervisor_flag(true);
        assert_eq!(cpu.get_supervisor_flag(), true);
    }
    #[test]
    pub fn test_supervisor_flag_false() {
        let mut cpu = RMProcessor::new();
        cpu.set_supervisor_flag(false);
        assert_eq!(cpu.get_supervisor_flag(), false);
    }
}