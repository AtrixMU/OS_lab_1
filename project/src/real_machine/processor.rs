
// The processor struct for our real machine.
// Debug allows us to print the struct using println!("{:?}", struct_name)
#[derive(Debug)]
pub struct Processor {
    ax: u32,
    bx: u32,
    cx: u32,
    dx: u32,
    pi: u8,
    ki: u8,
    ti: u8,
    sr: u16,
    ip: u32,
    ptr: u32,
}

// Methods implemented for the real machine Processor struct
impl Processor {
    // Create new instance with default values
    pub fn new() -> Processor {
        Processor {
            ax: 0,
            bx: 0,
            cx: 0,
            dx: 0,
            pi: 0,
            ki: 0,
            ti: 0,
            sr: 0,
            ip: 0,
            ptr: 0,
        }
    }
}