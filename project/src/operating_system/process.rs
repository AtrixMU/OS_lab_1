use crate::virtual_machine::processor::VMProcessor;
use crate::consts::*;

pub struct Process {
    id: usize,
    parent_id: usize,
    vm: VMProcessor,
    state: usize,
}

impl Process {
    pub fn new(id: usize, parent_id: usize, ptr: u32) -> Process {
        Process {
            id: id,
            parent_id: parent_id,
            vm: VMProcessor::new(ptr),
            state: P_READY,
        }
    }
}