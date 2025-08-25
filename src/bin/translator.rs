use csa_stack_machine::isa::*;
use std::fs;
use std::io;

pub fn read_programm(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name) 
}


fn main() {
    let file_name = "examples/hello.fth";
    let content = match read_programm(&file_name) {
        Ok(content) => content, 
        Err(e) => {
            eprintln!("Error reading file '{}' : '{}'", file_name, e);
            return; 
        }
    };
    println!("{content}");
}