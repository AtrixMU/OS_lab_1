use super::process::Process;
use crate::real_machine::processor::RMProcessor;
use crate::consts::*;
use super::resource::Resource;
use super::read_from_disk::ReadFromDisk;
use super::jcl::JCL;


pub struct JobGovernor {
    id: usize,
    parent_id: usize,
    vm: usize,
    state: usize,
    section: usize,
    resources: Vec<Resource>
}


impl JobGovernor {
    pub fn new(id: usize, parent_id: usize, vm: usize) -> JobGovernor {
        JobGovernor {
            id: id,
            parent_id: parent_id,
            vm: vm,
            state: P_READY,
            section: 0,
            resources: Vec::new(),
        }
    }
}

impl Process for JobGovernor {
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
                if self.has_resource(RES_U_MEM) {
                    self.section += 1;
                    self.state = P_READY;
                    (None,None,None, None)
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_U_MEM), None, None, None);
                }
            },
            1 => {
                let new_proc = VM::new(self.id+10, self.id, 0);
                self.section += 1;
                return (None, None, Some(Box::new(new_proc)), None);
            },
            2 => {
                if self.has_resource(RES_INTERRUPT) {
                    self.section += 1;
                    self.state = P_READY;
                    (None, None, None, None)
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_INTERRUPT), None, None, None);
                }
            },
            3 => { //Reikia padaryti virtualios masinos stabdyma
                todo!();
            },
            4 => { // patikrinti ar halt, patikrinti ar failas, ir ar ivedimo
                todo!();
            },
            5 => {// patikrinti pagal interrupta koki message nusiusti (cia yra output)
                todo!();
                let mut res = Resource::new(RES_LINE_IN_MEM);
                
                // if INT  1, print("n"), if INT 2, print("s")
                res.set_recipient(PID_PRINT_LINE);
                return(None, Some(res), None), None;
            },
            6 => {//ivedimas (input)
                todo!();
                
            },
            7 => { // nebestabdomas VM
                todo!();
            },
            8 => { // darbas su failu paketais, reikia, kad cia gautu atitinkama
                   // Kernel Interrupt reiksme
                   todo!();
                let mut res = Resource::new(RES_FILE_PACK);
            },
            9 => {
                if self.has_resource(RES_FROM_FILEWORK) {
                    self.section += 1;
                    self.state = P_READY;
                    (None, None, None, None)
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_FROM_FILEWORK), None, None, None);
                }
            },


            _ => panic!(),
        }
    }
    fn print(&self, rm:&mut RMProcessor) {
        println!("Section: {}", self.section);
        for resource in self.resources{
            println!("Resource: {}", resource.get_type());
        }
    }
}