use csa_stack_machine::isa::*;



fn main() {
    let opcode = str_to_opcode("lit").unwrap();
    println!("{:?}", opcode);
}