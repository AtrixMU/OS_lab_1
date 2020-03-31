use project::real_machine::processor::Processor;
use project::virtual_machine::vm_processor::Virtual_Processor;

fn main() {
    println!("Hello, world!");
    let test = Processor::new();
    let test_vm =Virtual_Processor::new();
    println!("{:?}", test);
    println!("{:?}",test_vm);
}
