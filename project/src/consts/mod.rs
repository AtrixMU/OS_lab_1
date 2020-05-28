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

pub const KERNEL_MEMORY_SIZE: usize = 48 * 16;
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
pub const INT_PRTN: u8 = 1;
pub const INT_PRTS: u8 = 2;
pub const INT_GETN: u8 = 3;
pub const INT_GETS: u8 = 4;
pub const INT_OPEN: u8 = 5;
pub const INT_READ: u8 = 6;
pub const INT_WRITE: u8 = 7;
pub const INT_CLOSE: u8 = 8;
pub const INT_DEL: u8 = 9;
pub const INT_HALT: u8 = 0xFF;

// Resource types
pub const RES_S_MEM: usize = 0;             // Supervisor memory ("Supervizorinė atmintis")
pub const RES_U_MEM: usize = 1;             // User memory ("Vartotojo atmintis")
pub const RES_DISK: usize = 2;              // Disk ("Išorinė atmintis")
pub const RES_CHNL: usize = 3;              // Channel device ("Kanalų įrenginys")
pub const RES_TASK_IN_SUPER: usize = 4;     // Task is in supervisor memory ("Užduotis supervizorinėje atmintyje")
pub const RES_FROM_USER_INT: usize = 5;     // From user interface ("Iš vartotojo sąsajos")
pub const RES_FILE_PACK: usize = 6;         // File pack ("Failo paketas")
pub const RES_USER_INPUT: usize = 7;        // User input ("Vartotojo įvedimas")
pub const RES_LINE_IN_MEM: usize = 8;       // Line in memory ("Eilutė atmintyje")
pub const RES_FROM_FILEWORK: usize = 9;     // From FileWork process ("Iš Filework")
pub const RES_INTERRUPT: usize = 10;        // Interrupt ("Petraukimo resursas")
pub const RES_FROM_INTERRUPT: usize = 11;   // From Interrupt process ("Iš interrupt")
pub const RES_THEAD_SUPER: usize = 12;      // Task header in supervisor memory ("Užduoties antraštė supervizorinėje atmintyje")
pub const RES_TPROG_SUPER: usize = 13;      // Task program in supervisor memory ("Užduoties programa supervizorinėje atmintyje")
pub const RES_TASK_IN_USER: usize = 14;     // Task program in user memory ("Užduotis UMem")

// Process states
pub const P_READY: usize = 0;
pub const P_RUNNING: usize = 1;
pub const P_BLOCKED: usize = 2;
pub const P_READY_SUSP: usize = 3;
pub const P_BLOCKED_SUSP: usize = 4;

// Channel consts
pub const CH_USER_MEM: u8 = 0;
pub const CH_SUPER_MEM: u8 = 1;
pub const CH_DISK: u8 = 2;
pub const CH_INPUT: u8 = 3;

// System process ids
pub const PID_STARTSTOP: usize = 0;
pub const PID_READ_FROM_DISK: usize = 1;
pub const PID_JCL: usize = 2;
pub const PID_PRINT_LINE: usize = 3;
pub const PID_JOB_TO_UMEM: usize = 4;