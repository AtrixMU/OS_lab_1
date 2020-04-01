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
            if 0x20u8 <= byte && byte <= 0x7Eu8 {
                result.push(byte as char);
            }
            else if byte == 0 {
                return Ok(result);
            }
            else {
                return Err(WordToTextError);
            }
        }
        return Ok(result);
    }
    pub fn as_u32(self) -> u32 {
        let result = self.data[0] as u32 * 0x1_00_00_00 +
            self.data[1] as u32 * 0x1_00_00 +
            self.data[2] as u32 * 0x1_00 +
            self.data[3] as u32;
        result
    }
    pub fn set_word(&mut self, input: [u8; 4]) {
        self.data = input;
    }
    pub fn set_value(&mut self, input: u32) {
        self.set_word(self.u32_to_u8_array(input));
    }
    pub fn set_text(&mut self, input: String) {
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

#[cfg(test)]
mod processor_tests {
    use crate::types::Word;

    #[test]
    pub fn test_get_byte() {
        let mut w = Word::new();
        w.data[0] = 0;
        w.data[1] = 32;
        w.data[2] = 64;
        w.data[3] = 128;
        assert_eq!(w.get_byte(0).unwrap(), 0);
        assert_eq!(w.get_byte(1).unwrap(), 32);
        assert_eq!(w.get_byte(2).unwrap(), 64);
        assert_eq!(w.get_byte(3).unwrap(), 128);
        assert!(w.get_byte(4).is_none());
    }
    #[test]
    pub fn test_as_text_1() {
        let mut w = Word::new();
        w.data[0] = 0x30u8;
        w.data[1] = 0x31u8;
        w.data[2] = 0x32u8;
        w.data[3] = 0x33u8;
        assert_eq!(w.as_text().unwrap(), "0123");
    }
    #[test]
    pub fn test_as_text_2() {
        let mut w = Word::new();
        w.data[0] = 0x30u8;
        w.data[1] = 0x31u8;
        assert_eq!(w.as_text().unwrap(), "01");
    }
    #[test]
    pub fn test_as_u32_1() {
        let mut w = Word::new();
        w.data[0] = 0x30u8;
        w.data[1] = 0x31u8;
        w.data[2] = 0x32u8;
        w.data[3] = 0x33u8;
        assert_eq!(w.as_u32(), 0x30313233u32);
    }
    #[test]
    pub fn test_as_u32_2() {
        let mut w = Word::new();
        w.data[1] = 0x31u8;
        assert_eq!(w.as_u32(), 0x00310000u32);
    }
    #[test]
    pub fn test_set_word() {
        let mut w = Word::new();
        let input = [1, 2, 3, 5];
        w.set_word(input);
        for (b, i) in w.data.iter().zip(input.iter()) {
            assert_eq!(b, i);
        }
    }
    #[test]
    pub fn test_set_value() {
        let mut w = Word::new();
        let input = 0xFF_03_02_01;
        let output = [0xFF, 3, 2, 1];
        w.set_value(input);
        for (b, i) in w.data.iter().zip(output.iter()) {
            assert_eq!(b, i);
        }
    }
    #[test]
    pub fn test_set_text_1() {
        let mut w = Word::new();
        let input = "TEST".to_string();
        let output = [0x54, 0x45, 0x53, 0x54];
        w.set_text(input);
        for (b, i) in w.data.iter().zip(output.iter()) {
            assert_eq!(b, i);
        }
    }
    #[test]
    pub fn test_set_text_2() {
        let mut w = Word::new();
        let input = "TE".to_string();
        let output = [0x54, 0x45, 0x0, 0x0];
        w.set_text(input);
        for (b, i) in w.data.iter().zip(output.iter()) {
            assert_eq!(b, i);
        }
    }
}