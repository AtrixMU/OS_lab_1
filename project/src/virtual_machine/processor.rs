#[derive(Debug)]
pub struct VMProcessor {
    r1: u32,
    r2: u32,
    r3: u32,
    r4: u32,
    ic: u32,
    sr: u16,
    ptr: u32,
}

impl VMProcessor {
    // Create new instance with default values
    pub fn new() -> VMProcessor {
        VMProcessor {
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            ic: 0,
            sr: 0,
            ptr: 0,
        }
    }
}
