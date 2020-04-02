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
use std::io::stdin;
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
    pub fn add_program(&mut self, program_name: String) {
        let ptr = self
            .mmu
            .load_program(program_name)
            .unwrap();
        self.mmu.print_user_memory();
        self.mmu.print_virtual_memory(ptr);
        self.vm_list.push(VMProcessor::new(ptr));
    }
    pub fn process_interrupt(self, process_id: u32) {
        match self.pi {
            0 => println!("all good"),
            _ => println!("oopsie"),
        }
    }

    pub fn get_command(&mut self) -> Word {
        let w = self.mmu.get_word(self.ptr, self.ip);
        self.ip += 1;
        println!("From page {} ip {} received {}",
            (self.ip as usize - 1) / PAGE_SIZE,
            self.ip as usize - 1,
            w.as_u32()
        );
        w
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
    pub fn run_instruction_loop(&mut self) {
        loop {
            let vm_len = self.vm_list.len();
            for i in 0..vm_len {
                self.process_command(i);
            }
            self.vm_list.retain(|e| !e.is_finished());
            if self.vm_list.len() == 0 {
                println!("All programs have halted.");
                return;
            }
        }
    }

    pub fn process_command(&mut self, vm: usize) {
        self.get_vars(vm);
        let cmd: String = self.get_command().as_text().unwrap();
        println!("Now processing command {} from program {}", cmd, vm);
        let c = cmd.as_str();
        match c {
            "ADDR" => self.process_addr(),
            "ADDV" => self.process_addv(),
            "SUBR" => self.process_subr(),
            "SUBV" => self.process_subv(),
            "MULR" => self.process_mulr(),
            "MULV" => self.process_mulv(),
            "DIVR" => self.process_divr(),
            "DIVV" => self.process_divv(),
            "ANDR" => self.process_andr(),
            "ANDV" => self.process_andv(),
             "ORR" => self.process_orr(),
             "ORV" => self.process_orv(),
            "XORR" => self.process_xorr(),
            "XORV" => self.process_xorv(),
            "CMPR" => self.process_cmpr(),
            "CMPV" => self.process_cmpv(),
            "JUMP" => self.process_jump(),
            "JPEQ" => self.process_jpeq(),
            "JPOF" => self.process_jpof(),
            "JPGE" => self.process_jpge(),
            "JPBE" => self.process_jpbe(),
            "JMPG" => self.process_jmpg(),
            "JMPB" => self.process_jmpb(),
            "LOOP" => self.process_loop(),
            "PRTN" => self.process_prtn(),
            "GETN" => self.process_getn(),
            "PRTS" => self.process_prts(),
            "GETS" => self.process_gets(),
            "MOVR" => self.process_movr(),
            "MOVN" => self.process_movn(),
          //"LOAD" => self.process_load(),
          //"STOR" => self.process_stor(),
          //"OPEN" => self.process_open(),
          //"READ" => self.process_read(),
          //"WRT" => self.process_wrt(),
          //"CLS" => self.process_cls(),
          //"DEL" => self.process_del(),
          //"ACTV" => self.process_actv(),
          //"GTST" => self.process_gtst(),
          //"STST" => self.process_stst(),
            "HALT" => self.process_halt(vm),
          
          _ => println!("NOT IMPLEMENTED"),
        }
        self.set_vars(vm);
    }
      
    pub fn get_vars(&mut self, vm: usize) {
        self.ax = self.vm_list[vm].get_ax();
        self.bx = self.vm_list[vm].get_bx();
        self.cx = self.vm_list[vm].get_cx();
        self.dx = self.vm_list[vm].get_dx();
        self.sr = self.vm_list[vm].get_sr();
        self.ptr = self.vm_list[vm].get_ptr();
        self.ip = self.vm_list[vm].get_ic();
    }

    pub fn set_vars(&mut self, vm: usize) {
        self.vm_list[vm].set_ax(self.ax);
        self.vm_list[vm].set_bx(self.bx);
        self.vm_list[vm].set_cx(self.cx);
        self.vm_list[vm].set_dx(self.dx);
        self.vm_list[vm].set_ic(self.ip);
        self.vm_list[vm].set_sr(self.sr);
    }
}

impl RMProcessor {
    pub fn process_addr(&mut self) {
        let cmd_1: String = self.get_command().as_text().unwrap();
        let cmd_2: String = self.get_command().as_text().unwrap();
        let c_2 = cmd_2.as_str();
        let val: u32;
        match c_2 {
            "REGA" => val = self.ax,
            "REGB" => val = self.bx,
            "REGC" => val = self.cx,
            "REGD" => val = self.dx,
            _ => panic!("Found: {}", c_2),
        }
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax += val,
            "REGB" => self.bx += val,
            "REGC" => self.cx += val,
            "REGD" => self.dx += val,
            _ => panic!("Found: {}", c_1),
        }
    }

    pub fn process_addv(&mut self) {
        let cmd_1: String = self.get_command().as_text().unwrap();
        let val: u32 = self.get_command().as_u32();
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax += val,
            "REGB" => self.bx += val,
            "REGC" => self.cx += val,
            "REGD" => self.dx += val,
            _ => panic!("Found: {}", c_1),
        }
        
    }

    pub fn process_subr(&mut self) {
        let cmd_1: String = self.get_command().as_text().unwrap();
        let cmd_2: String = self.get_command().as_text().unwrap();
        let c_2 = cmd_2.as_str();
        let val: u32;
        match c_2 {
            "REGA" => val = self.ax,
            "REGB" => val = self.bx,
            "REGC" => val = self.cx,
            "REGD" => val = self.dx,
            _ => panic!("Found: {}", c_2),
        }
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax -= val,
            "REGB" => self.bx -= val,
            "REGC" => self.cx -= val,
            "REGD" => self.dx -= val,
            _ => panic!("Found: {}", c_1),
        }

    }

    pub fn process_subv(&mut self) {
        let cmd_1: String = self.get_command().as_text().unwrap();
        let val: u32 = self.get_command().as_u32();
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax -= val,
            "REGB" => self.bx -= val,
            "REGC" => self.cx -= val,
            "REGD" => self.dx -= val,
            _ => panic!("Found: {}", c_1),
        }
    }

    pub fn process_mulr(&mut self) {
        let cmd_1: String = self.get_command().as_text().unwrap();
        let cmd_2: String = self.get_command().as_text().unwrap();
        let c_2 = cmd_2.as_str();
        let val: u32;
        match c_2 {
            "REGA" => val = self.ax,
            "REGB" => val = self.bx,
            "REGC" => val = self.cx,
            "REGD" => val = self.dx,
            _ => panic!("Found: {}", c_2),
        }
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax *= val,
            "REGB" => self.bx *= val,
            "REGC" => self.cx *= val,
            "REGD" => self.dx *= val,
            _ => panic!("Found: {}", c_1),
        }
    }

    pub fn process_mulv(&mut self) {
        let cmd_1: String = self.get_command().as_text().unwrap();
        let val: u32 = self.get_command().as_u32();
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax *= val,
            "REGB" => self.bx *= val,
            "REGC" => self.cx *= val,
            "REGD" => self.dx *= val,
            _ => panic!("Found: {}", c_1),
        }
    }

    pub fn process_divr(&mut self) {
        let cmd_1: String = self.get_command().as_text().unwrap();
        let cmd_2: String = self.get_command().as_text().unwrap();
        let c_2 = cmd_2.as_str();
        let val: u32;
        match c_2 {
            "REGA" => val = self.ax,
            "REGB" => val = self.bx,
            "REGC" => val = self.cx,
            "REGD" => val = self.dx,
            _ => panic!("Found: {}", c_2),
        }
        if val != 0{
            let c_1 = cmd_1.as_str();
            match c_1 {
                "REGA" => self.ax /= val,
                "REGB" => self.bx /= val,
                "REGC" => self.cx /= val,
                "REGD" => self.dx /= val,
                _ => panic!("Found: {}", c_1),
            }
        }
        else{
             //TODO
        }
    }

    pub fn process_divv(&mut self) {
        let cmd_1: String = self.get_command().as_text().unwrap();
        let val: u32 = self.get_command().as_u32();
        if val != 0{
            let c_1 = cmd_1.as_str();
            match c_1 {
            "REGA" => self.ax /= val,
            "REGB" => self.bx /= val,
            "REGC" => self.cx /= val,
            "REGD" => self.dx /= val,
            _ => panic!("Found: {}", c_1),
            }
        }
        else{
            //TODO
        }
    }

    pub fn process_andr(&mut self) {
        let cmd_1: String = self.get_command().as_text().unwrap();
        let cmd_2: String = self.get_command().as_text().unwrap();
        let c_2 = cmd_2.as_str();
        let val: u32;
        match c_2 {
            "REGA" => val = self.ax,
            "REGB" => val = self.bx,
            "REGC" => val = self.cx,
            "REGD" => val = self.dx,
            _ => panic!("Found: {}", c_2),
        }
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax &= val,
            "REGB" => self.bx &= val,
            "REGC" => self.cx &= val,
            "REGD" => self.dx &= val,
            _ => panic!("Found: {}", c_1),
        }
    }

    pub fn process_andv(&mut self) {
        let cmd_1: String = self.get_command().as_text().unwrap();
        let val: u32 = self.get_command().as_u32();
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax &= val,
            "REGB" => self.bx &= val,
            "REGC" => self.cx &= val,
            "REGD" => self.dx &= val,
            _ => panic!("Found: {}", c_1),
        }
    }

    pub fn process_orr(&mut self) {
        let cmd_1: String = self.get_command().as_text().unwrap();
        let cmd_2: String = self.get_command().as_text().unwrap();
        let c_2 = cmd_2.as_str();
        let val: u32;
        match c_2 {
            "REGA" => val = self.ax,
            "REGB" => val = self.bx,
            "REGC" => val = self.cx,
            "REGD" => val = self.dx,
            _ => panic!("Found: {}", c_2),
        }
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax |= val,
            "REGB" => self.bx |= val,
            "REGC" => self.cx |= val,
            "REGD" => self.dx |= val,
            _ => panic!("Found: {}", c_1),
        }
    }

    pub fn process_orv(&mut self) {
        let cmd_1: String = self.get_command().as_text().unwrap();
        let val: u32 = self.get_command().as_u32();
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax |= val,
            "REGB" => self.bx |= val,
            "REGC" => self.cx |= val,
            "REGD" => self.dx |= val,
            _ => panic!("Found: {}", c_1),
        }
    }

    pub fn process_xorr(&mut self) {
        let cmd_1: String = self.get_command().as_text().unwrap();
        let cmd_2: String = self.get_command().as_text().unwrap();
        let c_2 = cmd_2.as_str();
        let val: u32;
        match c_2 {
            "REGA" => val = self.ax,
            "REGB" => val = self.bx,
            "REGC" => val = self.cx,
            "REGD" => val = self.dx,
            _ => panic!("Found: {}", c_2),
        }
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax ^= val,
            "REGB" => self.bx ^= val,
            "REGC" => self.cx ^= val,
            "REGD" => self.dx ^= val,
            _ => panic!("Found: {}", c_1),
        }
    }

    pub fn process_xorv(&mut self) {
        let cmd_1: String = self.get_command().as_text().unwrap();
        let val: u32 = self.get_command().as_u32();
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax ^= val,
            "REGB" => self.bx ^= val,
            "REGC" => self.cx ^= val,
            "REGD" => self.dx ^= val,
            _ => panic!("Found: {}", c_1),
        }
    }

    pub fn process_cmpr(&mut self) {
        let cmd_1: String = self.get_command().as_text().unwrap();
        let cmd_2: String = self.get_command().as_text().unwrap();
        let c_2 = cmd_2.as_str();
        let val: u32;
        let val_2:u32;
        match c_2 {
            "REGA" => val_2 = self.ax,
            "REGB" => val_2 = self.bx,
            "REGC" => val_2 = self.cx,
            "REGD" => val_2 = self.dx,
            _ => panic!("Found: {}", c_2),
        }
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => val = self.ax,
            "REGB" => val = self.bx,
            "REGC" => val = self.cx,
            "REGD" => val = self.dx,
            _ => panic!("Found: {}", c_1),
        }
        match val.checked_sub(val_2) {
            Some(v) => {
                if v == 0{
                    self.set_zero_flag(true);
                    self.set_sign_flag(false);
                    self.set_overflow_flag(false);
                }
                else {
                    self.set_zero_flag(false);
                    self.set_sign_flag(false);
                    self.set_overflow_flag(false)
                } 
            }
            None => {
                self.set_zero_flag(false);
                self.set_sign_flag(true);
                self.set_overflow_flag(false)
            }

        }
    }
    
    pub fn process_cmpv(&mut self) {
        let cmd_1: String = self.get_command().as_text().unwrap();
        let val_2: u32 = self.get_command().as_u32();
        let val: u32;
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => val = self.ax,
            "REGB" => val = self.bx,
            "REGC" => val = self.cx,
            "REGD" => val = self.dx,
            _ => panic!(),
        }
        match val.checked_sub(val_2) {
            Some(v) => {
                if v == 0{
                    self.set_zero_flag(true);
                    self.set_sign_flag(false);
                    self.set_overflow_flag(false);
                }
                else {
                    self.set_zero_flag(false);
                    self.set_sign_flag(false);
                    self.set_overflow_flag(false)
                } 
            }
            None => {
                self.set_zero_flag(false);
                self.set_sign_flag(true);
                self.set_overflow_flag(false)
            }
        }
    }

    pub fn process_jump(&mut self) {
        let val: u32 = self.get_command().as_u32();
        self.ip = val;
    }

    pub fn process_jpeq(&mut self) {
        if self.get_zero_flag() {
            let val: u32 = self.get_command().as_u32();
            self.ip = val;
        }
    }

    pub fn process_jpof(&mut self) {
        if self.get_overflow_flag() {
            let val: u32 = self.get_command().as_u32();
            self.ip = val;
        }
    }

    pub fn process_jpge(&mut self) {
        if !self.get_sign_flag() {
            let val: u32 = self.get_command().as_u32();
            self.ip = val;
        }
    }
    pub fn process_jpbe(&mut self) {
        if self.get_sign_flag() {
            let val: u32 = self.get_command().as_u32();
            self.ip = val;
        }
    }
    pub fn process_jmpg(&mut self) {
        if !self.get_zero_flag() && !self.get_sign_flag() {
            let val: u32 = self.get_command().as_u32();
            self.ip = val;
        }
    }
    pub fn process_jmpb(&mut self) {
        if !self.get_zero_flag() && self.get_sign_flag() {
            let val: u32 = self.get_command().as_u32();
            self.ip = val;
        }
    }

    pub fn process_loop(&mut self) {
        let cmd_1: String = self.get_command().as_text().unwrap();
        let val: u32 = self.get_command().as_u32();
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => {
                if self.ax != 0 {
                    self.ax -= 1;
                    self.ip = val;
                }
        },
            "REGB" => {
                if self.bx != 0 {
                    self.bx -= 1;
                    self.ip = val;
                }
        },
            "REGC" => {
                if self.cx != 0 {
                    self.cx -= 1;
                    self.ip = val;
                 }
        },
            "REGD" => {
                if self.dx != 0 {
                    self.dx -= 1;
                    self.ip = val;
            }
        },
            _ => panic!("Found: {}", c_1),
        }
    }

    pub fn process_prtn(&mut self) {
        let cmd_1: String = self.get_command().as_text().unwrap();
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => println!("{}", self.ax),
            "REGB" => println!("{}", self.bx),
            "REGC" => println!("{}", self.cx),
            "REGD" => println!("{}", self.dx), 
            _ => panic!("Found: {}", c_1),
        } //TODO 
    }
    pub fn process_getn(&mut self) {
        let cmd_1: String = self.get_command().as_text().unwrap();
        let c_1 = cmd_1.as_str();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let val: u32 = input.trim().parse().unwrap();
        match c_1 {
            "REGA" => self.ax = val,
            "REGB" => self.bx = val,
            "REGC" => self.cx = val,
            "REGD" => self.dx = val, 
            _ => panic!("Found: {}", c_1),
        }
    }
    pub fn process_prts(&mut self) {
        let cmd_1: String = self.get_command().as_text().unwrap();
        let val: u32;
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => val = self.ax,
            "REGB" => val = self.bx,
            "REGC" => val = self.cx,
            "REGD" => val = self.dx, 
            _ => panic!("Found: {}", c_1),
        }
        let word = Word::from_u32(val);
        let print: String = word.as_text().unwrap();
        println!("{}",print)
    }

    pub fn process_gets(&mut self) {
        let cmd_1: String = self.get_command().as_text().unwrap();
        let c_1 = cmd_1.as_str();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let word = Word::from_string(input);
        match c_1 {
            "REGA" => self.ax = word.as_u32(),
            "REGB" => self.bx = word.as_u32(),
            "REGC" => self.cx = word.as_u32(),
            "REGD" => self.dx = word.as_u32(), 
            _ => panic!("Found: {}", c_1),
        }
    }

    pub fn process_movr(&mut self) {
        let cmd_1: String = self.get_command().as_text().unwrap();
        let cmd_2: String = self.get_command().as_text().unwrap();
        let c_2 = cmd_2.as_str();
        let val: u32;
        match c_2 {
            "REGA" => val = self.ax,
            "REGB" => val = self.bx,
            "REGC" => val = self.cx,
            "REGD" => val = self.dx,
            _ => panic!("Found: {}", c_2),
        }
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax = val,
            "REGB" => self.bx = val,
            "REGC" => self.cx = val,
            "REGD" => self.dx = val,
            _ => panic!("Found: {}", c_1),
        }
    }

    pub fn process_movn(&mut self) {
        let cmd_1: String = self.get_command().as_text().unwrap();
        let val: u32 = self.get_command().as_u32();
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax = val,
            "REGB" => self.bx = val,
            "REGC" => self.cx = val,
            "REGD" => self.dx = val,
            _ => panic!("Found: {}", c_1),
        }
    }

    pub fn process_halt(&mut self, vm: usize) {
        self.vm_list[vm].stop();
        self.mmu.unload_program(self.ptr);
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