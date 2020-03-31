use project::virtual_machine::vm_processor::Virtual_Processor;
use project::real_machine::processor::RMProcessor;
use project::traits::Processor;

fn main() {
    println!("Hello, world!");
    let test_vm =Virtual_Processor::new();
    println!("{:?}",test_vm);
    let test = RMProcessor::new();
    println!("{:?}", test);
    println!("{:?}", test.get_zero_flag());
    println!("{:?}",test_vm.get_supervisor_flag());
}
