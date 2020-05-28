use super::process::Process;
use crate::real_machine::processor::RMProcessor;
use crate::consts::*;
use super::resource::Resource;

pub struct PrintLine {
    id: usize,
    parent_id: usize,
    vm: usize,
    state: usize,
    section: usize,
    resources: Vec<Resource>
}

impl PrintLine {
    pub fn new(id: usize, parent_id: usize, vm: usize) -> PrintLine {
        PrintLine {
            id: id,
            parent_id: parent_id,
            vm: vm,
            state: P_READY,
            section: 0,
            resources: Vec::new(),
        }
    }
}

impl Process for PrintLine {
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
        for res in self.resources {
            if res.get_type() == resource_type {
                return true;
            }
        }
        false
    }
    fn step(&mut self, rm: &mut RMProcessor) -> (Option<usize>, Option<Resource>, Option<Box<dyn Process>>) {
        match self.section {
            0 => {
                if self.has_resource(RES_LINE_IN_MEM) {
                    self.section += 1;
                    self.state = P_READY;
                    (None,None,None)
                }
                else {
                    return (Some(RES_LINE_IN_MEM), None, None);
                }
            },
            1 => {
                if self.has_resource(RES_CHNL){
                    self.section += 1;
                    self.state = P_READY;
                    (None,None,None)
                }
                    
                else {
                    return (Some(RES_CHNL), None, None);
                }
                
            },
            2 => {
                let source_id = 0;
                let destination_id = 4;
                self.print(rm);
                (None,None,None)
            },
            3 => {
                return(None,Some(self.take_resource(RES_CHNL)),None);
            },
            _ => panic!(),
            
        }     
    }
    fn print(&self, rm:&mut RMProcessor) {
        let message = String::new();
        for resource in self.resources {
            if resource.get_type() == RES_LINE_IN_MEM {
                message = self.resources.remove(resource.get_type()).get_msg();
                break;
            }      
        }
        let letter = message.chars().next();
        let printing =&message[1..];
        match letter.unwrap() {
            'e' => println!("{}", printing),
            'n' => {rm.process_prtn()},
            's' => {rm.process_prts()},
             _ => println!("Invalid type for printing!") 

        }
    }
}