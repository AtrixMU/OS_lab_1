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

                    }
                }
                
            

            }

        }

    } 
}