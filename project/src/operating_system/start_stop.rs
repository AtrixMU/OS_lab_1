use super::process::Process;
use crate::real_machine::processor::RMProcessor;
use crate::consts::*;
use super::resource::Resource;
use super::read_from_disk::ReadFromDisk;
use super::jcl::JCL;
use super::job_to_umem::JobToUMem;
use super::main_proc::MainProc;


pub struct StartStop {
    id: usize,
    parent_id: usize,
    vm: usize,
    state: usize,
    section: usize,
    resources: Vec<Resource>,
    priority: usize,
}


impl StartStop {
    pub fn new(id: usize, parent_id: usize, vm: usize) -> StartStop {
        StartStop {
            id: id,
            parent_id: parent_id,
            vm: vm,
            state: P_READY,
            section: 0,
            resources: Vec::new(),
            priority: 3,
        }
    }
}

impl Process for StartStop {
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
                let res = Resource::new(RES_S_MEM);
                self.section += 1;
                return (None, Some(res), None, None);
            },
            1 => {
                let res = Resource::new(RES_U_MEM);
                self.section += 1;
                return (None, Some(res), None, None);
            },
            2 => {
                let res = Resource::new(RES_DISK);
                self.section += 1;
                return (None, Some(res), None, None);
            },
            3 => {
                let new_proc = ReadFromDisk::new(PID_READ_FROM_DISK, self.id, 0);
                self.section += 1;
                return (None, None, Some(Box::new(new_proc)), None);
            },
            4 => {
                let new_proc = JCL::new(PID_JCL, self.id, 0);
                self.section += 1;
                return (None, None, Some(Box::new(new_proc)), None);
            },
            5 => {
                let new_proc = JobToUMem::new(PID_JOB_TO_UMEM, self.id, 0);
                self.section += 1;
                return (None, None, Some(Box::new(new_proc)), None);
            },
            6 => {
                let new_proc = MainProc::new(PID_MAIN_PROC, self.id, 0);
                self.section += 1;
                return (None, None, Some(Box::new(new_proc)), None);
            }
            _ => panic!(),
        }
    }
    fn print(&self, _rm: &RMProcessor) {
        println!("Process: StartStop");
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
