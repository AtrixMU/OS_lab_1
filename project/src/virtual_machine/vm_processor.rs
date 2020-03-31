#[derive(Debug)]
pub struct Virtual_Processor {
    r1: u32,
    r2: u32,
    r3: u32,
    r4: u32,
    ic: u32,
    sr: u16,
    ptr: u32,
}

impl Virtual_Processor {
    // Create new instance with default values
    pub fn new() -> Virtual_Processor {
        Virtual_Processor {
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
