pub enum Opcode {
    // base operations 
    Lit, 
    Dup, 
    Drop, 
    Swap, 
    Over,
    // arithmetic and logic operations
    Add, 
    Sub, 
    Mul, 
    Div, 
    Mod, 
    And, 
    Or, 
    Eq, 
    Lt, 
    Gr, 
    // memory operations 
    Store, 
    Fetch, 
    Variable, 
    // In/Out put
    In, 
    Out, 
    // Flow control
    If, 
    Then, 
    Call, 
    Begin, 
    Until, 
    // system operations 
    LoadAddr, 
    Ret, 
    Halt,
    Nop,
}


pub fn instr_to_u8(opcode: &Opcode) -> u8 {
    match opcode {
        Opcode::Lit => 0x01, 
        Opcode::Dup => 0x02, 
        Opcode::Drop => 0x03, 
        Opcode::Swap => 0x04, 
        Opcode::Over => 0x05, 

        Opcode::Add => 0x10, 
        Opcode::Sub => 0x11, 
        Opcode::Mul => 0x12, 
        Opcode::Div => 0x13, 
        Opcode::Mod => 0x14, 
        Opcode::And => 0x15, 
        Opcode::Or => 0x16, 
        Opcode::Eq => 0x17, 
        Opcode::Lt => 0x18, 
        Opcode::Gr => 0x19, 

        Opcode::Store => 0x20, 
        Opcode::Fetch => 0x21, 
        Opcode::Variable => 0x22, 

        Opcode::In => 0x30, 
        Opcode::Out => 0x31, 

        Opcode::If => 0x40, 
        Opcode::Then => 0x42, 
        Opcode::Call => 0x43, 
        Opcode::Begin => 0x44, 
        Opcode::Until => 0x45, 

        Opcode::LoadAddr => 0x50, 
        Opcode::Ret => 0xFE, 
        Opcode::Halt => 0xFF, 
        Opcode::Nop => 0x00 
    }
}

pub fn u8_to_instr(num: u8) -> Opcode {
    match num {
        0x01 => Opcode::Lit,
        0x02 => Opcode::Dup,
        0x03 => Opcode::Drop,
        0x04 => Opcode::Swap,
        0x05 => Opcode::Over,

        0x10 => Opcode::Add,
        0x11 => Opcode::Sub,
        0x12 => Opcode::Mul,
        0x13 => Opcode::Div,
        0x14 => Opcode::Mod,
        0x15 => Opcode::And,
        0x16 => Opcode::Or,
        0x17 => Opcode::Eq,
        0x18 => Opcode::Lt,
        0x19 => Opcode::Gr,

        0x20 => Opcode::Store,
        0x21 => Opcode::Fetch,
        0x22 => Opcode::Variable,

        0x30 => Opcode::In,
        0x31 => Opcode::Out,

        0x40 => Opcode::If,
        0x42 => Opcode::Then,
        0x43 => Opcode::Call,
        0x44 => Opcode::Begin,
        0x45 => Opcode::Until,

        0x50 => Opcode::LoadAddr,
        0xFE => Opcode::Ret,
        0xFF => Opcode::Halt,
        0x00 => Opcode::Nop,
    }
}
