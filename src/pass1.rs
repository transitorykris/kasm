pub use crate::instructions::Mnemonic;
pub use crate::instructions::AddressMode;
pub use crate::instructions::Value;
pub use crate::instructions::str_to_instruction;

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

pub struct Program {
    symbol_table: SymbolTable,
    pub code: CodeTable,
}

pub fn pass1(source: &Vec<(String, u16)>) -> Program {
    let mut program = Program {
        symbol_table: SymbolTable::new(),
        code: CodeTable::new(),
    };

    // Iterate over each line of source file
    for line in source {
        if line.0.chars().next().unwrap() == '.' {
            // We've got a directive
            handle_directive(&mut program, &line.0);
        } else if line.0.chars().next().unwrap().is_ascii_alphabetic() {
            // TODO: this doesn't handle labels:
            // We got an instruction
            handle_instruction(&mut program, &line.0);
        } else {
            // this isn't good!
            panic!("Unknown syntax: {} at line: {}", line.0, line.1);
        }
    }
    // Store labels in a label table to have their addresses determined later
    // Determine length in bytes the symbol requires
    program
}

// TODO: implement directives!
fn handle_directive(program: &mut Program, line: &String) {

}

fn handle_instruction(program: &mut Program, line: &String) {
    let mut parts = line.split_ascii_whitespace();
    let instruction = parts.next().unwrap().to_lowercase();
    println!("About to handle instruction {}", instruction);
    let operand_part = parts.next();
    let value: Value;
    let operand_type: AddressMode;
    if operand_part.is_none() {
        println!("is_none!!!!");
        operand_type = AddressMode::Implied;
        value = Value::Null;
    } else {
        println!("unwrap!!!!");
        let (operand_type_tmp, value_tmp) = get_operand_type(operand_part.unwrap());
        operand_type = operand_type_tmp;
        value = value_tmp;
    }
    program.code.push(str_to_instruction(instruction, operand_type, value));
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

    // ^\$([0-9a-f]{2})$                ; Zeropage
    // ^\$([0-9a-f]{2})\s*,\s*x$        ; ZeropageX
    // ^\$([0-9a-f]{2})\s*,\s*y$        ; ZeropageY
    // ^#\$([0-9a-f]{2})$               ; Immediate
    // ^\$([0-9a-f]{4})$                ; Absolute
    // ^\$([0-9a-f]{4})\s*,\s*x$        ; AbsoluteX
    // ^\$([0-9a-f]{4})\s*,\s*y$        ; AbsoluteY
    // ^\(\$([0-9a-f]{4})\)$            ; Indirect
    // ^\(\$([0-9a-f]{4}\s*,\s*x)\)$    ; x-indexed indirect
    // ^\(\$([0-9a-f]{4})\)\s*,\s*y$    ; indirect y-indexed
    // oh no... forgot about opcode $ab relative address mode...
    // for branch targets...

    
    // Check if this is X or Y indexed
    if raw_operand.ends_with(",x") {
        println!("X indexed");
        x_indexed = true;
    } else if raw_operand.ends_with(",y") {
        println!("Y indexed");
        y_indexed = true;
    }

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
    } else if raw_operand.starts_with("($") && operand.ends_with(")") {
        println!("indirect");
        let operand_trimmed = raw_operand.trim_start_matches("($").trim_end_matches(")");
        let val = u16::from_str_radix(operand_trimmed, 16);
        return (AddressMode::Indirect, Value::U16(val.unwrap()));
    } else if raw_operand.starts_with("($") && x_indexed {
        println!("indirectX");
        println!("{}", raw_operand);
        let operand_trimmed = raw_operand.trim_start_matches("($").trim_end_matches(",x");
        println!("{}", operand_trimmed);
        let val = u16::from_str_radix(operand_trimmed, 16);
        return (AddressMode::IndirectX, Value::U16(val.unwrap()));
    } else if raw_operand.starts_with("($") && y_indexed {
        println!("indirectY");
        let operand_trimmed = raw_operand.trim_start_matches("($").trim_end_matches("),y");
        let val = u16::from_str_radix(operand_trimmed, 16);
        return (AddressMode::IndirectY, Value::U16(val.unwrap()));
    } else {
        // Otherwise must be a label
        println!("unknown");
        (AddressMode::Unknown, Value::String(raw_operand))
    }
}