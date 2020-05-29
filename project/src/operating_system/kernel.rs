//use super::process::Process;
use super::resource_list::ResourceList;
use super::process::Process;
use crate::consts::*;
use crate::real_machine::processor::RMProcessor;
use super::start_stop::StartStop;

pub struct Kernel {
    process_list: Vec<Box<dyn Process>>,
    resources: ResourceList,

}

impl Kernel {
    pub fn new() -> Kernel {
        Kernel {
            process_list: Vec::new(),
            resources: ResourceList::new(),
        }
    }
}

impl Kernel {
    pub fn planner(&mut self, rm:&mut RMProcessor) {
        self.process_list.sort_by_key(|d| d.get_priority());
        let mut queued_processes: Vec<Box<dyn Process>> = Vec::new();
        let mut to_be_removed: Vec<usize> = Vec::new();
        let mut statuses: Vec<(usize, usize)> = Vec::new();
        self.process_list.push(Box::new(StartStop::new(PID_STARTSTOP, 0, 0)));
        loop {
            for process in &mut self.process_list {
                println!("hello");
                process.print(rm);
                if to_be_removed.contains(&process.get_id()) {
                    continue;
                }
                if process.get_state() != P_READY_SUSP || process.get_state() != P_BLOCKED_SUSP {
                    let result = process.step(rm);
                    if result.0.is_some() && result.3.is_none() {
                        let value = result.0.unwrap();
                        let gotten_resource = self.resources.take(value);
                        if gotten_resource.is_some() {
                            process.add_resource(gotten_resource.unwrap());
                        }
                    }
                    else if result.1.is_some() {
                        let value = result.1.unwrap();
                        self.resources.add(value);
                    }
                    else if result.2.is_some() {
                        let value = result.2.unwrap();
                        queued_processes.push(value);
                    }
                    else if result.3.is_some() && result.0.is_none() {
                        let p_id = result.3.unwrap();
                        to_be_removed.push(p_id);
                    }
                    else if result.0.is_some() && result.3.is_some() {
                        let p_id = result.3.unwrap();
                        let status_id = result.0.unwrap();
                        statuses.push((p_id, status_id));
                    }
                    else if result.0.is_none() && result.1.is_none() && result.2.is_none()
                        && result.3.is_none()
                    {
                        process.step(rm);
                    }

                }
            }
            self.process_list.append(&mut queued_processes);
            queued_processes.clear();
            self.process_list.retain(|i| !to_be_removed.contains(&i.get_id()) );
            to_be_removed.clear();
            for (p_id, status_id) in &statuses {
                self.process_list.iter_mut().find(| x | x.get_id() == *p_id).unwrap().set_state(*status_id);
            }
        }
    } 
}