use project::virtual_machine::processor::VMProcessor;
use project::real_machine::processor::RMProcessor;
use project::traits::Processor;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::{BufRead,BufReader};
use std::io::prelude::*;

fn main() {
    println!("Hello, world!");
    let test_vm = VMProcessor::new();
    println!("{:?}", test_vm);
    let test = RMProcessor::new();
    println!("{:?}", test);
    println!("{:?}", test.get_zero_flag());
    println!("{:?}", test_vm.get_supervisor_flag());
    println!("{:?}", 0 as char);
}
