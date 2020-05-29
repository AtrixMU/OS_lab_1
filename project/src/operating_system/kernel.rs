//use super::process::Process;
use super::resource_list::ResourceList;
use super::process::Process;
use crate::consts::*;
use crate::real_machine::processor::RMProcessor;

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
    pub fn planner( &mut self, rm:&mut RMProcessor) {
        self.process_list.sort_by_key(|d| d.get_priority());
        let i = 0;
        
        while true  {
            for process in &mut self.process_list {
                if process.get_state() == P_BLOCKED {
                    let result =    process.step(rm);
                    if result.0.is_some() && result.3.is_none() {
                        let value = result.0.unwrap();
                        let gotten_resource = self.resources.take(value);
                        if gotten_resource.is_some() {
                            process.add_resource(gotten_resource.unwrap());
                        }
                    }
                    else if result.1.is_some() {
                        let value = result.1.unwrap();
                        let taken_resource = process.take_resource(value.get_type());
                        self.resources.add(taken_resource);
                    }
                    else if result.2.is_some() {
                        let value = result.2.unwrap();
                        self.process_list.push(value);
                    }
                    else if result.3.is_some() && result.0.is_none() {
                        let p_id = result.3.unwrap();
                        for processs in self.process_list{
                            if processs.get_id() == p_id{
                                self.process_list.remove(p_id);
                                break;
                            }
                        }
                    }
                    else if result.0.is_some() && result.3.is_some() {
                        let p_id = result.3.unwrap();
                        let status_id = result.0.unwrap();
                        for process in self.process_list {
                            if process.get_id() == p_id{
                                process.set_state(status_id);
                            }
                        }
                    }
                    else if result.0.is_none() && result.1.is_none() && result.2.is_none() &&
                    result.3.is_none(){
                                process.step(rm);
                    }
                }
                
            

            }

        }

    } 
}