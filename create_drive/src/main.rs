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

fn string_into_words(input: String) -> Vec<Word> {
    let mut inp = input;
    let mut words: Vec<Word> = Vec::new();
    while inp.len() > 4 {
        let substring: String = inp.drain(..4).collect();
        words.push(Word::from_string(substring));
    }
    words.push(Word::from_string(inp));
    words
}

fn init_disk_info(
    disk: &mut Vec<Word>,
    name: String,
    disk_size: u32,
) {
    let words = string_into_words(name);
    for (index, w) in words.iter().take(DISK_NAME_LEN).enumerate() {
        write_to_page(disk, 0, index, *w);
    }
    write_to_page(disk, 0, DISK_NAME_LEN + 1, Word::from_u32(disk_size));
}

fn add_file(
    disk: &mut Vec<Word>,
    meda: &Metadata,
) {
    let file_header_page = get_first_empty_block(disk);
    if !insert_file_header_address(disk, file_header_page) {
        panic!();
    }
}

fn get_first_empty_block(disk: &mut Vec<Word>) -> u32 {
    if disk.len() == DRIVE_SIZE * PAGE_SIZE {
        disk.append(&mut [Word::new(); PAGE_SIZE].to_vec());
        return DRIVE_SIZE as u32;
    }
    for i in DRIVE_SIZE..disk.len() / PAGE_SIZE {
        if disk[i * PAGE_SIZE].is_empty() {
            return i as u32;
        }
    }
    disk.append(&mut [Word::new(); PAGE_SIZE].to_vec());
    (disk.len() / PAGE_SIZE) as u32
}

fn insert_file_header_address(
    disk: &mut Vec<Word>,
    address: u32
) -> bool {
    for i in PAGE_SIZE..DRIVE_SIZE*PAGE_SIZE {
        if disk[i].is_empty() {
            disk[i].set_value(address);
            return true;
        }
    }
    return false;
}

fn write_to_page(
    disk: &mut Vec<Word>,
    page_index: usize,
    word_index: usize,
    input: Word
) {
    let w = &mut disk[page_index * PAGE_SIZE + word_index];
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
