//Matas Udris, Robertas Povedionok 4 grupe, informatika
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use create_drive::types::Word;


pub const PAGE_SIZE: usize = 16;
pub const DISK_NAME_LEN: usize = 8;
pub const MAX_BLOCK_COUNT_LEN: usize = 1;
pub const FREE_BLOCK_COUNT_LEN: usize = 1;
pub const DRIVE_SIZE: usize = 32;
pub const FILE_NAME_LEN: usize = 8;
pub const FILE_EXTENSION_LEN: usize = 1;
pub const FILE_TYPE_LEN: usize = 1;
pub const CREATION_DATE_LEN: usize = 2;
pub const LAST_MODIFIED_LEN: usize = 2;
pub const PERMISSIONS_LEN: usize = 1;

#[derive(Debug, Deserialize)]
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
    meta: Metadata,
) {
    let file_header_page = get_first_empty_block(disk);
    let mut cursor = 0;
    if !insert_file_header_address(disk, file_header_page) {
        panic!();
    }
    let name = string_into_words(meta.file_name);
    for (index, w) in name.iter().take(FILE_NAME_LEN).enumerate() {
        write_to_page(disk, file_header_page as usize, cursor + index, *w);
    }
    cursor += FILE_NAME_LEN;
    let file_extension = string_into_words(meta.file_extension);
    write_to_page(disk, file_header_page as usize, cursor, file_extension[0]);
    cursor += 1;
    let file_type = string_into_words(meta.file_type);
    write_to_page(disk, file_header_page as usize, cursor, file_type[0]);
    cursor += 1;
    let creation_date = string_into_words(meta.creation_date);
    for (index, w) in creation_date.iter().take(CREATION_DATE_LEN).enumerate() {
        write_to_page(disk, file_header_page as usize, cursor + index, *w);
    }
    cursor += CREATION_DATE_LEN;
    let last_modified = string_into_words(meta.last_modified);
    for (index, w) in last_modified.iter().take(LAST_MODIFIED_LEN).enumerate() {
        write_to_page(disk, file_header_page as usize, cursor + index, *w);
    }
    cursor += LAST_MODIFIED_LEN;
    let permissions = meta.permissions;
    write_to_page(
        disk,
        file_header_page as usize,
        cursor,
        Word::from_u32(permissions)
    );
    cursor += PERMISSIONS_LEN;
    let block_list_adr = get_first_empty_block(disk);
    write_to_page(
        disk,
        file_header_page as usize,
        cursor,
        Word::from_u32(block_list_adr)
    );
    write_file_contents(disk, block_list_adr, meta.path);
}

fn write_file_contents(
    disk: &mut Vec<Word>,
    block_list_adr: u32,
    path: String
) {
    let mut block_number = 0;
    let mut counter = 0;
    write_to_page(disk, block_list_adr as usize, 0, Word::from_u32(1));
    let mut cmd_block_adr = get_first_empty_block(disk);
    write_to_page(
        disk,
        block_list_adr as usize,
        block_number,
        Word::from_u32(cmd_block_adr)
    );
    
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(cmd) = line {
                if cmd == "" {
                    continue;
                }
                println!("{} {}", counter, cmd);
                if counter == PAGE_SIZE {
                    block_number += 1;
                    cmd_block_adr = get_first_empty_block(disk);
                    write_to_page(
                        disk,
                        block_list_adr as usize,
                        block_number,
                        Word::from_u32(cmd_block_adr)
                    );
                    counter = 0;
                }
                let number = cmd.parse::<u32>();
                if number.is_ok() {
                    write_to_page(
                        disk,
                        cmd_block_adr as usize,
                        counter,
                        Word::from_u32(number.unwrap())
                    );
                }
                else {
                    write_to_page(
                        disk,
                        cmd_block_adr as usize,
                        counter,
                        Word::from_string(cmd)
                    );
                }
                counter += 1;
            }
        }
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
    (disk.len() / PAGE_SIZE) as u32 - 1
}

fn insert_file_header_address(
    disk: &mut Vec<Word>,
    address: u32
) -> bool {
    for i in PAGE_SIZE..(DRIVE_SIZE*PAGE_SIZE) {
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
    if word_index >= PAGE_SIZE {
        panic!();
    }
    let w = &mut disk[page_index * PAGE_SIZE + word_index];
    w.set_word(input);
}

fn print_disk(disk: &Vec<Word>) {
    for i in 0..(disk.len() / PAGE_SIZE) {
        print!("PAGE {}:    ", i);
        for j in 0..PAGE_SIZE {
            for b in 0..4 {
                print!("{} ", disk[i*PAGE_SIZE + j].get_byte(b).unwrap());
            }
            print!("| ");
        }
        println!("");
    }
}

fn write_to_file(disk: &Vec<Word>, file: &mut File) {
    for i in 0..(disk.len() / PAGE_SIZE) {
        for j in 0..PAGE_SIZE {
                file.write(&disk[i*PAGE_SIZE + j].get_data());
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut mem = [Word::new(); DRIVE_SIZE * PAGE_SIZE].to_vec();
    let mut entries = fs::read_dir("./files/metadata/").unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>().unwrap();
    init_disk_info(&mut mem, "DISK".to_string(), 64);
    for entry in entries {
        println!("{:?}", entry);
        let file = File::open(entry).unwrap();
        let reader = BufReader::new(file);
        // Read the JSON contents of the file as an instance of `User`.
        let u: Metadata = serde_json::from_reader(reader).unwrap();
        println!("{:?}", u);
        add_file(&mut mem, u);
    }
    print_disk(&mem);
    let mut file = File::create("../project/disk/disk.dsk").unwrap();
    write_to_file(&mem, &mut file);
}
