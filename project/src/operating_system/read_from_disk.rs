use super::process::Process;
use crate::virtual_machine::processor::VMProcessor;
use crate::real_machine::processor::RMProcessor;
use crate::consts::*;
use super::resource::Resource;


pub struct ReadFromDisk {
    id: usize,
    parent_id: usize,
    vm: usize,
    state: usize,
    section: usize,
    resources: Vec<Resource>
}


impl ReadFromDisk {
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
    fn take_resource(&mut self, resource_index: usize) -> Resource {
        self.resources.remove(resource_index)
    }
    fn has_resource(&self, resource_type: usize) -> bool {
        for res in self.resources {
            if res.get_type() == resource_type {
                return true;
            }
        }
        false
    }
    fn step(&mut self, rm: &RMProcessor) -> (Option<usize>, Option<Resource>, Option<Box<dyn Process>>) {
        if self.section == 0 {
            if self.has_resource(RES_FROM_USER_INT) {
                self.section += 1;
            }
            else {
                return (Some(RES_FROM_USER_INT), None, None);
            }
        }
        if self.section == 1 {
            if self.has_resource(RES_CHNL) {
                self.section += 1;
            }
            else {
                return (Some(RES_CHNL), None, None);
            }
        }
        if self.section == 2 {
            
        }
        (None, None, None)
    }
}
