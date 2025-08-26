use csa_stack_machine::isa::*;

use std::fs;
use std::env;
use std::io;

pub fn read_programm(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name) 
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: translator <source_file> <out_file>");
        std::process::exit(1);
    }
    let source_file = &args[1];
    let out_file = &args[2];

    println!("{source_file}, {out_file}");

    let program = match read_programm(&source_file) {
        Ok(text) => text, 
        Err(e) => {
            eprintln!("Error reading file {} : {}", source_file, e);
            std::process::exit(1);
        }
    };

    match fs::write(out_file, program.as_bytes()) {
        Ok(_) => {
            println!("Successfully wrote into {}", out_file);
        }
        Err(e) => {
            eprintln!("Error writing into file {} : {}", out_file, e); 
            std::process::exit(1);
        }
    }

}