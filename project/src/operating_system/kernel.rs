use super::process::Process;

pub struct Kernel {
    process_list: Vec<Process>,
}