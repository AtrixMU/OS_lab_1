use super::process::Process;
use crate::real_machine::processor::RMProcessor;
use crate::virtual_machine::processor::VMProcessor;
use crate::consts::*;
use super::resource::Resource;
use super::vm::VM;


pub struct JobGovernor {
    id: usize,
    parent_id: usize,
    vm: usize,
    state: usize,
    section: usize,
    resources: Vec<Resource>,
    ptr: u32,
    int_code: u8,
}


impl JobGovernor {
    pub fn new(id: usize, parent_id: usize, vm: usize) -> JobGovernor {
        JobGovernor {
            id: id,
            parent_id: parent_id,
            vm: vm,
            state: P_READY,
            section: 0,
            resources: Vec::new(),
            ptr: 0,
            int_code: 0,
        }
    }
    fn get_msg(&self, resource_type: usize) -> String {
        for res in &self.resources {
            if res.get_type() == resource_type {
                return res.get_msg();
            }
        }
        panic!()
    }
}

impl Process for JobGovernor {
    fn get_state(&self) -> usize {
        self.state
    }
    fn set_state(&mut self, state: usize) {
        self.state = state;
    }
    fn get_vm(&self) -> usize {
        self.vm
    }
    fn set_vm(&mut self, vm: usize) {
        self.vm = vm;
    }
    fn add_resource(&mut self, res: Resource) {
        self.resources.push(res);
    }
    fn take_resource(&mut self, resource_type: usize) -> Resource {
        let mut resource_index = self.resources.len();
        for (index, res) in self.resources.iter().enumerate() {
            if res.get_type() == resource_type {
                resource_index = index;
                break;
            }
        }
        if resource_index == self.resources.len() {
            panic!();
        }
        self.resources.remove(resource_index)
    }
    fn has_resource(&self, resource_type: usize) -> bool {
        for res in &self.resources {
            if res.get_type() == resource_type {
                return true;
            }
        }
        false
    }
    fn step(&mut self, rm: &mut RMProcessor) -> (Option<usize>, Option<Resource>, Option<Box<dyn Process>>, Option<usize>) {
        match self.section {
            0 => {
                if self.has_resource(RES_TASK_IN_USER) {
                    self.section += 1;
                    self.state = P_READY;
                    (None, None, None, None)
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_TASK_IN_USER), None, None, None);
                }
            }
            1 => {
                if self.has_resource(RES_U_MEM) {
                    self.section += 1;
                    self.state = P_READY;
                    (None, None, None, None)
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_U_MEM), None, None, None);
                }
            },
            2 => {
                let msg = self.get_msg(RES_TASK_IN_USER);
                let params: Vec<&str> = msg.split_whitespace().collect();
                self.ptr = params[0].parse::<u32>().unwrap();
                let new_proc = VM::new(self.id + 10, self.id, 0);
                rm.vm_list.insert(self.id + 10, VMProcessor::new(self.ptr));
                self.section += 1;
                return (None, None, Some(Box::new(new_proc)), None);
            },
            3 => {
                if self.has_resource(RES_INTERRUPT) {
                    self.section += 1;
                    self.state = P_READY;
                    return (Some(P_READY_SUSP), None, None, Some(self.id + 10));
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_INTERRUPT), None, None, None);
                }
            },
            4 => {
                let msg = self.get_msg(RES_INTERRUPT);
                let params: Vec<&str> = msg.split_whitespace().collect();
                self.int_code = params[0].parse::<u8>().unwrap();
                if self.int_code == INT_HALT {
                    self.section = 5;
                    return (None, None, None, None);
                }
                if [INT_OPEN, INT_WRITE, INT_CLOSE, INT_READ, INT_DEL].contains(&self.int_code) {
                    self.section = 8;
                    return (None, None, None, None);
                }
                if [INT_GETN, INT_GETS].contains(&self.int_code) {
                    self.section = 11;
                    return (None, None, None, None);
                }
                if [INT_PRTN, INT_PRTS].contains(&self.int_code) {
                    self.section = 13;
                    return (None, None, None, None);
                }
                return (None, None, None, None);
            },
            5 => {
                self.section = 6;
                return (None, None, None, Some(self.id + 10));
            }
            6 => {
                rm.mmu.unload_program(self.ptr);
                self.section = 7;
                return (None, Some(self.take_resource(RES_U_MEM)), None, None);
            },
            7 => {
                let mut res = self.take_resource(RES_TASK_IN_USER);
                res.set_msg(format!("{} {} {}", self.ptr, 0, self.id));
                return(None, Some(res), None, None);
            },
            8 => {
                let mut res = Resource::new(RES_FILE_PACK);
                res.set_msg(format!("{} {}", self.int_code, self.id));
                self.section = 9;
                return (None, Some(res), None, None);
            },
            9 => {
                if self.has_resource(RES_FROM_FILEWORK) {
                    self.section = 10;
                    self.state = P_READY;
                    (None, None, None, None)
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_FROM_FILEWORK), None, None, None);
                }
            },
            10 => {
                let msg = self.get_msg(RES_FROM_FILEWORK);
                if msg.contains("Filework error") {
                    let mut res = Resource::new(RES_LINE_IN_MEM);
                    res.set_msg("eERROR: Filework Error!".to_string());
                    res.set_recipient(PID_PRINT_LINE);
                    self.section = 5;
                    return(None, Some(res), None, None);
                }
                self.take_resource(RES_FROM_FILEWORK);
                self.take_resource(RES_FROM_INTERRUPT);
                self.section = 3;
                return (Some(P_READY), None, None, Some(self.id + 10));
            },
            11 => {
                let mut res = Resource::new(RES_USER_INPUT);
                res.set_msg(format!("{} {}", self.id, self.int_code));
                self.section = 12;
                return (None, Some(res), None, None);
            }
            12 => {
                if self.has_resource(RES_FROM_USER_INT) {
                    self.state = P_READY;
                    self.take_resource(RES_FROM_USER_INT);
                    self.take_resource(RES_FROM_INTERRUPT);
                    self.section = 3;
                    (None, None, None, None)
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_FROM_USER_INT), None, None, None);
                }
            },
            13 => {
                let mut res = Resource::new(RES_LINE_IN_MEM);
                if self.int_code == INT_PRTN {
                    res.set_msg(format!("n{}", self.id));
                }
                else {
                    res.set_msg(format!("s{}", self.id));
                }
                self.section = 3;
                self.take_resource(RES_FROM_INTERRUPT);
                return (None, Some(res), None, None);
            },
            _ => panic!(),
        }
    }
    fn print(&self, rm: &RMProcessor) {
        println!("Process: JobGovernor");
        print!("Status: ");
        match self.state {
            P_READY => println!("P_READY"),
            P_RUNNING => println!("P_RUNNING"),
            P_BLOCKED => println!("P_BLOCKED"),
            P_READY_SUSP => println!("P_READY_SUSP"),
            P_BLOCKED_SUSP => println!("P_BLOCKED_SUSP"),
            _ => println!("INVALID STATE"),
        }
        println!("Section: {}", self.section);
        for resource in &self.resources {
            print!("Resource: ");
            match resource.get_type() {
                RES_S_MEM => println!("RES_S_MEM"),
                RES_U_MEM => println!("RES_U_MEM"),
                RES_DISK => println!("RES_DISK"),
                RES_CHNL => println!("RES_CHNL"),
                RES_TASK_IN_SUPER => println!("RES_TASK_IN_SUPER"),
                RES_FROM_USER_INT => println!("RES_FROM_USER_INT"),
                RES_FILE_PACK => println!("RES_FILE_PACK"),
                RES_USER_INPUT => println!("RES_USER_INPUT"),
                RES_LINE_IN_MEM => println!("RES_LINE_IN_MEM"),
                RES_FROM_FILEWORK => println!("RES_FROM_FILEWORK"),
                RES_INTERRUPT => println!("RES_INTERRUPT"),
                RES_FROM_INTERRUPT => println!("RES_FROM_INTERRUPT"),
                RES_THEAD_SUPER => println!("RES_THEAD_SUPER"),
                RES_TPROG_SUPER => println!("RES_TPROG_SUPER"),
                RES_TASK_IN_USER => println!("RES_TASK_IN_USER"),
                RES_TDAT_SUPER => println!("RES_TDAT_SUPER"),
                _ => println!("INVALID RESOURSE"),
            }
        }
        println!();
    }
}