use super::resource::Resource;
use crate::real_machine::processor::RMProcessor;

pub trait Process {
    fn get_state(&self) -> usize;
    fn set_state(&mut self, state: usize);
    fn get_vm(&self) -> usize;
    fn set_vm(&mut self, vm: usize);
    fn add_resource(&mut self, res: Resource);
    fn take_resource(&mut self, resource_tpe: usize) -> Resource;
    fn has_resource(&self, resource_type: usize) -> bool;
    fn step(&mut self, rm: &mut RMProcessor) -> (Option<usize>, Option<Resource>, Option<Box<dyn Process>>);
}