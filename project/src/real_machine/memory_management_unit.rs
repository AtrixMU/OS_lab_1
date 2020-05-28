//Matas Udris, Robertas Povedionok 4 grupe, informatika

use rand::prelude::*;
use crate::types::Word;
use crate::consts::*;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;


#[derive(Debug)]
pub struct MemoryManagementUnit {
    pub kernel_memory: Vec<Word>,
    user_memory: Vec<Word>,
    hard_drive: Vec<Word>,
    open_files: HashMap<u32, usize>
}

impl MemoryManagementUnit {
    pub fn new() -> MemoryManagementUnit {
        let mut mmu = MemoryManagementUnit {
            kernel_memory: [Word::new(); KERNEL_MEMORY_SIZE].to_vec(),
            user_memory: [Word::new(); USER_MEMORY_SIZE].to_vec(),
            hard_drive: Vec::new(),
            open_files: HashMap::new(),
        };
        mmu.mount_drive();
        mmu.print_hard_drive();
        mmu
    }
    pub fn mount_drive(&mut self) {
        let mut file = File::open("./disk/disk.dsk").expect("Error opening hard drive");
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
                    print!("{:3}", self.user_memory[i * PAGE_SIZE + j]
                        .get_byte(b)
                        .expect("Error getting byte in print_user_memory")
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
                        .expect("Failed getting byte @ print_virtual_memory")
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
                if i >= DATA_PAGES && (i - DATA_PAGES) * PAGE_SIZE + j < commands.len() {
                    let word = format!("{: >4}", commands[(i - DATA_PAGES) * PAGE_SIZE + j]);
                    for b in word.chars() {
                        print!("{: >3}", b);
                    }
                }
                else {
                    for b in 0..4 {
                        print!("{:3}", self.get_word(ptr, (i * PAGE_SIZE + j) as u32)
                            .get_byte(b)
                            .expect("Failed getting byte @ print_virtual_memory_words")
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
        let mut i = DATA_PAGES;
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
        let word = self.get_word(ptr, (*i * PAGE_SIZE + *j) as u32).as_text().expect("error parsing cmd");
        commands.push(word.clone());
        *j += 1;
        *i += *j / PAGE_SIZE;
        *j %= PAGE_SIZE;
        if "HALT" == word {
            return (commands, true);
        }
        if word.chars().last().expect("error parsing cmd") == 'R' || ["LOAD", "STOR"].contains(&word.as_str()) {
            let word = self.get_word(ptr, (*i * PAGE_SIZE + *j) as u32).as_text().expect("error parsing cmd");
            commands.push(word.clone());
            *j += 1;
            *i += *j / PAGE_SIZE;
            *j %= PAGE_SIZE;
            let word = self.get_word(ptr, (*i * PAGE_SIZE + *j) as u32).as_text().expect("error parsing cmd");
            commands.push(word.clone());
            *j += 1;
            *i += *j / PAGE_SIZE;
            *j %= PAGE_SIZE;
            return (commands, false);
        }
        if word.chars().last().expect("error parsing cmd") == 'V' || word == "LOOP" || word == "MOVN" {
            let word = self.get_word(ptr, (*i * PAGE_SIZE + *j) as u32).as_text().expect("error parsing cmd");
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
        let word = self.get_word(ptr, (*i * PAGE_SIZE + *j) as u32).as_text().expect("error parsing cmd");
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
                    file_name.push_str(&w.as_text().expect("Error parsing file name"));
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
        let disk_cmd_list_page = disk_cmd_list_page_result.expect("Failed to get disk_cmd_list_page_result");
        println!("code address: {}", disk_cmd_list_page);

        let mem_cmd_list_page_res = self.get_first_empty_user_mem_page();
        if mem_cmd_list_page_res.is_none() {
            return None;
        }
        let mem_cmd_list_page = mem_cmd_list_page_res.expect("Failed to get mem_cmd_list_page_res");
        self.user_memory[mem_cmd_list_page as usize * PAGE_SIZE].set_value(1);
        let mut mem_cmd_page = self.get_first_empty_user_mem_page().expect("failed to get_first_empty_user_mem_page");
        let mut disk_cmd_page =
            self.hard_drive[disk_cmd_list_page as usize * PAGE_SIZE].as_u32();
        self.write_to_user_mem_page(
            mem_cmd_list_page as usize,
            0,
            Word::from_u32(mem_cmd_page)
        );
        let mut mem_counter = 0;
        let mut disk_counter = 0;

        // Load data segment into first page
        loop {
            let cmd = self.hard_drive[(disk_cmd_page as usize * PAGE_SIZE) + disk_counter % PAGE_SIZE];
            println!("{} {}", mem_counter, cmd.as_u32());
            if cmd.as_text().is_ok() {
                if cmd.as_text().expect("Unexpected") == "#COD" {
                    disk_counter += 1;
                    mem_counter = PAGE_SIZE * DATA_PAGES;
                    break;
                }
                if cmd.as_text().expect("Unexpected") == "#DAT" {
                    disk_counter += 1;
                    continue;
                }
            }
            if disk_counter % PAGE_SIZE == 0 {
                disk_cmd_page = self.hard_drive[
                    (disk_cmd_list_page as usize * PAGE_SIZE) + disk_counter
                    ]
                    .as_u32();
            }
            if mem_counter % PAGE_SIZE == 0 && mem_counter > 0 {
                if mem_counter / PAGE_SIZE == DATA_PAGES {
                    break;
                }
                mem_cmd_page = self.get_first_empty_user_mem_page().expect("Failed get_first_empty_user_mem_page");
                self.write_to_user_mem_page(
                    mem_cmd_list_page as usize,
                    mem_counter / PAGE_SIZE,
                    Word::from_u32(mem_cmd_page)
                );
            }            
            self.write_to_user_mem_page(mem_cmd_page as usize, mem_counter % PAGE_SIZE, cmd);
            mem_counter += 1;
            disk_counter += 1;
        }

        // Load code segment into other pages
        loop {
            if disk_counter % PAGE_SIZE == 0 {
                disk_cmd_page = self.hard_drive[
                    (disk_cmd_list_page as usize * PAGE_SIZE) + disk_counter / PAGE_SIZE
                    ]
                    .as_u32();
            }
            let cmd = self.hard_drive[(disk_cmd_page as usize * PAGE_SIZE) + disk_counter % PAGE_SIZE];
            println!("{} {}", mem_counter, cmd.as_u32());
            
            if mem_counter % PAGE_SIZE == 0 {
                mem_cmd_page = self.get_first_empty_user_mem_page().expect("Failed get_first_empty_user_mem_page");
                self.write_to_user_mem_page(
                    mem_cmd_list_page as usize,
                    mem_counter / PAGE_SIZE,
                    Word::from_u32(mem_cmd_page)
                );
            }

            self.write_to_user_mem_page(mem_cmd_page as usize, mem_counter % PAGE_SIZE, cmd);
            if cmd.as_text().is_ok() {
                if cmd.as_text().unwrap() == "HALT" {
                    break;
                }
            }
            disk_counter += 1;
            mem_counter += 1;
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
                            (header_page as usize * PAGE_SIZE) + PAGE_SIZE - 1
                        ]
                        .as_u32()
                    );
                }
            }
        }
        None
    }
    fn find_file_start(&self, program_name: String) -> Option<u32> {
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
                    return Some(i as u32);
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
    fn get_first_empty_kernel_mem_page(&mut self) -> Option<u32> {
        let mut rng = rand::thread_rng();
        loop {
            let rand_int: usize = rng.gen_range(0, self.kernel_memory.len() / PAGE_SIZE);
            if self.kernel_memory[rand_int * PAGE_SIZE].is_empty() {
                return Some(rand_int as u32);
            }
        }
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
    pub fn store_word(&mut self, ptr: u32, value: u32) -> u32 {
        let page_addr = self.user_memory[ptr as usize * PAGE_SIZE].as_u32();
        for i in 0..PAGE_SIZE {
            if self.user_memory[(page_addr as usize * PAGE_SIZE) + i].as_u32() == 0 {
                self.user_memory[(page_addr as usize * PAGE_SIZE) + i].set_value(value);
                return i as u32;
            }
        }
        panic!();
    }
    pub fn open_file(&mut self, file_name: String, process_id: usize) -> (u32, u8) {
        let found_file = self.find_file_start(file_name.clone());
        if found_file.is_some() {
            let file_ptr = found_file.expect("but file was found :(");
            if self.open_files.contains_key(&file_ptr) {
                return (0, INT_FILE_OCCUPIED);
            }
            self.open_files.insert(file_ptr, process_id);
            return (file_ptr, 0);
        }
        else {
            let file_ptr = self.create_file(file_name);
            self.open_files.insert(file_ptr, process_id);
            return (file_ptr, 0);
        }
    }
    fn create_file(&mut self, file_name: String) -> u32 {
        let page_start = 1;
        let mut file_ptr = 0;
        let mut header = 0;
        for i in (page_start * PAGE_SIZE)..(DRIVE_SIZE * PAGE_SIZE) {
            if self.hard_drive[i].is_empty() {
                self.hard_drive[i].set_value(1);
                file_ptr = i;
                header = self.get_first_empty_disk_page();
                self.hard_drive[i].set_value(header);
                break;
            }
        }
        if header == 0 || file_ptr == 0 {
            panic!();
        }
        self.hard_drive[header as usize].set_text(file_name[0..4].to_string());
        let block_list = self.get_first_empty_disk_page();
        self.hard_drive[header as usize + PAGE_SIZE - 1].set_value(block_list);
        self.hard_drive[block_list as usize].set_value(1);
        let block_1 = self.get_first_empty_disk_page();
        self.hard_drive[block_list as usize].set_value(block_1);
        println!("Created file {} {} {}", header, block_list, block_1);
        file_ptr as u32
    }
    fn get_first_empty_disk_page(&mut self) -> u32 {
        let mut i = DRIVE_SIZE * PAGE_SIZE;
        while self.hard_drive.len() < i {
            self.hard_drive.push(Word::new());
        }
        loop {
            if i >= self.hard_drive.len() {
                for _ in 0..PAGE_SIZE {
                    self.hard_drive.push(Word::new());
                }
                return i as u32;
            }
            if self.hard_drive[i].is_empty() {
                return i as u32;
            }
            i += PAGE_SIZE;
        }
    }
    pub fn read_from_file(&self, file_ptr: u32, cursor: u32, process_id: usize) -> (u32, u8) {
        if !self.open_files.contains_key(&file_ptr) {
            return (0, INT_BAD_FILE);
        }
        if self.open_files[&file_ptr] != process_id  {
            return (0, INT_FILE_OCCUPIED);
        }
        let header = self.hard_drive[file_ptr as usize].as_u32();
        let blocks = self.hard_drive[header as usize + PAGE_SIZE - 1].as_u32();
        let page = cursor as usize / PAGE_SIZE;
        for i in 0..(page + 1) {
            if self.hard_drive[blocks as usize + i].is_empty() {
                return (0, INT_BAD_ADR);
            }
        }
        let block_page = self.hard_drive[blocks as usize + page].as_u32();
        return (self.hard_drive[block_page as usize + (cursor as usize % PAGE_SIZE)].as_u32(), 0);
    }
    pub fn write_to_file(&mut self, file_ptr: u32, cursor: u32, value: u32, process_id: usize) -> u8 {
        if !self.open_files.contains_key(&file_ptr) {
            return INT_BAD_FILE;
        }
        if self.open_files[&file_ptr] != process_id  {
            return INT_FILE_OCCUPIED;
        }
        let header = self.hard_drive[file_ptr as usize].as_u32();
        let blocks = self.hard_drive[header as usize + PAGE_SIZE - 1].as_u32();
        let page = cursor as usize / PAGE_SIZE;
        for i in 0..page {
            if self.hard_drive[blocks as usize + i].is_empty() {
                return INT_BAD_ADR;
            }
        }
        if self.hard_drive[blocks as usize + page].is_empty() {
            let new_page = self.get_first_empty_disk_page();
            self.hard_drive[blocks as usize + page].set_value(new_page);
        }
        let block_page = self.hard_drive[blocks as usize + page].as_u32();
        self.hard_drive[block_page as usize + (cursor as usize % PAGE_SIZE)].set_value(value);
        0
    }
    pub fn close_file(&mut self, file_ptr: u32, process_id: usize) -> u8 {
        if !self.open_files.contains_key(&file_ptr) {
            return INT_BAD_FILE;
        }
        if self.open_files[&file_ptr] != process_id  {
            return INT_FILE_OCCUPIED;
        }
        self.open_files.remove(&file_ptr);

        0
    }
    pub fn delete_file(&mut self, file_ptr: u32, process_id: usize) -> u8 {
        if !self.open_files.contains_key(&file_ptr) {
            return INT_BAD_FILE;
        }
        if self.open_files[&file_ptr] != process_id  {
            return INT_FILE_OCCUPIED;
        }
        let header = self.hard_drive[file_ptr as usize].as_u32();
        let blocks = self.hard_drive[header as usize].as_u32();
        for i in 0..PAGE_SIZE {
            if !self.hard_drive[blocks as usize + i].is_empty() {
                let block_page = self.hard_drive[blocks as usize + i].as_u32();
                for i in 0..PAGE_SIZE {
                    self.hard_drive[block_page as usize + i].set_value(0);
                }
                self.hard_drive[blocks as usize + i].set_value(0);
            }
        }
        for i in 0..PAGE_SIZE {
            self.hard_drive[header as usize + i].set_value(0);
        }
        self.hard_drive[file_ptr as usize].set_value(0);
        self.open_files.remove(&file_ptr);
        0
    }
    pub fn get_file_data(&self, file_name: String) -> Vec<Word> {
        let mut file_data = Vec::new();
        let file_start = self.find_file_start(file_name).unwrap();
        let header_page = self.hard_drive[file_start as usize].as_u32() as usize;
        for i in 0..(PAGE_SIZE - 1) {
            file_data.push(self.hard_drive[(header_page as usize * PAGE_SIZE) + i]);
        }
        file_data.push(Word::new());
        let block_list = self.hard_drive[(header_page as usize * PAGE_SIZE) + PAGE_SIZE - 1].as_u32();
        let mut last_word = String::new();
        let mut cmd_index = 0;
        let mut current_block = 0;
        while last_word != String::from("HALT") {
            if cmd_index % PAGE_SIZE == 0 {
                current_block = self.hard_drive[(block_list as usize * PAGE_SIZE) + cmd_index / PAGE_SIZE].as_u32() as usize;
            }
            if current_block == 0 {
                panic!();
            }
            let cmd = self.hard_drive[(current_block * PAGE_SIZE) + (cmd_index % PAGE_SIZE)];
            file_data.push(cmd);
            if cmd.as_text().is_ok() {
                last_word = cmd.as_text().unwrap();
            }
            cmd_index += 1;
        }
        file_data
    }
    pub fn upload_to_smem(&mut self, file_data: &Vec<Word>) -> u32 {
        let ptr = self.get_first_empty_kernel_mem_page().unwrap();
        for i in 0..(PAGE_SIZE - 1) {
            self.kernel_memory[(ptr as usize * PAGE_SIZE) + i].set_word(file_data[i]);
        }
        let block_list = self.get_first_empty_kernel_mem_page().unwrap();
        self.kernel_memory[(ptr as usize * PAGE_SIZE) + PAGE_SIZE - 1].set_value(block_list);
        let mut current_block = 0;
        for (cmd_index, word) in file_data.iter().enumerate() {
            if cmd_index % PAGE_SIZE == 0 {
                self.kernel_memory[(block_list as usize * PAGE_SIZE) + cmd_index / PAGE_SIZE].set_value(1);
                current_block = self.get_first_empty_kernel_mem_page().unwrap();
                self.kernel_memory[(block_list as usize * PAGE_SIZE) + cmd_index / PAGE_SIZE].set_value(current_block);
            }
            if current_block == 0 {
                panic!();
            }
            self.kernel_memory[(current_block as usize * PAGE_SIZE) + (cmd_index % PAGE_SIZE)].set_word(*word);
        }
        ptr
    }
    pub fn smem_to_umem(&mut self, kernel_ptr: usize) -> u32 {
        let ptr = self.get_first_empty_user_mem_page().unwrap();
        self.user_memory[ptr as usize * PAGE_SIZE].set_value(1);

        let block_list = self.kernel_memory[(kernel_ptr * PAGE_SIZE) + PAGE_SIZE - 1].as_u32() as usize;
        let mut current_block = self.kernel_memory[block_list as usize * PAGE_SIZE].as_u32() as usize;

        let mut umem_counter = 0;
        let mut smem_counter = 0;
        let mut umem_cmd_page = 0;

        // Load data segment into first page
        loop {
            let cmd = self.kernel_memory[(current_block * PAGE_SIZE) + smem_counter % PAGE_SIZE];
            if cmd.as_text().is_ok() {
                if cmd.as_text().expect("Unexpected") == "#COD" {
                    umem_counter += 1;
                    smem_counter = PAGE_SIZE * DATA_PAGES;
                    break;
                }
                if cmd.as_text().expect("Unexpected") == "#DAT" {
                    smem_counter += 1;
                    continue;
                }
            }
            if smem_counter % PAGE_SIZE == 0 {
                current_block = self.kernel_memory[(block_list as usize * PAGE_SIZE) + smem_counter / PAGE_SIZE].as_u32() as usize;
            }
            if umem_counter % PAGE_SIZE == 0 && umem_counter > 0 {
                if umem_counter / PAGE_SIZE == DATA_PAGES {
                    break;
                }
                umem_cmd_page = self.get_first_empty_user_mem_page().expect("Failed get_first_empty_user_mem_page");
                
            }            
            self.write_to_user_mem_page(umem_cmd_page as usize, umem_counter % PAGE_SIZE, cmd);
            umem_counter += 1;
            smem_counter += 1;
        }
        // Load code segment into other pages
        loop {
            if smem_counter % PAGE_SIZE == 0 {
                current_block = self.kernel_memory[
                    (block_list as usize * PAGE_SIZE) + smem_counter / PAGE_SIZE
                ].as_u32() as usize;
            }
            let cmd = self.kernel_memory[(current_block * PAGE_SIZE) + smem_counter % PAGE_SIZE];
            
            if umem_counter % PAGE_SIZE == 0 {
                umem_cmd_page = self.get_first_empty_user_mem_page().expect("Failed get_first_empty_user_mem_page");
                self.write_to_user_mem_page(
                    ptr as usize,
                    umem_counter / PAGE_SIZE,
                    Word::from_u32(umem_cmd_page)
                );
            }

            self.write_to_user_mem_page(umem_cmd_page as usize, umem_counter % PAGE_SIZE, cmd);
            if cmd.as_text().is_ok() {
                if cmd.as_text().unwrap() == "HALT" {
                    break;
                }
            }
            smem_counter += 1;
            umem_counter += 1;
        }
        ptr
    }
}
