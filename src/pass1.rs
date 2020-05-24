use std::collections::HashMap;

pub use crate::instructions::str_to_mnemonic;
pub use crate::instructions::AddressMode;
pub use crate::instructions::Mnemonic;
pub use crate::instructions::Value;

pub use crate::scanner::SourceTable;

use regex::Regex;

struct Label {
    address: Address,
    line: Line,
}

type LabelTable = HashMap<String, Label>;
type Address = u16;
type Line = u16;

pub struct Code {
    pub mnemonic: Mnemonic,
    pub address_mode: AddressMode,
    pub value: Value,
}

pub struct Directive {
    pub label: String,
}

pub enum CodeTableEntry {
    Code(Code),
    Label(String),
    Directive(Directive),
}

pub type CodeTable = Vec<CodeTableEntry>;

pub struct Program {
    symbol_table: LabelTable,
    pub code: CodeTable,
}

pub fn pass1(source: &SourceTable) -> Program {
    let mut program = Program {
        symbol_table: LabelTable::new(),
        code: CodeTable::new(),
    };

    for line in source {
        let mut chars = line.line.chars();
        if line.line.ends_with(":") {
            handle_label(&mut program, line.line.to_string(), line.line_number);
        } else if chars.next().unwrap() == '.' {
            handle_directive(&mut program, &line.line);
        } else if chars.next().unwrap().is_ascii_alphabetic() {
            handle_instruction(&mut program, &line.line);
        } else {
            panic!(
                "Unknown syntax: {} at line: {}",
                line.line, line.line_number
            );
        }
    }
    // Store labels in a label table to have their addresses determined later
    // Determine length in bytes the symbol requires
    program
}

// TODO: implement labels!
fn handle_label(program: &mut Program, label: String, line_number: Line) {
    println!("Warning: labels are not implemented yet: {}", label);

    if program.symbol_table.contains_key(&label) {
        panic!("Duplicate label found: {}", label);
    }

    program.symbol_table.insert(
        label,
        Label {
            address: 0,
            line: line_number,
        },
    );
}

// TODO: implement directives!
fn handle_directive(program: &mut Program, line: &String) {
    println!("Warning: directives are not implemented yet: {}", line);
}

fn handle_instruction(program: &mut Program, line: &String) {
    let mut parts = line.split_ascii_whitespace();
    let instruction = parts.next().unwrap().to_lowercase();
    let operand_part = parts.next();
    let value: Value;
    let address_mode: AddressMode;

    if operand_part.is_none() {
        address_mode = AddressMode::Implied;
        value = Value::Null;
    } else {
        let (address_mode_tmp, value_tmp) = get_operand_type(operand_part.unwrap());
        address_mode = address_mode_tmp;
        value = value_tmp;
    }

    let entry = CodeTableEntry::Code(Code{
            mnemonic: str_to_mnemonic(instruction),
            address_mode,
            value,
        });

    program.code.push(entry);
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
