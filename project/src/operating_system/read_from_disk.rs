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

    fn step(&mut self, rm: &mut RMProcessor) -> (Option<usize>, Option<Resource>, Option<Box<dyn Process>>) {
        match self.section {
            0 => {
                if self.has_resource(RES_FROM_USER_INT) {
                    self.section += 1;
                }
                else {
                    return (Some(RES_FROM_USER_INT), None, None);
                }
            },
            1 => {
                if self.has_resource(RES_CHNL) {
                    self.section += 1;
                }
                else {
                    return (Some(RES_CHNL), None, None);
                }
            },
            2 => {
                let file_name = self.get_msg(RES_FROM_USER_INT);
                self.file_data = rm.mmu.get_file_data(file_name);
            },
            3 => {
                if self.has_resource(RES_S_MEM) {
                    self.section += 1;
                }
                else {
                    return (Some(RES_S_MEM), None, None);
                }
            },
            4 => {
                let ptr = rm.mmu.upload_to_smem(&self.file_data);
                let mut res = Resource::new(RES_TASK_IN_SUPER);
                res.set_msg(format!("{}", ptr));
                self.add_resource(res);
                self.section += 1;
                return (None, None, None);
            },
            5 => {
                self.section += 1;
                return (None, Some(self.take_resource(RES_S_MEM)), None)
            },
            6 => {
                self.section += 1;
                return (None, Some(self.take_resource(RES_CHNL)), None)
            },
            7 => {
                self.section = 0;
                self.take_resource(RES_FROM_USER_INT);
                return (None, Some(self.take_resource(RES_TASK_IN_SUPER)), None)
            }
            _ => panic!(),
        }
        (None, None, None)
    }
}
