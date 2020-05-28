use super::process::Process;
use crate::real_machine::processor::RMProcessor;
use crate::consts::*;
use super::resource::Resource;
use super::read_from_disk::ReadFromDisk;
use super::jcl::JCL;


pub struct StartStop {
    id: usize,
    parent_id: usize,
    vm: usize,
    state: usize,
    section: usize,
    resources: Vec<Resource>
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
    fn step(&mut self, rm: &mut RMProcessor) -> (Option<usize>, Option<Resource>, Option<Box<dyn Process>>) {
        match self.section {
            0 => {
                let res = Resource::new(RES_S_MEM);
                self.section += 1;
                return (None, Some(res), None);
            },
            1 => {
                let res = Resource::new(RES_U_MEM);
                self.section += 1;
                return (None, Some(res), None);
            },
            2 => {
                let res = Resource::new(RES_DISK);
                self.section += 1;
                return (None, Some(res), None);
            },
            3 => {
                let new_proc = ReadFromDisk::new(PID_READ_FROM_DISK, self.id, 0);
                self.section += 1;
                return (None, None, Some(Box::new(new_proc)));
            },
            4 => {
                let new_proc = JCL::new(PID_JCL, self.id, 0);
                self.section += 1;
                return (None, None, Some(Box::new(new_proc)));
            }
            _ => panic!(),
        }
    }
}
