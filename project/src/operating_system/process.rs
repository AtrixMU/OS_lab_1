use crate::virtual_machine::processor::VMProcessor;
use super::resource::Resource;

pub trait Process {
    fn get_state(&self) -> usize;
    fn set_state(&mut self, state: usize);
    fn get_vm(&self) -> VMProcessor;
    fn set_vm(&mut self, vm: VMProcessor);
    fn add_resource(&mut self, res: Resource);
    fn take_resource(&mut self, res: usize) -> Resource;
    fn progress(&mut self) -> Option<(usize, bool)>;
}