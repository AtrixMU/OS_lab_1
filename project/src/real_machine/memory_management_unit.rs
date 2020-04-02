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

    pub fn get_word(ptr: u32, ic: u32) -> Word {
        let page_i: = ic / PAGE_SIZE;
        let word_i: = ic % PAGE_SIZE;
        let page_addr = self.user_memory[(ptr * 16 + page_i) as usize].as_u32();
        let word = self.user_memory[(page_addr * 16 + word_i) as usize];
        word
      }
}