use super::process::Process;
use crate::real_machine::processor::RMProcessor;
use crate::consts::*;
use super::resource::Resource;


pub struct JCL {
    id: usize,
    parent_id: usize,
    vm: usize,
    state: usize,
    section: usize,
    resources: Vec<Resource>,
    ptr: usize,
}


impl JCL {
    pub fn new(id: usize, parent_id: usize, vm: usize) -> JCL {
        JCL {
            id: id,
            parent_id: parent_id,
            vm: vm,
            state: P_READY,
            section: 0,
            resources: Vec::new(),
            ptr: 0,
        }
    }
}

impl Process for JCL {
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
                if self.has_resource(RES_TASK_IN_SUPER) {
                    self.section += 1;
                    self.state = P_READY;
                }
                else {
                    self.state = P_BLOCKED;
                    return (Some(RES_TASK_IN_SUPER), None, None);
                }
            },
            1 => {
                todo!(); // saraso inicijavimas
            },
            2 => {
                todo!(); // imamas blokas
            },
            3 => {
                todo!(); // antrastes bloko patikrinimas
            },
            4 => { //Error'as jeigu netinkamas
                let mut res = Resource::new(RES_LINE_IN_MEM);
                res.set_msg("eERROR: Invalid header block!".to_string());
                res.set_recipient(PID_PRINT_LINE);
                self.section = 0;
                return (None, Some(res), None);
            },
            5 => { // einame toliau jeigu tinkama
                todo!();
                //speju kazkas tokio: let resource = new Resource (RES_THEAD_SUPER,"", PID_JOB_TO_UMEM);
                self.section += 1;
                let mut res = Resource::new(RES_THEAD_SUPER);
                res.set_recipient(PID_JOB_TO_UMEM);
                return(None, Some(res),None);
            },
            6 => { // imamas blokas is supervizorines atminties
                self.section += 1;
                todo!();
            },            
            7 => { // patikriname ar tai yra ar dar vis yra #DAT bloke
                todo!();
            },
            8 => { // prijungame #DAT bloka, griztame i 7
                self.section = 7;
                todo!();
            },
            9 => { // patikriname, ar tai yra #COD blokas
                todo!();
            },
            10 => { // metame error, jeigu nera #COD bloko
                let mut res = Resource::new(RES_LINE_IN_MEM);
                res.set_msg("eERROR: Code block does not exist!".to_string());
                res.set_recipient(PID_PRINT_LINE);
                self.section = 0;
                return(None, Some(res),None);
            },
            11 => { // Atlaisviname "Uzduoties duomenys super atmintyje " resursa
                todo!();
                self.section += 1;
                return(None, Some(self.take_resource(RES_TASK_IN_SUPER)), None);
            },
            12 => { // skaitome komanda
                self.section += 1;
                todo!();
            },
            13 => { // tikriname, ar komanda korektiska
                todo!();
            },
            14 => { // metame error, jeigu nekorektiska
                let mut res = Resource::new(RES_LINE_IN_MEM);
                res.set_msg("eERROR: invalid command!".to_string());
                res.set_recipient(PID_PRINT_LINE);
                self.section = 0;
                return(None, Some(res), None);
            },
            15 => { // tikriname, ar tai yra HALT, jei ne, griztame i 12, kitu atveju i 16
                todo!();

            },
            16 => { // atlaisviname "Uzduoties programa supervizorineje atmintyje" resursa
                self.section = 0;
                todo!();
            },
            _ => panic!(),
        }
        (None, None, None)
    }
}
