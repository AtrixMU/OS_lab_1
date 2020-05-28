use super::process::Process;
use crate::real_machine::processor::RMProcessor;
use crate::consts::*;
use super::resource::Resource;
use crate::types::Word;


pub struct ReadFromDisk {
    id: usize,
    parent_id: usize,
    vm: usize,
    state: usize,
    section: usize,
    resources: Vec<Resource>,
    file_data: Vec<Word>
}

impl ReadFromDisk {
    pub fn new(id: usize, parent_id: usize, vm: usize) -> ReadFromDisk {
        ReadFromDisk {
            id: id,
            parent_id: parent_id,
            vm: vm,
            state: P_READY,
            section: 0,
            resources: Vec::new(),
            file_data: Vec::new(),
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

impl Process for ReadFromDisk {
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
                if self.has_resource(RES_FROM_USER_INT) {
                    self.state = P_READY;
                    self.section += 1;
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_FROM_USER_INT), None, None, None);
                }
            },
            1 => {
                if self.has_resource(RES_CHNL) {
                    self.state = P_READY;
                    self.section += 1;
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_CHNL), None, None, None);
                }
            },
            2 => {
                let file_name = self.get_msg(RES_FROM_USER_INT);
                self.file_data = rm.mmu.get_file_data(file_name);
            },
            3 => {
                if self.has_resource(RES_S_MEM) {
                    self.state = P_READY;
                    self.section += 1;
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_S_MEM), None, None, None);
                }
            },
            4 => {
                let ptr = rm.mmu.upload_to_smem(&self.file_data);
                let mut res = Resource::new(RES_TASK_IN_SUPER);
                res.set_msg(format!("{}", ptr));
                self.add_resource(res);
                self.section += 1;
                return (None, None, None, None);
            },
            5 => {
                self.section += 1;
                return (None, Some(self.take_resource(RES_S_MEM)), None, None)
            },
            6 => {
                self.section += 1;
                return (None, Some(self.take_resource(RES_CHNL)), None, None)
            },
            7 => {
                self.section = 0;
                self.take_resource(RES_FROM_USER_INT);
                let res = self.take_resource(RES_TASK_IN_SUPER);
                self.resources = Vec::new();
                return (None, Some(res), None, None)
            }
            _ => panic!(),
        }
        (None, None, None, None)
    }
    fn print(&self, _rm: &RMProcessor) {
        println!("Process: ReadFromDisk");
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
