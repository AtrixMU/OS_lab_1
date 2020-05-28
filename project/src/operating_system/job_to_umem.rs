use super::process::Process;
use crate::real_machine::processor::RMProcessor;
use crate::consts::*;
use super::resource::Resource;


pub struct JobToUMem {
    id: usize,
    parent_id: usize,
    vm: usize,
    state: usize,
    section: usize,
    resources: Vec<Resource>,
    user_ptr: usize,
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
            user_ptr: 0,
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
    fn step(&mut self, rm: &mut RMProcessor) -> (Option<usize>, Option<Resource>, Option<Box<dyn Process>>, Option<usize>) {
        match self.section {
            0 => {
                if self.has_resource(RES_THEAD_SUPER) {
                    self.section += 1;
                    self.state = P_READY;
                    return (None, None, None, None);
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_THEAD_SUPER), None, None, None);
                }
            },
            1 => {
                if self.has_resource(RES_TDAT_SUPER) {
                    self.section += 1;
                    self.state = P_READY;
                    return (None, None, None, None);
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_TDAT_SUPER), None, None, None);
                }
            },
            2 => {
                if self.has_resource(RES_TPROG_SUPER) {
                    self.section += 1;
                    self.state = P_READY;
                    return (None, None, None, None);
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_TPROG_SUPER), None, None, None);
                }
            },
            3 => {
                if self.has_resource(RES_U_MEM) {
                    self.section += 1;
                    self.state = P_READY;
                    return (None, None, None, None);
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_U_MEM), None, None, None);
                }
            },
            4 => {
                let kernel_ptr = self.get_msg(RES_THEAD_SUPER).parse::<usize>().unwrap();
                self.user_ptr = rm.mmu.smem_to_umem(kernel_ptr) as usize;
                return (None, None, None, None);
            },
            5 => {
                let mut res = Resource::new(RES_TASK_IN_USER);
                res.set_msg(format!("{} {}", self.user_ptr, RUN_DUR));
                self.section = 0;
                self.resources = Vec::new();
                return(None, Some(res),None, None);
            },

            _ => panic!(),
        }
    }
}