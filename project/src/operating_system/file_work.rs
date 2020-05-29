use super::process::Process;
use crate::real_machine::processor::RMProcessor;
use crate::consts::*;
use super::resource::Resource;


pub struct FileWork {
    id: usize,
    parent_id: usize,
    vm: usize,
    state: usize,
    section: usize,
    resources: Vec<Resource>,
    vm_id: usize,
    priority: usize,
}


impl FileWork {
    pub fn new(id: usize, parent_id: usize, vm: usize) -> FileWork {
        FileWork {
            id: id,
            parent_id: parent_id,
            vm: vm,
            state: P_READY,
            section: 0,
            resources: Vec::new(),
            vm_id: 10,
            priority: 3,
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

impl Process for FileWork {
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
                if self.has_resource(RES_FILE_PACK) {
                    self.section = 1;
                    self.state = P_READY;
                    return (None, None, None, None);
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_FILE_PACK), None, None, None);
                }
            },
            1 => {
                if self.has_resource(RES_DISK) {
                    self.section += 1;
                    self.state = P_READY;
                    return (None, None, None, None);
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_DISK), None, None, None);
                }

            },
            2 => {
                if self.has_resource(RES_CHNL) {
                    self.section +=1;
                    self.state = P_READY;
                    return (None, None, None, None);
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_CHNL), None, None, None);
                }
            },
            3 => { // Parenkame failu dirbimo rezima pagal gauta pranesima
                let message = self.take_resource(RES_FILE_PACK).get_msg();
                match message.as_str() {
                    "Open" => {
                        rm.process_open();
                    },
                    "Close" => {
                        rm.process_cls();
                    },
                    "Read" => {
                        rm.process_read();
                    },
                    "Write" => {
                        rm.process_wrt();
                    },
                    "Delete" => {
                        rm.process_del();
                    }
                    _ => panic!()
                }
                if rm.get_pi() != 5 || rm.get_pi() != 5 {
                    self.section += 1;
                }
                else {
                    self.section = 7;
                }
               return(None,None,None,None);     
            },
            4 => {
                // let msg = self.get_msg(RES_TASK_IN_USER);
                let mut res = self.take_resource(RES_CHNL);
                self.section += 1;
                // res.set_msg(format!("{} {}", msg, self.vm_id));
                return (None, Some(res), None, None)
            },
            5 => {
                let mut res = self.take_resource(RES_DISK);
                self.section += 1;
                return (None, Some(res), None, None)
            },

            6 => {
                let mut res = Resource::new(RES_FROM_FILEWORK);
                self.section = 0;
                self.resources = Vec::new();
                return (None, Some(res), None, None);
            },

            7 => {
                let mut res = self.take_resource(RES_CHNL);
                self.section += 1;
                // res.set_msg(format!("{} {}", msg, self.vm_id));
                return (None, Some(res), None, None)
            },
            8 => {
                let mut res = self.take_resource(RES_DISK);
                self.section += 1;
                return (None, Some(res), None, None)
            },
            9 => {
                let mut res = Resource::new(RES_FROM_FILEWORK);
                res.set_msg("Filework error".to_string());
                res.set_recipient(self.get_vm());
                self.section = 0;
                self.resources = Vec::new();
                return(None, Some(res), None, None);
            }

            _ => panic!(),
        }
    }

    fn print(&self, rm: &RMProcessor) {
        println!("Process: MainProc");
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