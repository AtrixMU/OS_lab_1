//Matas Udris, Robertas Povedionok 4 grupe, informatika
pub const CARRY_FLAG: u16 = 0b0000_0000_0000_0001;
pub const PARITY_FLAG: u16 = 0b0000_0000_0000_0100;
pub const AUXILIARY_CARRY_FLAG: u16 = 0b0000_0000_0001_0000;
pub const ZERO_FLAG: u16 = 0b0000_0000_0100_0000;
pub const SIGN_FLAG: u16 = 0b0000_0000_1000_0000;
pub const TRAP_FLAG: u16 = 0b0000_0001_0000_0000;
pub const INTERRUPT_FLAG: u16 = 0b0000_0010_0000_0000;
pub const DIRECTIONAL_FLAG: u16 = 0b0000_0100_0000_0000;
pub const OVERFLOW_FLAG: u16 = 0b0000_1000_0000_0000;
pub const SUPERVISOR_FLAG: u16 = 0b1000_0000_0000_0000;

pub const KERNEL_MEMORY_SIZE: usize = 16 * 16;
pub const USER_MEMORY_SIZE: usize = 48 * 16;

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

pub const DATA_PAGES: usize = 1;

// Errors
pub const INT_DIV_ZERO: u8 = 1;
pub const INT_OF: u8 = 2;
pub const INT_BAD_ADR: u8 = 3;
pub const INT_BAD_CMD: u8 = 4;
pub const INT_BAD_FILE: u8 = 5;
pub const INT_BAD_DEVICE: u8 = 6;
pub const INT_FILE_OCCUPIED: u8 = 7;
pub const INT_OOM: u8 = 8;

// Interrupts
pub const INT_OPEN: u8 = 5;
pub const INT_READ: u8 = 6;
pub const INT_WRITE: u8 = 7;
pub const INT_CLOSE: u8 = 8;
pub const INT_DEL: u8 = 9;