use super::process::Process;
use crate::virtual_machine::processor::VMProcessor;
use crate::real_machine::processor::RMProcessor;
use crate::consts::*;
use super::resource::Resource;


pub struct JCL {
    id: usize,
    parent_id: usize,
    vm: usize,
    state: usize,
    section: usize,
    resources: Vec<Resource>

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
            pointer: usize,
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
        match self.section {
            0 => {
                if self.has_resource(RES_TASK_IN_SUPER){
                    self.section+=1;
                }
                else {
                    return (Some(RES_TASK_IN_SUPER),None,None);
                }
            }

            1 => {
                todo!(); // saraso inicijavimas
            }

            2 => {
                todo!(); // imamas blokas
            }

            3 => {
                todo!(); // antrastes bloko patikrinimas
            }

            4 => { //Error'as jeigu netinkamas
                let resource = new Resource(RES_LINE_IN_MEM, "eERROR: Wrong header block!", PID_PRINT_LINE);
                self.section = 0;
                return (None, Some(resource), None);
            }

            5 => { // einame toliau jeigu tinkama
                todo!();
                //speju kazkas tokio: let resource = new Resource (RES_THEAD_SUPER,"", PID_JOB_TO_UMEM);
                self.section+= 1;
                return(None, Some(reource),None);
            }

            6 => { // imamas blokas is supervizorines atminties
                todo!();
                self.section+= 1;
            }
            
            7 => { // patikriname ar tai yra ar dar vis yra #DAT bloke
                todo!();
            }

            8 => { // prijungame #DAT bloka, griztame i 7
                todo!();
                self.section = 7;
            }

            9 => { // patikriname, ar tai yra #COD blokas
                todo!();

            }

            10 => { // metame error, jeigu nera #COD bloko
                let resource = new Resource (RES_LINE_IN_MEM, "eERROR: Code block Does not exist!", PID_PRINT_LINE);
                self.section = 0;
                return(None, some(resource),None);
            }

            11 => { // Atlaisviname "Uzduoties duomenys super atmintyje " resursa
                todo!();
                self.section += 1;
                return(None, some(resource), None);
            }

            12 => { // skaitome komanda
                todo!();
                self.section += 1;
                
            }

            13 => { // tikriname, ar komanda korektiska
                todo!();
            }

            14 => { // metame error, jeigu nekorektiska
                let resource = new Resource(RES_LINE_IN_MEM, "eERROR: Wrong command!", PID_PRINT_LINE);
                self.section = 0;
                return(None, some(resource),None);
            }

            15 => { // tikriname, ar tai yra HALT, jei ne, griztame i 12, kitu atveju i 16
                todo!();

            }

            16 => { // atlaisviname "Uzduoties programa supervizorineje atmintyje" resursa
                todo!();
                self.section = 0;
            }

        }
    }
}
