//Matas Udris, Robertas Povedionok 4 grupe, informatika

use crate::virtual_machine::processor::VMProcessor;
use super::memory_management_unit::MemoryManagementUnit;
use crate::traits::Processor;
use crate::consts::*;
use crate::types::Word;
use std::io::stdin;
use std::collections::HashMap;


use std::{
    time::Duration,
};
use crossterm::{
    event::{poll, read, Event, KeyCode},
    Result,
};

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
    vm_list: HashMap<usize, VMProcessor>,
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
            vm_list: HashMap::new(),
            mmu: MemoryManagementUnit::new(),
        }
    }
    pub fn find_lowest_free_pid(&self) -> usize {
        let mut i: usize = 0;
        while self.vm_list.contains_key(&i) {
            i += 1;
        }
        i
    }
    pub fn add_program(&mut self, program_name: String, debug_mode: bool) {
        let ptr = self
            .mmu
            .load_program(program_name)
            .expect("Failed to load program");
        self.mmu.print_user_memory();
        self.mmu.print_virtual_memory_words(ptr);
        let key = self.find_lowest_free_pid();
        self.vm_list.insert(key, VMProcessor::new(ptr));
        self.vm_list.get_mut(&key).expect("Error getting mut vm").set_trap_flag(debug_mode);
    }
    pub fn process_interrupt(&mut self, process_id: usize) {
        match self.pi {
            0 => return,
            1 => println!("PROCESS {}> ERROR: DIVISION BY ZERO", process_id),
            4 => println!("PROCESS {}> ERROR: INVALID COMMAND", process_id),
            _ => println!("PROCESS {}> ERROR: oopsie", process_id),
        }
        self.process_halt(process_id);
    }

    pub fn get_command(&mut self) -> Word {
        let w = self.mmu.get_word(self.ptr, self.ip + (DATA_PAGES * PAGE_SIZE) as u32);
        self.ip += 1;
        println!("From page {} ip {} received {}",
            (self.ip as usize - 1) / PAGE_SIZE + DATA_PAGES,
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
            let mut key_list: Vec<usize> = Vec::new();
            for key in self.vm_list.keys() {
                key_list.push(*key);
            }
            for i in key_list {
                if !self.vm_list.contains_key(&i) {
                    continue;
                }
                self.process_command(i);
            }
            if self.vm_list.len() == 0 {
                println!("All programs have halted.");
                return;
            }
        }
    }
    fn print_registers(&self) {
        println!("ax: {}", self.ax);
        println!("bx: {}", self.bx);
        println!("cx: {}", self.cx);
        println!("dx: {}", self.dx);
        println!("ip: {}", self.ip);
        println!("ptr: {}", self.ptr);
        println!("sr: {:#032b}", self.sr);
    }
    fn process_trap_flag(&mut self, vm: usize) -> Result<()> {
        if self.get_trap_flag() {
            println!("Program {}> Trapped.", vm);
            println!("Program {} registers:", vm);
            self.print_registers();
    
            println!("Press U to print User memory.\nPress V to print Virtual memory\nPress Esc to continue.");
            loop {
                // Wait up to 1s for another event
                if poll(Duration::from_millis(1_000))? {
                    let event = read()?;
                    if event == Event::Key(KeyCode::Char('u').into()) {
                        self.mmu.print_user_memory();
                    }
                    if event == Event::Key(KeyCode::Char('v').into()) {
                        self.mmu.print_virtual_memory_words(self.ptr);
                    }
                    if event == Event::Key(KeyCode::Esc.into()) {
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn process_command(&mut self, vm: usize) {
        self.pi = 0;
        self.get_vars(vm);
        if self.process_trap_flag(vm).is_err() {
            panic!("");
        }
        let cmd: String = self.get_command().as_text().expect("Failed to get text");
        println!("program {}: Now processing command {}", vm, cmd);
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
            "LOAD" => self.process_load(),
          //"STOR" => self.process_stor(),
          //"OPEN" => self.process_open(),
          //"READ" => self.process_read(),
          //"WRT" => self.process_wrt(),
          //"CLS" => self.process_cls(),
          //"DEL" => self.process_del(),
          //"ACTV" => self.process_actv(),
          //"GTST" => self.process_gtst(),
          //"STST" => self.process_stst(),
            "HALT" =>  {
                self.process_halt(vm);
                return;
            }
          
            _ => { 
                println!("NOT IMPLEMENTED");
                self.pi = 4;
            }
        }
        self.process_interrupt(vm);
        self.set_vars(vm);
    }
      
    pub fn get_vars(&mut self, vm: usize) {
        self.ax = self.vm_list.get_mut(&vm).expect("Failed to get ax").get_ax();
        self.bx = self.vm_list.get_mut(&vm).expect("Failed to get bx").get_bx();
        self.cx = self.vm_list.get_mut(&vm).expect("Failed to get cx").get_cx();
        self.dx = self.vm_list.get_mut(&vm).expect("Failed to get dx").get_dx();
        self.sr = self.vm_list.get_mut(&vm).expect("Failed to get sr").get_sr();
        self.ptr = self.vm_list.get_mut(&vm).expect("Failed to get ptr").get_ptr();
        self.ip = self.vm_list.get_mut(&vm).expect("Failed to get ip").get_ic();
    }

    pub fn set_vars(&mut self, vm: usize) {
        if !self.vm_list.contains_key(&vm) {
            return;
        }
        self.vm_list.get_mut(&vm).expect("Failed to set ax").set_ax(self.ax);
        self.vm_list.get_mut(&vm).expect("Failed to set bx").set_bx(self.bx);
        self.vm_list.get_mut(&vm).expect("Failed to set cx").set_cx(self.cx);
        self.vm_list.get_mut(&vm).expect("Failed to set dx").set_dx(self.dx);
        self.vm_list.get_mut(&vm).expect("Failed to set ip").set_ic(self.ip);
        self.vm_list.get_mut(&vm).expect("Failed to set sr").set_sr(self.sr);
    }
}

impl RMProcessor {
    pub fn process_addr(&mut self) {
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let cmd_2: String = self.get_command().as_text().expect("Failed to get text");
        let c_2 = cmd_2.as_str();
        let val: u32;
        match c_2 {
            "REGA" => val = self.ax,
            "REGB" => val = self.bx,
            "REGC" => val = self.cx,
            "REGD" => val = self.dx,
            _ => { 
                println!("Register not found: {}", c_2);
                self.pi = 4;
                return;
            }
        }
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax += val,
            "REGB" => self.bx += val,
            "REGC" => self.cx += val,
            "REGD" => self.dx += val,
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
        }
    }

    pub fn process_addv(&mut self) {
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let val: u32 = self.get_command().as_u32();
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax += val,
            "REGB" => self.bx += val,
            "REGC" => self.cx += val,
            "REGD" => self.dx += val,
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
        }
        
    }

    pub fn process_subr(&mut self) {
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let cmd_2: String = self.get_command().as_text().expect("Failed to get text");
        let c_2 = cmd_2.as_str();
        let val: u32;
        match c_2 {
            "REGA" => val = self.ax,
            "REGB" => val = self.bx,
            "REGC" => val = self.cx,
            "REGD" => val = self.dx,
            _ => { 
                println!("Register not found: {}", c_2);
                self.pi = 4;
                return;
            }
        }
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax -= val,
            "REGB" => self.bx -= val,
            "REGC" => self.cx -= val,
            "REGD" => self.dx -= val,
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
        }

    }

    pub fn process_subv(&mut self) {
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let val: u32 = self.get_command().as_u32();
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax -= val,
            "REGB" => self.bx -= val,
            "REGC" => self.cx -= val,
            "REGD" => self.dx -= val,
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
        }
    }

    pub fn process_mulr(&mut self) {
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let cmd_2: String = self.get_command().as_text().expect("Failed to get text");
        let c_2 = cmd_2.as_str();
        let val: u32;
        match c_2 {
            "REGA" => val = self.ax,
            "REGB" => val = self.bx,
            "REGC" => val = self.cx,
            "REGD" => val = self.dx,
            _ => { 
                println!("Register not found: {}", c_2);
                self.pi = 4;
                return;
            }
        }
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax *= val,
            "REGB" => self.bx *= val,
            "REGC" => self.cx *= val,
            "REGD" => self.dx *= val,
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
        }
    }

    pub fn process_mulv(&mut self) {
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let val: u32 = self.get_command().as_u32();
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax *= val,
            "REGB" => self.bx *= val,
            "REGC" => self.cx *= val,
            "REGD" => self.dx *= val,
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
        }
    }

    pub fn process_divr(&mut self) {
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let cmd_2: String = self.get_command().as_text().expect("Failed to get text");
        let c_2 = cmd_2.as_str();
        let val: u32;
        match c_2 {
            "REGA" => val = self.ax,
            "REGB" => val = self.bx,
            "REGC" => val = self.cx,
            "REGD" => val = self.dx,
            _ => { 
                println!("Register not found: {}", c_2);
                self.pi = 4;
                return;
            }
        }
        if val != 0 {
            let c_1 = cmd_1.as_str();
            match c_1 {
                "REGA" => self.ax /= val,
                "REGB" => self.bx /= val,
                "REGC" => self.cx /= val,
                "REGD" => self.dx /= val,
                _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
            }
        }
        else {
            self.pi = 1;
        }
    }

    pub fn process_divv(&mut self) {
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let val: u32 = self.get_command().as_u32();
        if val != 0{
            let c_1 = cmd_1.as_str();
            match c_1 {
            "REGA" => self.ax /= val,
            "REGB" => self.bx /= val,
            "REGC" => self.cx /= val,
            "REGD" => self.dx /= val,
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
            }
        }
        else {
            self.pi = 1;
        }
    }

    pub fn process_andr(&mut self) {
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let cmd_2: String = self.get_command().as_text().expect("Failed to get text");
        let c_2 = cmd_2.as_str();
        let val: u32;
        match c_2 {
            "REGA" => val = self.ax,
            "REGB" => val = self.bx,
            "REGC" => val = self.cx,
            "REGD" => val = self.dx,
            _ => { 
                println!("Register not found: {}", c_2);
                self.pi = 4;
                return;
            }
        }
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax &= val,
            "REGB" => self.bx &= val,
            "REGC" => self.cx &= val,
            "REGD" => self.dx &= val,
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
        }
    }

    pub fn process_andv(&mut self) {
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let val: u32 = self.get_command().as_u32();
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax &= val,
            "REGB" => self.bx &= val,
            "REGC" => self.cx &= val,
            "REGD" => self.dx &= val,
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
        }
    }

    pub fn process_orr(&mut self) {
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let cmd_2: String = self.get_command().as_text().expect("Failed to get text");
        let c_2 = cmd_2.as_str();
        let val: u32;
        match c_2 {
            "REGA" => val = self.ax,
            "REGB" => val = self.bx,
            "REGC" => val = self.cx,
            "REGD" => val = self.dx,
            _ => { 
                println!("Register not found: {}", c_2);
                self.pi = 4;
                return;
            }
        }
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax |= val,
            "REGB" => self.bx |= val,
            "REGC" => self.cx |= val,
            "REGD" => self.dx |= val,
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
        }
    }

    pub fn process_orv(&mut self) {
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let val: u32 = self.get_command().as_u32();
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax |= val,
            "REGB" => self.bx |= val,
            "REGC" => self.cx |= val,
            "REGD" => self.dx |= val,
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
        }
    }

    pub fn process_xorr(&mut self) {
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let cmd_2: String = self.get_command().as_text().expect("Failed to get text");
        let c_2 = cmd_2.as_str();
        let val: u32;
        match c_2 {
            "REGA" => val = self.ax,
            "REGB" => val = self.bx,
            "REGC" => val = self.cx,
            "REGD" => val = self.dx,
            _ => { 
                println!("Register not found: {}", c_2);
                self.pi = 4;
                return;
            }
        }
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax ^= val,
            "REGB" => self.bx ^= val,
            "REGC" => self.cx ^= val,
            "REGD" => self.dx ^= val,
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
        }
    }

    pub fn process_xorv(&mut self) {
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let val: u32 = self.get_command().as_u32();
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax ^= val,
            "REGB" => self.bx ^= val,
            "REGC" => self.cx ^= val,
            "REGD" => self.dx ^= val,
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
        }
    }

    pub fn process_cmpr(&mut self) {
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let cmd_2: String = self.get_command().as_text().expect("Failed to get text");
        let c_2 = cmd_2.as_str();
        let val: u32;
        let val_2:u32;
        match c_2 {
            "REGA" => val_2 = self.ax,
            "REGB" => val_2 = self.bx,
            "REGC" => val_2 = self.cx,
            "REGD" => val_2 = self.dx,
            _ => { 
                println!("Register not found: {}", c_2);
                self.pi = 4;
                return;
            }
        }
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => val = self.ax,
            "REGB" => val = self.bx,
            "REGC" => val = self.cx,
            "REGD" => val = self.dx,
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
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
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let val_2: u32 = self.get_command().as_u32();
        let val: u32;
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => val = self.ax,
            "REGB" => val = self.bx,
            "REGC" => val = self.cx,
            "REGD" => val = self.dx,
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
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
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
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
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
        }
    }

    pub fn process_prtn(&mut self) {
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => println!("{}", self.ax),
            "REGB" => println!("{}", self.bx),
            "REGC" => println!("{}", self.cx),
            "REGD" => println!("{}", self.dx), 
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
        } //TODO 
    }
    pub fn process_getn(&mut self) {
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let c_1 = cmd_1.as_str();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let val: u32 = input.trim().parse().expect("Failed to parse n");
        match c_1 {
            "REGA" => self.ax = val,
            "REGB" => self.bx = val,
            "REGC" => self.cx = val,
            "REGD" => self.dx = val, 
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
        }
    }
    pub fn process_prts(&mut self) {
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let val: u32;
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => val = self.ax,
            "REGB" => val = self.bx,
            "REGC" => val = self.cx,
            "REGD" => val = self.dx, 
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
        }
        let word = Word::from_u32(val);
        let print: String = word.as_text().expect("Failed to get text");
        println!("{}",print)
    }

    pub fn process_gets(&mut self) {
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let c_1 = cmd_1.as_str();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let word = Word::from_string(input);
        match c_1 {
            "REGA" => self.ax = word.as_u32(),
            "REGB" => self.bx = word.as_u32(),
            "REGC" => self.cx = word.as_u32(),
            "REGD" => self.dx = word.as_u32(), 
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
        }
    }

    pub fn process_movr(&mut self) {
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let cmd_2: String = self.get_command().as_text().expect("Failed to get text");
        let c_2 = cmd_2.as_str();
        let val: u32;
        match c_2 {
            "REGA" => val = self.ax,
            "REGB" => val = self.bx,
            "REGC" => val = self.cx,
            "REGD" => val = self.dx,
            _ => { 
                println!("Register not found: {}", c_2);
                self.pi = 4;
                return;
            }
        }
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax = val,
            "REGB" => self.bx = val,
            "REGC" => self.cx = val,
            "REGD" => self.dx = val,
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
        }
    }

    pub fn process_movn(&mut self) {
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let val: u32 = self.get_command().as_u32();
        let c_1 = cmd_1.as_str();
        match c_1 {
            "REGA" => self.ax = val,
            "REGB" => self.bx = val,
            "REGC" => self.cx = val,
            "REGD" => self.dx = val,
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
        }
    }

    pub fn process_load(&mut self) {
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let cmd_2: String = self.get_command().as_text().expect("Failed to get text");
        let c_1 = cmd_1.as_str();
        let c_2 = cmd_2.as_str();
        let adr: u32;
        match c_2 {
            "REGA" => adr = self.ax,
            "REGB" => adr = self.bx,
            "REGC" => adr = self.cx,
            "REGD" => adr = self.dx,
            _ => { 
                println!("Register not found: {}", c_2);
                self.pi = 4;
                return;
            }
        }
        let w = self.mmu.get_word(self.ptr, adr).as_u32();
        match c_1 {
            "REGA" => self.ax = w,
            "REGB" => self.bx = w,
            "REGC" => self.cx = w,
            "REGD" => self.dx = w,
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
        }
    }

    pub fn process_stor(&mut self) {
        let cmd_1: String = self.get_command().as_text().expect("Failed to get text");
        let cmd_2: String = self.get_command().as_text().expect("Failed to get text");
        let c_1 = cmd_1.as_str();
        let c_2 = cmd_2.as_str();
        let val: u32;
        match c_1 {
            "REGA" => val = self.ax,
            "REGB" => val = self.bx,
            "REGC" => val = self.cx,
            "REGD" => val = self.dx,
            _ => { 
                println!("Register not found: {}", c_1);
                self.pi = 4;
                return;
            }
        }
        let w = self.mmu.store_word(self.ptr, val);
        match c_2 {
            "REGA" => self.ax = w,
            "REGB" => self.bx = w,
            "REGC" => self.cx = w,
            "REGD" => self.dx = w,
            _ => { 
                println!("Register not found: {}", c_2);
                self.pi = 4;
                return;
            }
        }
    }



    pub fn process_halt(&mut self, vm: usize) {
        self.vm_list.get_mut(&vm).expect("Failed to get mut vm").stop();
        println!("Deleting from {}", self.ptr);
        // self.mmu.print_virtual_memory(self.ptr);
        self.mmu.unload_program(self.ptr);
        // self.mmu.print_user_memory();
        self.vm_list.remove(&vm);
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