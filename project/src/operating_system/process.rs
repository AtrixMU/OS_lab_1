use super::resource::Resource;

pub trait Process {
    fn get_state(&self) -> usize;
    fn set_state(&mut self, state: usize);
    fn get_vm(&self) -> usize;
    fn set_vm(&mut self, vm: usize);
    fn add_resource(&mut self, res: Resource);
    fn take_resource(&mut self, resource_index: usize) -> Resource;
    fn progress(&mut self) -> (Option<usize>, Option<Resource>);
}