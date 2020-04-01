use crate::types::Word;
use crate::consts::{KERNEL_MEMORY_SIZE, USER_MEMORY_SIZE};

#[derive(Debug)]
pub struct MemoryManagementUnit {
    kernel_memory: Vec<Word>,
    user_memory: Vec<Word>,
}

impl MemoryManagementUnit {
    pub fn new() -> MemoryManagementUnit {
        MemoryManagementUnit {
            kernel_memory: [Word::new(); KERNEL_MEMORY_SIZE].to_vec(),
            user_memory: [Word::new(); USER_MEMORY_SIZE].to_vec(),
        }
    }
}