pub use crate::instructions::str_to_instruction;
pub use crate::instructions::AddressMode;
pub use crate::instructions::Mnemonic;
pub use crate::instructions::Value;
use regex::Regex;

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
    Unresolved(Counter, Mnemonic), // Uses a label we haven't resolved
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
fn handle_directive(program: &mut Program, line: &String) {}

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
    program
        .code
        .push(str_to_instruction(instruction, operand_type, value));
}

fn get_operand_type(operand: &str) -> (AddressMode, Value) {
    // TODO: use lazy_static somehow!
    let implied_re = Regex::new(r"^$").unwrap();
    let zeropage_re = Regex::new(r"^\$([0-9a-f]{2})$").unwrap();
    let zeropagex_re = Regex::new(r"^\$([0-9a-f]{2})\s*,\s*x$").unwrap();
    let zeropagey_re = Regex::new(r"^\$([0-9a-f]{2})\s*,\s*y$").unwrap();
    let immediate_re = Regex::new(r"^#\$([0-9a-f]{2})$").unwrap();
    let absolute_re = Regex::new(r"^\$([0-9a-f]{4})$").unwrap();
    let absolutex_re = Regex::new(r"^\$([0-9a-f]{4})\s*,\s*x$").unwrap();
    let absolutey_re = Regex::new(r"^\$([0-9a-f]{4})\s*,\s*y$").unwrap();
    let indirect_re = Regex::new(r"^\(\$([0-9a-f]{4})\)$").unwrap();
    let xindexed_re = Regex::new(r"^\(\$([0-9a-f]{4})\s*,\s*x\)$").unwrap();
    let yindexed_re = Regex::new(r"^\(\$([0-9a-f]{4})\)\s*,\s*y$").unwrap();
    // oh no... forgot about opcode $ab relative address mode...
    // for branch targets...

    if implied_re.is_match(operand) {
        return (AddressMode::Implied, Value::Null);
    } else if zeropage_re.is_match(operand) {
        let caps = zeropage_re.captures(operand).unwrap();
        let val = u8::from_str_radix(&caps[1], 16).unwrap();
        return (AddressMode::Zeropage, Value::U8(val));
    } else if zeropagex_re.is_match(operand) {
        let caps = zeropagex_re.captures(operand).unwrap();
        let val = u8::from_str_radix(&caps[1], 16).unwrap();
        return (AddressMode::ZeropageX, Value::U8(val));
    } else if zeropagey_re.is_match(operand) {
        let caps = zeropagey_re.captures(operand).unwrap();
        let val = u8::from_str_radix(&caps[1], 16).unwrap();
        return (AddressMode::ZeropageY, Value::U8(val));
    } else if immediate_re.is_match(operand) {
        let caps = immediate_re.captures(operand).unwrap();
        let val = u8::from_str_radix(&caps[1], 16).unwrap();
        return (AddressMode::Immediate, Value::U8(val));
    } else if absolute_re.is_match(operand) {
        let caps = absolute_re.captures(operand).unwrap();
        let val = u16::from_str_radix(&caps[1], 16).unwrap();
        return (AddressMode::Absolute, Value::U16(val));
    } else if absolutex_re.is_match(operand) {
        let caps = absolutex_re.captures(operand).unwrap();
        let val = u16::from_str_radix(&caps[1], 16).unwrap();
        return (AddressMode::AbsoluteX, Value::U16(val));
    } else if absolutey_re.is_match(operand) {
        let caps = absolutey_re.captures(operand).unwrap();
        let val = u16::from_str_radix(&caps[1], 16).unwrap();
        return (AddressMode::AbsoluteY, Value::U16(val));
    } else if indirect_re.is_match(operand) {
        let caps = indirect_re.captures(operand).unwrap();
        let val = u16::from_str_radix(&caps[1], 16).unwrap();
        return (AddressMode::Indirect, Value::U16(val));
    } else if xindexed_re.is_match(operand) {
        let caps = xindexed_re.captures(operand).unwrap();
        let val = u16::from_str_radix(&caps[1], 16).unwrap();
        return (AddressMode::IndirectX, Value::U16(val));
    } else if yindexed_re.is_match(operand) {
        let caps = yindexed_re.captures(operand).unwrap();
        let val = u16::from_str_radix(&caps[1], 16).unwrap();
        return (AddressMode::IndirectY, Value::U16(val));
    }

    (AddressMode::Unknown, Value::String(String::from(operand)))
}
