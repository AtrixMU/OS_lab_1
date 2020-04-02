#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]

use project::virtual_machine::processor::VMProcessor;
use project::real_machine::processor::RMProcessor;
use project::traits::Processor;

fn main() {
    println!("Hello, world!");

    let mut test = RMProcessor::new();
    // println!("{:?}", test);
    // println!("{:?}", test.get_zero_flag());
    // println!("{:?}", test_vm.get_supervisor_flag());
    // println!("{:?}", 0 as char);
    // let temp = mmu.load_program("Simple_program".to_string()).unwrap();
    // mmu.print_user_memory();
    // mmu.print_virtual_memory(temp);
    // mmu.unload_program(temp);
    // mmu.print_user_memory();
    test.add_program("Simple_program4".to_string());
    test.run_instruction_loop();
}
