use super::process::Process;
use crate::virtual_machine::processor::VMProcessor;
use crate::consts::*;
use super::resource::Resource;


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
    fn take_resource(&mut self, resource_index: usize) -> Resource {
        self.resources.remove(resource_index)
    }
    fn progress(&mut self) -> (Option<usize>, Option<Resource>) {
        (None, None)
    }
}
