use super::process::Process;
use super::resource_list::ResourceList;

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