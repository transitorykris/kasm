pub use crate::instructions::Mnemonic;
pub use crate::instructions::AddressMode;

enum Instruction {
    Absolute(Counter, Mnemonic, u16),
    AbsoluteX(Counter, Mnemonic, u16),
    AbsoluteY(Counter, Mnemonic, u16),
    Immediate(Counter, Mnemonic, u8),
    Implied(Counter, Mnemonic),
    Indirect(Counter, Mnemonic, u8),
    IndirectX(Counter, Mnemonic, u8),
    IndirectY(Counter, Mnemonic, u8),
    Relative(Counter, Mnemonic, u8),
    Zeropage(Counter, Mnemonic, u8),
    ZeropageX(Counter, Mnemonic, u8),
    ZeropageY(Counter, Mnemonic, u8),
    Unresolved(Counter, Mnemonic),  // Uses a label we haven't resolved
}

type SymbolTable = Vec<(String, Address, Line)>;
type Address = u16;
type Counter = u16;
type Line = u16;
pub type CodeTable = Vec<(Mnemonic, AddressMode, Value)>;

pub enum Value {
    String(String),
    U8(u8),
    U16(u16),
    Null,
}

pub struct Program {
    symbol_table: SymbolTable,
    pub code: CodeTable,
}

pub fn pass1(source: &str) -> Program {
    let mut program = Program {
        symbol_table: SymbolTable::new(),
        code: CodeTable::new(),
    };

    // Iterate over each line of source file
    for raw_line in source.lines() {
        let line = raw_line.trim();     // Trim off leading and trailing whitespace
        if line.len() == 0 {
            // We got a blank line
        } else if line.chars().next().unwrap() == ';' {
            // We got a comment
        } else if line.chars().next().unwrap() == '.' {
            // We've got a directive
            handle_directive(&mut program, line);
        } else if line.chars().next().unwrap().is_ascii_alphabetic() {
            // We got an instruction
            println!("Instruction found: {}", line);
            handle_instruction(&mut program, line);
        } else {
            // this isn't good!
            panic!("Unknown syntax: {}", line);
        }
    }
    // Store labels in a label table to have their addresses determined later
    // Determine length in bytes the symbol requires
    program
}

// TODO: implement directives!
fn handle_directive(program: &mut Program, line: &str) {

}

fn handle_instruction(program: &mut Program, line: &str) {
    let mut parts = line.split_ascii_whitespace();
    let part = parts.next().unwrap().to_lowercase();
    let (operand_type, value) = get_operand_type(parts.next().unwrap());
    if part == "lda" {
        println!("Found LDA!");
        program.code.push((Mnemonic::LDA, operand_type, value));
    } else if part == "inc" {
        println!("Found INC!");
        program.code.push((Mnemonic::INC, operand_type, value));
    } else if part == "sta" {
        println!("Found STA!");
        program.code.push((Mnemonic::STA, operand_type, value));
    }
}

fn get_operand_type(operand: &str) -> (AddressMode, Value) {
    let mut x_indexed = false;
    let mut y_indexed = false;
    let mut raw_operand = String::from(operand);

    println!("Checking operand {} length {}", raw_operand, raw_operand.len());
    if raw_operand == "" {
        println!("implied");
        return (AddressMode::Implied, Value::Null);
    }

    if raw_operand.starts_with(";") {
        // That's a comment there!
        println!("implied");
        return (AddressMode::Implied, Value::Null);
    }
    
    // Check if this is X or Y indexed
    if raw_operand.ends_with(",X") {
        println!("X indexed");
        x_indexed = true;
    } else if raw_operand.ends_with(",Y") {
        println!("Y indexed");
        y_indexed = true;
    }

    // If it is, strip off the index
    if x_indexed || y_indexed {
        raw_operand.truncate(raw_operand.len()-2);
    }

    // XXX the strings are hex, but right now they read as decimal
    // XXX from_str_radix?
    if raw_operand.starts_with("$") {
        if raw_operand.len() == 3 && x_indexed {
            println!("zeropage");
            let val = u8::from_str_radix(raw_operand.trim_start_matches("$"), 16);
            return (AddressMode::ZeropageX, Value::U8(val.unwrap()));
        } else if raw_operand.len() == 3 && y_indexed {
            println!("zeropage");
            let val = u8::from_str_radix(raw_operand.trim_start_matches("$"), 16);
            return (AddressMode::ZeropageY, Value::U8(val.unwrap()));
        } else if raw_operand.len() == 3 {
            println!("absolute zeropage");
            let val = u8::from_str_radix(raw_operand.trim_start_matches("$"), 16);
            return (AddressMode::Absolute, Value::U8(val.unwrap()));
        } else {
            println!("absolute");
            let val = u16::from_str_radix(raw_operand.trim_start_matches("$"), 16);
            return (AddressMode::Absolute, Value::U16(val.unwrap()));
        }
    } else if raw_operand.starts_with("#$") {
        println!("immediate");
        let val = u8::from_str_radix(raw_operand.trim_start_matches("#$"), 16);
        return (AddressMode::Immediate, Value::U8(val.unwrap()));
    } else if raw_operand.starts_with("($") {
        println!("indirect");
        let val = u16::from_str_radix(raw_operand.trim_start_matches("($"), 16);
        return (AddressMode::Indirect, Value::U16(val.unwrap()));
    }
    // Otherwise must be a label
    println!("unknown");
    (AddressMode::Unknown, Value::String(raw_operand))
}