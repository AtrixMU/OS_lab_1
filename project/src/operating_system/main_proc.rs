use super::process::Process;
use crate::real_machine::processor::RMProcessor;
use crate::consts::*;
use super::resource::Resource;
use super::job_governor::JobGovernor;


pub struct MainProc {
    id: usize,
    parent_id: usize,
    vm: usize,
    state: usize,
    section: usize,
    resources: Vec<Resource>
    vm_id: usize
}


impl MainProc {
    pub fn new(id: usize, parent_id: usize, vm: usize) -> MainProc {
        MainProc {
            id: id,
            parent_id: parent_id,
            vm: vm,
            state: P_READY,
            section: 0,
            resources: Vec::new(),
            vm_id: 10,
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

impl Process for MainProc {
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
                if self.has_resource(RES_TASK_IN_USER) {
                    self.section = 2;
                    self.state = P_READY;
                    return (None, None, None, None);
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_TASK_IN_SUPER), None, None, None);
                }
            },
            1 => {
                let msg = self.get_msg(RES_TASK_IN_USER);
                let params: Vec<&str> = msg.split_whitespace().collect();
                if params[1].parse::<usize>().unwrap() > 0 {
                    self.section = 3;
                    return (None, None, None, None);
                }
                self.section = 2;
                return (None, None, None, None);

            },
            2 => {
                let msg = self.get_msg(RES_TASK_IN_USER);
                let params: Vec<&str> = msg.split_whitespace().collect();
                let kill = params[2].parse::<usize>().unwrap();
                self.section = 0;
                self.resources = Vec::new();
                return (None, None, None, Some(kill));
            },
            3 => {
                let new_proc = JobGovernor::new(self.vm_id, self.id, 0);
                self.section += 1;
                return (None, None, Some(Box::new(new_proc)), None);
            }
            4 => {
                // let msg = self.get_msg(RES_TASK_IN_USER);
                let mut res = self.take_resource(RES_TASK_IN_USER);
                res.set_recipient(self.vm_id);
                // res.set_msg(format!("{} {}", msg, self.vm_id));
                self.vm_id += 1;
                self.section = 0;
                self.resources = Vec::new();
                return (None, Some(res), None, None)
            }

            _ => panic!(),
        }
    }
}