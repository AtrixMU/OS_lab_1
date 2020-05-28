use super::process::Process;
use crate::real_machine::processor::RMProcessor;
use crate::consts::*;
use super::resource::Resource;
use super::read_from_disk::ReadFromDisk;
use super::jcl::JCL;


pub struct JobToUMem {
    id: usize,
    parent_id: usize,
    vm: usize,
    state: usize,
    section: usize,
    resources: Vec<Resource>
}


impl JobToUMem {
    pub fn new(id: usize, parent_id: usize, vm: usize) -> JobToUMem {
        JobToUMem {
            id: id,
            parent_id: parent_id,
            vm: vm,
            state: P_READY,
            section: 0,
            resources: Vec::new(),
        }
    }
}

impl Process for JobToUMem {
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
    fn step(&mut self, rm: &mut RMProcessor) -> (Option<usize>, Option<Resource>, Option<Box<dyn Process>>) {
        match self.section {
            0 => {
                if self.has_resource(RES_THEAD_SUPER) {
                    self.section += 1;
                    self.state = P_READY;
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_THEAD_SUPER), None, None);
                }
            },
            1 => {
                if self.has_resource(RES_TDAT_SUPER) {
                    self.section += 1;
                    self.state = P_READY;
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_TDAT_SUPER), None, None);
                }
            },
            2 => {
                if self.has_resource(RES_TPROG_SUPER) {
                    self.section += 1;
                    self.state = P_READY;
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_TPROG_SUPER), None, None);
                }
            },
            3 => {
                if self.has_resource(RES_U_MEM) {
                    self.section += 1;
                    self.state = P_READY;
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_U_MEM), None, None);
                }
            },
            4 => {
                todo!();
            }
            5 => {
                let mut res = Resource::new(RES_TASK_IN_USER);
                res.set_recipient(PID_MAIN_PROC);
                self.section = 0;
                return(None, Some(res),None);
            }

            _ => panic!(),
        }
    }
}