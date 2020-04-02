use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

use create_drive::types::Word;

pub const PAGE_SIZE: usize = 16;
pub const DISK_NAME_LEN: usize = 8;
pub const MAX_BLOCK_COUNT_LEN: usize = 1;
pub const FREE_BLOCK_COUNT_LEN: usize = 1;
pub const DRIVE_SIZE: usize = 32;

pub struct Metadata {
    file_name: String,
    file_extension: String,
    file_type: String,
    creation_date: String,
    last_modified: String,
    permissions: u32,
    path: String,
}

fn write_to_page(
    disk: &mut Vec<Word>,
    page_index: usize,
    word_index: usize,
    input: Word) {
    let w = &mut disk[page_index * 10 + word_index];
    w.set_word(input);
}


fn main() {
    println!("Hello, world!");
    let mem = [Word::new(); DRIVE_SIZE].to_vec();
    let mut entries = fs::read_dir("./files/metadata/").unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>().unwrap();
    for entry in entries {
        println!("{:?}", entry);
    }
}
