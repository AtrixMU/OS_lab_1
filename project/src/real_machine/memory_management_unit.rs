//Matas Udris, Robertas Povedionok 4 grupe, informatika

use rand::prelude::*;
use crate::types::Word;
use crate::consts::*;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct MemoryManagementUnit {
    kernel_memory: Vec<Word>,
    user_memory: Vec<Word>,
    hard_drive: Vec<Word>,
}

impl MemoryManagementUnit {
    pub fn new() -> MemoryManagementUnit {
        let mut mmu = MemoryManagementUnit {
            kernel_memory: [Word::new(); KERNEL_MEMORY_SIZE].to_vec(),
            user_memory: [Word::new(); USER_MEMORY_SIZE].to_vec(),
            hard_drive: Vec::new(),
        };
        mmu.mount_drive();
        mmu.print_hard_drive();
        mmu
    }
    pub fn mount_drive(&mut self) {
        let mut file = File::open("./disk/disk.dsk").unwrap();
        let mut buffer = [0u8; 4];
    
        loop {
            match file.read(&mut buffer) {
                Ok(count) => {
                    if count == 0 {
                        break;
                    }
                    self.hard_drive.push(Word::from_bytes(buffer));
                },
                Err(_) => panic!(),
            }
        }
    }
    pub fn print_hard_drive(&self) {
        println!("{:-<130}{:-<150}", "", "HARD DRIVE MEMORY");
        for i in 0..(self.hard_drive.len() / PAGE_SIZE) {
            print!("PAGE {:2}: ", i);
            for j in 0..PAGE_SIZE {
                for b in 0..4 {
                    print!("{:3} ", self.hard_drive[i * PAGE_SIZE + j]
                        .get_byte(b)
                        .unwrap()
                    );
                }
                print!("|");
            }
            println!("");
        }
        println!("{:-<281}", "");  
    }
    pub fn print_user_memory(&self) {
        println!("{:-<130}{:-<150}", "", "USER MEMORY");
        for i in 0..(self.user_memory.len() / PAGE_SIZE) {
            print!("PAGE {:2}: ", i);
            for j in 0..PAGE_SIZE {
                for b in 0..4 {
                    print!("{:3} ", self.user_memory[i * PAGE_SIZE + j]
                        .get_byte(b)
                        .unwrap()
                    );
                }
                print!("|");
            }
            println!("");
        }
        println!("{:-<281}", "");
    }
    pub fn print_virtual_memory(&self, ptr: u32) {
        println!("{:-<130}{:-<150}", "", "VIRTUAL MEMORY");
        for i in 0..PAGE_SIZE {
            print!("PAGE {:2}: ", i);
            for j in 0..PAGE_SIZE {
                for b in 0..4 {
                    print!("{:3} ", self.get_word(ptr, (i * PAGE_SIZE + j) as u32)
                        .get_byte(b)
                        .unwrap()
                    );
                }
                print!("|");
            }
            println!("");
        }
        println!("{:-<281}", "");
    }

    pub fn print_virtual_memory_words(&self, ptr: u32) {
        let commands = self.parse_virtual_memory(ptr);
        println!("{:-<130}{:-<150}", "", "VIRTUAL MEMORY");
        for i in 0..PAGE_SIZE {
            print!("PAGE {:2}: ", i);
            for j in 0..PAGE_SIZE {
                if i * PAGE_SIZE + j < commands.len() {
                    let word = format!("{: >4}", commands[i * PAGE_SIZE + j]);
                    for b in word.chars() {
                        print!("{: >3} ", b);
                    }
                }
                else {
                    for b in 0..4 {
                        print!("{:3} ", self.get_word(ptr, (i * PAGE_SIZE + j) as u32)
                            .get_byte(b)
                            .unwrap()
                        );
                    }
                }
                print!("|");
            }
            println!("");
        }
        println!("{:-<281}", "");

    }

    fn parse_virtual_memory(&self, ptr: u32) -> Vec::<String> {
        let mut commands: Vec<String> = Vec::new();
        let mut i = 0;
        let mut j = 0;
        let mut halt_received = false;
        while !halt_received {
            let (mut cmds, was_halt) = self.parse_cmd(ptr, &mut i, &mut j);
            halt_received = was_halt;
            commands.append(&mut cmds);
        }
        commands
    }

    fn parse_cmd(&self, ptr: u32, i: &mut usize, j: &mut usize) -> (Vec<String>, bool) {
        let mut commands: Vec<String> = Vec::new();
        let word = self.get_word(ptr, (*i * PAGE_SIZE + *j) as u32).as_text().unwrap();
        commands.push(word.clone());
        *j += 1;
        *i += *j / PAGE_SIZE;
        *j %= PAGE_SIZE;
        if "HALT" == word {
            return (commands, true);
        }
        if word.chars().last().unwrap() == 'R' {
            let word = self.get_word(ptr, (*i * PAGE_SIZE + *j) as u32).as_text().unwrap();
            commands.push(word.clone());
            *j += 1;
            *i += *j / PAGE_SIZE;
            *j %= PAGE_SIZE;
            let word = self.get_word(ptr, (*i * PAGE_SIZE + *j) as u32).as_text().unwrap();
            commands.push(word.clone());
            *j += 1;
            *i += *j / PAGE_SIZE;
            *j %= PAGE_SIZE;
            return (commands, false);
        }
        if word.chars().last().unwrap() == 'V' || word == "LOOP" || word == "MOVN" {
            let word = self.get_word(ptr, (*i * PAGE_SIZE + *j) as u32).as_text().unwrap();
            commands.push(word.clone());
            *j += 1;
            *i += *j / PAGE_SIZE;
            *j %= PAGE_SIZE;
            let word = format!("{}", self.get_word(ptr, (*i * PAGE_SIZE + *j) as u32).as_u32());
            commands.push(word.clone());
            *j += 1;
            *i += *j / PAGE_SIZE;
            *j %= PAGE_SIZE;
            return (commands, false);
        }
        if ["JUMP", "JPEQ", "JPOF", "JPGE", "JPBE", "JMPG", "JMPB"].contains(&word.as_str()) {
            let word = format!("{}", self.get_word(ptr, (*i * PAGE_SIZE + *j) as u32).as_u32());
            commands.push(word.clone());
            *j += 1;
            *i += *j / PAGE_SIZE;
            *j %= PAGE_SIZE;
            return (commands, false);
        }
        let word = self.get_word(ptr, (*i * PAGE_SIZE + *j) as u32).as_text().unwrap();
        commands.push(word.clone());
        *j += 1;
        *i += *j / PAGE_SIZE;
        *j %= PAGE_SIZE;
        (commands, false)
    }

    pub fn list_programs(&self) {
        let page_start = 1;
        for i in (page_start * PAGE_SIZE)..(DRIVE_SIZE * PAGE_SIZE) {
            if !self.hard_drive[i].is_empty() {
                let header_page = self.hard_drive[i].as_u32();
                let mut file_name = String::new();
                let mut cursor = 0;
                for j in cursor..(cursor + FILE_NAME_LEN) {
                    let w = self.hard_drive[(header_page as usize * PAGE_SIZE) + j];
                    if w.is_empty() {
                        break;
                    }
                    file_name.push_str(&w.as_text().unwrap());
                }
                cursor += FILE_NAME_LEN;
                file_name.push('.');
                for j in cursor..(cursor + FILE_EXTENSION_LEN) {
                    let w = self.hard_drive[(header_page as usize * PAGE_SIZE) + j];
                    if w.is_empty() {
                        break;
                    }
                    file_name.push_str(&w.as_text().unwrap());
                }
                println!("Program found: {}", file_name);
                cursor += FILE_EXTENSION_LEN;
                let mut file_type = String::new();
                for j in cursor..(cursor + FILE_TYPE_LEN) {
                    let w = self.hard_drive[(header_page as usize * PAGE_SIZE) + j];
                    if w.is_empty() {
                        break;
                    }
                    file_type.push_str(&w.as_text().unwrap());
                }
                println!("File type: {}", file_type);
            }
        }
    }
    pub fn load_program(&mut self, program_name: String) -> Option<u32> {
        let disk_cmd_list_page_result = self.get_program_code_start(program_name);
        if disk_cmd_list_page_result.is_none() {
            return None;
        }
        let disk_cmd_list_page = disk_cmd_list_page_result.unwrap();
        println!("code address: {}", disk_cmd_list_page);

        let mem_cmd_list_page_res = self.get_first_empty_user_mem_page();
        if mem_cmd_list_page_res.is_none() {
            return None;
        }
        let mem_cmd_list_page = mem_cmd_list_page_res.unwrap();
        self.user_memory[mem_cmd_list_page as usize * PAGE_SIZE].set_value(1);
        let mut mem_cmd_page = self.get_first_empty_user_mem_page().unwrap();
        let mut disk_cmd_page =
            self.hard_drive[disk_cmd_list_page as usize * PAGE_SIZE].as_u32();
        let mut page_number = 0;
        self.write_to_user_mem_page(
            mem_cmd_list_page as usize,
            page_number,
            Word::from_u32(mem_cmd_page)
        );
        let mut counter = 0;
        loop {
            let cmd = self.hard_drive[(disk_cmd_page as usize * PAGE_SIZE) + counter];
            println!("{} {}", counter, cmd.as_u32());
            if counter == PAGE_SIZE {
                page_number += 1;
                counter = 0;
                mem_cmd_page = self.get_first_empty_user_mem_page().unwrap();
                disk_cmd_page = self.hard_drive[
                    (disk_cmd_list_page as usize * PAGE_SIZE) + page_number
                ]
                    .as_u32();
                self.write_to_user_mem_page(
                    mem_cmd_list_page as usize,
                    page_number,
                    Word::from_u32(mem_cmd_page)
                );
            }
            self.write_to_user_mem_page(mem_cmd_page as usize, counter, cmd);
            if cmd.as_text().is_ok() {
                if cmd.as_text().unwrap() == "HALT" {
                    break;
                }
            }
            counter += 1;
        }
        Some(mem_cmd_list_page)
    }
    pub fn get_program_code_start(&self, program_name: String) -> Option<u32> {
        let page_start = 1;
        for i in (page_start * PAGE_SIZE)..(DRIVE_SIZE * PAGE_SIZE) {
            if !self.hard_drive[i].is_empty() {
                let header_page = self.hard_drive[i].as_u32();
                let mut file_name = String::new();
                for j in 0..FILE_NAME_LEN {
                    let w = self.hard_drive[(header_page as usize * PAGE_SIZE) + j];
                    if w.is_empty() {
                        break;
                    }
                    file_name.push_str(&w.as_text().unwrap());
                }
                if file_name == program_name {
                    return Some(
                        self.hard_drive[
                            (header_page as usize * PAGE_SIZE) + 15
                        ]
                        .as_u32()
                    );
                }
            }
        }
        None
    }
    pub fn unload_program(&mut self, ptr: u32) {
        let mem_cmd_list_page_index = ptr as usize * PAGE_SIZE;
        for i in 0..PAGE_SIZE {
            let cmd_page_list_index = mem_cmd_list_page_index + i;
            let cmd_list_index = self.user_memory[cmd_page_list_index].as_u32();
            for j in 0..PAGE_SIZE {
                if self.user_memory[
                    (cmd_list_index as usize * PAGE_SIZE) + j
                    ]
                    .as_text()
                    .is_ok()
                    && self.user_memory[
                        (cmd_list_index as usize * PAGE_SIZE) + j
                        ]
                        .as_text()
                        .unwrap() == "HALT".to_string() {
                    self.user_memory[(cmd_list_index as usize * PAGE_SIZE) + j].set_value(0);
                    self.user_memory[cmd_page_list_index].set_value(0);
                    return;
                }
                self.user_memory[(cmd_list_index as usize * PAGE_SIZE) + j].set_value(0);
            }
            self.user_memory[cmd_page_list_index].set_value(0);
        }
    }
    fn get_first_empty_user_mem_page(&mut self) -> Option<u32> {
        let mut rng = rand::thread_rng();
        loop {
            let rand_int: usize = rng.gen_range(0, self.user_memory.len() / PAGE_SIZE);
            if self.user_memory[rand_int * PAGE_SIZE].is_empty() {
                return Some(rand_int as u32);
            }
        }
        // for i in 0..self.user_memory.len() / PAGE_SIZE {
        //     if self.user_memory[i * PAGE_SIZE].is_empty() {
        //         return Some(i as u32);
        //     }
        // }
        // return None;
    }
    fn write_to_user_mem_page(
        &mut self,
        page_index: usize,
        word_index: usize,
        input: Word
    ) {
        if word_index >= PAGE_SIZE {
            panic!();
        }
        let w = &mut self.user_memory[
            (page_index * PAGE_SIZE) + word_index
        ];
        w.set_word(input);
    }

    pub fn get_word(&self, ptr: u32, ic: u32) -> Word {
        let page_i = ic as usize / PAGE_SIZE;
        let word_i = ic as usize % PAGE_SIZE;
        let page_addr = self.user_memory[(ptr as usize * PAGE_SIZE) + page_i].as_u32();
        if page_addr == 0 {
            return Word::new();
        }
        let word = self.user_memory[(page_addr as usize * PAGE_SIZE) + word_i];
        word
    }
}