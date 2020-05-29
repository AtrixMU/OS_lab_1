use super::process::Process;
use crate::real_machine::processor::RMProcessor;
use crate::consts::*;
use super::resource::Resource;


pub struct JCL {
    id: usize,
    parent_id: usize,
    vm: usize,
    state: usize,
    section: usize,
    resources: Vec<Resource>,
    ptr: usize,
    code_index: usize,
    priority: usize,
}


impl JCL {
    pub fn new(id: usize, parent_id: usize, vm: usize) -> JCL {
        JCL {
            id: id,
            parent_id: parent_id,
            vm: vm,
            state: P_READY,
            section: 0,
            resources: Vec::new(),
            ptr: 0,
            code_index: 0,
            priority: 3
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

impl Process for JCL {
    fn get_id(&self) -> usize {
        self.id
    }
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
                if self.has_resource(RES_TASK_IN_SUPER) {
                    self.section += 1;
                    self.state = P_READY;
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_TASK_IN_SUPER), None, None, None);
                }
            },
            1 => {
                // saraso inicijavimas
                self.ptr = self.get_msg(RES_TASK_IN_SUPER)
                    .parse::<usize>()
                    .unwrap();
                self.section += 1;
            },
            2 => {
                // imamas blokas
                if rm.mmu.kernel_memory[self.ptr * PAGE_SIZE].is_empty()
                    || rm.mmu.kernel_memory[(self.ptr * PAGE_SIZE) + PAGE_SIZE - 1].is_empty()
                {
                    self.section = 4;
                }
                else {
                    self.section += 1;
                }
            },
            3 => {
                // antrastes bloko patikrinimas
                let mut res = Resource::new(RES_THEAD_SUPER);
                res.set_msg(format!("{}", self.ptr));
                self.section = 5;
                return (None, Some(res), None, None);
            },
            4 => { //Error'as jeigu netinkamas
                let mut res = Resource::new(RES_LINE_IN_MEM);
                res.set_msg("eERROR: Invalid header block!".to_string());
                res.set_recipient(PID_PRINT_LINE);
                self.section = 0;
                self.resources = Vec::new();
                return (None, Some(res), None, None);
            },
            5 => { 
                let block_list = rm.mmu.kernel_memory[(self.ptr * PAGE_SIZE) + PAGE_SIZE - 1].as_u32() as usize;
                let mut last_word = String::new();
                let mut cmd_index = 0;
                let mut current_block = 0;
                while last_word != String::from("#COD") {
                    if cmd_index % PAGE_SIZE == 0 {
                        current_block = rm.mmu.kernel_memory[(block_list as usize * PAGE_SIZE) + cmd_index / PAGE_SIZE].as_u32() as usize;
                    }
                    if current_block == 0 {
                        self.section = 7;
                        return (None, None, None, None);
                    }
                    let cmd = rm.mmu.kernel_memory[(current_block * PAGE_SIZE) + (cmd_index % PAGE_SIZE)];
                    if cmd.as_text().is_ok() {
                        last_word = cmd.as_text().unwrap();
                    }
                    cmd_index += 1;
                }
                self.code_index = cmd_index;
                self.section = 6;
            },
            6 => {
                let res = Resource::new(RES_TDAT_SUPER);
                self.section = 8;
                return (None, Some(res), None, None);
            },            
            7 => {
                let mut res = Resource::new(RES_LINE_IN_MEM);
                res.set_msg("eERROR: Code block does not exist!".to_string());
                res.set_recipient(PID_PRINT_LINE);
                self.section = 0;
                self.resources = Vec::new();
                return(None, Some(res), None, None);
            },
            8 => {
                let block_list = rm.mmu.kernel_memory[(self.ptr * PAGE_SIZE) + PAGE_SIZE - 1].as_u32() as usize;
                let current_block = rm.mmu.kernel_memory[(block_list as usize * PAGE_SIZE) + self.code_index / PAGE_SIZE].as_u32() as usize;
                if current_block == 0 {
                    self.section = 10;
                    return (None, None, None, None);
                }
                let cmd = rm.mmu.kernel_memory[(current_block * PAGE_SIZE) + (self.code_index % PAGE_SIZE)];
                self.code_index += 1;
                if cmd.as_text().is_err() {
                    self.section = 10;
                    return (None, None, None, None);
                }
                let cmd = cmd.as_text().unwrap();
                if &cmd == "HALT" {
                    self.section = 9;
                    return (None, None, None, None);
                }
                if cmd.chars().last().expect("error parsing cmd") == 'R' || ["LOAD", "STOR"].contains(&cmd.as_str()) {
                    for _ in 0..2 {
                        let block_list = rm.mmu.kernel_memory[(self.ptr * PAGE_SIZE) + PAGE_SIZE - 1].as_u32() as usize;
                        let current_block = rm.mmu.kernel_memory[(block_list as usize * PAGE_SIZE) + self.code_index / PAGE_SIZE].as_u32() as usize;
                        if current_block == 0 {
                            self.section = 10;
                            return (None, None, None, None);
                        }
                        let cmd = rm.mmu.kernel_memory[(current_block * PAGE_SIZE) + (self.code_index % PAGE_SIZE)];
                        self.code_index += 1;
                        if cmd.as_text().is_err() {
                            self.section = 10;
                            return (None, None, None, None);
                        }
                    }
                    self.section = 8;
                    return (None, None, None, None);
                }
                else if cmd.chars().last().expect("error parsing cmd") == 'V' || cmd == "LOOP" || cmd == "MOVN" {
                    let block_list = rm.mmu.kernel_memory[(self.ptr * PAGE_SIZE) + PAGE_SIZE - 1].as_u32() as usize;
                    let current_block = rm.mmu.kernel_memory[(block_list as usize * PAGE_SIZE) + self.code_index / PAGE_SIZE].as_u32() as usize;
                    if current_block == 0 {
                        self.section = 10;
                        return (None, None, None, None);
                    }
                    let cmd = rm.mmu.kernel_memory[(current_block * PAGE_SIZE) + (self.code_index % PAGE_SIZE)];
                    self.code_index += 1;
                    if cmd.as_text().is_err() {
                        self.section = 10;
                        return (None, None, None, None);
                    }
                    self.code_index += 1;
                    self.section = 8;
                    return (None, None, None, None);
                }
                else if ["JUMP", "JPEQ", "JPOF", "JPGE", "JPBE", "JMPG", "JMPB"].contains(&cmd.as_str()) {
                    self.code_index += 1;
                    self.section = 8;
                    return (None, None, None, None);
                }
                self.code_index += 1;
                self.section = 8;
                return (None, None, None, None);
            },
            9 => {
                self.section = 0;
                return(None, Some(self.take_resource(RES_TASK_IN_SUPER)), None, None);
            },
            10 => {
                let mut res = Resource::new(RES_LINE_IN_MEM);
                res.set_msg("eERROR: invalid command!".to_string());
                res.set_recipient(PID_PRINT_LINE);
                self.section = 0;
                self.resources = Vec::new();
                return(None, Some(res), None, None);
            },
            _ => panic!(),
        }
        (None, None, None, None)
    }
    fn print(&self, _rm: &RMProcessor) {
        println!("Process: JCL");
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
    fn get_priority(&self) -> usize {
        self.priority
    }
}
