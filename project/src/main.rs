use project::real_machine::processor::RMProcessor;
use project::traits::Processor;

fn main() {
    println!("Hello, world!");
    let test = RMProcessor::new();
    println!("{:?}", test);
    println!("{:?}", test.get_zero_flag());
}
