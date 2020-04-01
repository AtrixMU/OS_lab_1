use crate::types::Word;

#[derive(Debug)]
pub struct MemoryManagementUnit {
    kernel_memory: Vec<Word>,
    kernel_size: u32,
    user_memory: Vec<Word>,
    user_size: u32,
}

impl MemoryManagementUnit {
    pub fn new() -> MemoryManagementUnit {
        MemoryManagementUnit {
            kernel_memory: Vec::new(),
            kernel_size: 16 * 16,
            user_memory: Vec::new(),
            user_size: 48 * 16,
        }
    }
}