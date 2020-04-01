use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct WordToTextError;

impl fmt::Display for WordToTextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not convert Word to text")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for WordToTextError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Word {
    data: [u8; 4],
}

impl Word {
    pub fn new() -> Word {
        Word {
            data: [0, 0, 0, 0],
        }
    }
    pub fn get_byte(self, index: usize) -> Option<u8> {
        if index >= 4 {
            return None;
        }
        Some(self.data[index])
    }
    pub fn as_text(self) -> Result<String, WordToTextError> {
        let mut result = String::new();
        for &byte in self.data.iter() {
            if 20 <= byte && byte <= 0x7Eu8 {
                return Err(WordToTextError);
            } 
            result.push(byte as char);
        }
        return Ok(result);
    }
    pub fn set_word(&mut self, input: [u8; 4]) {
        self.data = input;
    }
    pub fn set_value(&mut self, input: u32) {
        self.set_word(self.u32_to_u8_array(input));
    }
    pub fn set_from_text(&mut self, input: String) {
        let mut temp_str = input;
        while temp_str.len() < 4  {
            temp_str.push(0 as char);
        }
        for (byte, source_byte) in self.data.iter_mut().zip(temp_str[0..4].bytes()) {
            *byte = source_byte;
        }
    }
    pub fn u32_to_u8_array(self, x: u32) -> [u8; 4] {
        let b0: u8 = ((x >> 24) & 0xff) as u8;
        let b1: u8 = ((x >> 16) & 0xff) as u8;
        let b2: u8 = ((x >> 8) & 0xff) as u8;
        let b3: u8 = (x & 0xff) as u8;
        return [b0, b1, b2, b3]
    }
}