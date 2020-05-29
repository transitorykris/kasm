use std::cmp::Ordering;
use std::collections::HashMap;

pub use crate::ascii::ascii_to_bytes;
pub use crate::ascii::unescape;

pub use crate::errors::Error;

pub use crate::instructions::address_mode_length;
pub use crate::instructions::address_mode_name;
pub use crate::instructions::str_to_mnemonic;
pub use crate::instructions::AddressMode;
pub use crate::instructions::Mnemonic;
pub use crate::instructions::Value;

pub use crate::scanner::SourceTable;

use regex::Regex;

pub struct Label {
    pub address: Address,
    #[allow(dead_code)]
    line: Line,
}

type LabelTable = HashMap<String, Label>;
type Address = u16;
type Line = u16;
type Data = Vec<u8>;

#[derive(Eq, PartialEq)]
pub enum Content {
    Code(Code),
    Data(Data),
}

#[derive(Eq, PartialEq)]
pub struct Code {
    pub mnemonic: Mnemonic,
    pub address_mode: AddressMode,
    pub value: Value,
}

#[derive(Eq)]
pub struct CodeTableEntry {
    pub address: Address,
    pub content: Content,
}

impl Ord for CodeTableEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.address.cmp(&other.address)
    }
}

impl PartialOrd for CodeTableEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CodeTableEntry {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

pub type CodeTable = Vec<CodeTableEntry>;

pub struct Program {
    pub symbol_table: LabelTable,
    pub code: CodeTable,
    counter: Address, // The current address as we go through pass1
}

impl Program {
    pub fn new() -> Program {
        Program {
            symbol_table: LabelTable::new(),
            code: CodeTable::new(),
            counter: 0x1000, // Worry about zeropage a little later
        }
    }
}

pub fn pass1(source: &SourceTable) -> Result<Program, Error> {
    //let mut program = Program {
    //    symbol_table: LabelTable::new(),
    //    code: CodeTable::new(),
    //    counter: 0x1000, // Worry about zeropage a little later
    //};
    let mut program = Program::new();

    // TODO:
    // - Handle the zeropage! Labels are funky here.
    // zeropage is addresses $00 through to $ff
    // We track whether we're in zeropage or not because it affects
    // how we handle labels (1 byte vs 2 bytes!)
    //let mut zeropage = true;

    for line in source {
        //if program.counter > 0xff {
        //    zeropage = false;
        //}
        let mut chars = line.line.chars();
        if line.line.ends_with(":") {
            handle_label(&mut program, line.line.to_string(), line.line_number);
        } else if chars.next().unwrap() == '.' {
            // XXX UNWRAP
            handle_directive(&mut program, &line.line);
        } else if chars.next().unwrap().is_ascii_alphabetic() {
            // BUG: This ^ is looking at the second charcter not the first!!
            // XXX UNWRAP
            handle_instruction(&mut program, &line.line);
        } else {
            return Err(Error::UnknownSyntax);
            // error!(
            //     Error::UnknownSyntax,
            //     "Unknown syntax: {} at line: {}", line.line, line.line_number
            // );
        }
    }

    program.code.sort();
    Ok(program)
}

// TODO: finish implement labels!
fn handle_label(program: &mut Program, raw_label: String, line_number: Line) {
    let label = String::from(raw_label.trim_end_matches(":"));

    if program.symbol_table.contains_key(&label) {
        error!(Error::DuplicateLabel, "Duplicate label found: {}", label);
    }

    program.symbol_table.insert(
        label,
        Label {
            address: program.counter,
            line: line_number,
        },
    );
}

// TODO: .equ directive
fn handle_directive(program: &mut Program, raw_line: &String) {
    let trimmed = raw_line.trim().trim_start_matches(".");

    let split: Vec<&str> = trimmed.splitn(2, " ").collect(); // Get two parts, the directive and data
    let dir = split[0];
    let value = String::from(split[1]);
    match dir {
        "org" => {
            let value = value.trim_start_matches("$");
            // XXX UNWRAP
            let address = u16::from_str_radix(value, 16).unwrap();
            program.counter = address;
        }
        "byte" => {
            let data = parse_bytes(value);
            program.counter += data.len() as u16;
            program.code.push(CodeTableEntry {
                address: program.counter,
                content: Content::Data(data),
            });
        }
        "ascii" => {
            let raw_string = String::from(value);
            let trimmed = String::from(raw_string.trim_start_matches("\"").trim_end_matches("\""));
            let (data, size) = ascii_to_bytes(trimmed);
            program.code.push(CodeTableEntry {
                address: program.counter,
                content: Content::Data(data),
            });
            program.counter += size;
        }
        "equ" => {
            let (label, value) = parse_equ(value);
            println!("Found an EQU: {} {:04x}", label, value);
            if program.symbol_table.contains_key(&label) {
                error!(Error::DuplicateLabel, "Duplicate label found: {}", label);
            }

            program.symbol_table.insert(
                label,
                Label {
                    address: value,
                    line: program.counter,
                },
            );
        }
        _ => error!(Error::UnknownDirective, "Unknown directive: {}", raw_line),
    }
}

fn parse_bytes(bytes: String) -> Data {
    let mut data = Vec::new();
    let parts = bytes.split_terminator(",");

    for part in parts {
        let raw_value = part.trim().trim_start_matches("$");
        // XXX UNWRAP
        let value = u8::from_str_radix(raw_value, 16).unwrap();
        data.push(value);
    }

    data
}

fn parse_equ(equ: String) -> (String, u16) {
    // XXX this isn't great
    // TODO:
    // - handle multiple kinds of values
    // - make it fail nicely
    let mut parts = equ.split("=");
    // XXX UNWRAP
    let label = parts.next().unwrap().trim().to_string();
    // XXX UNWRAP
    let raw_value = parts.next().unwrap().trim().trim_start_matches("$");
    // XXX UNWRAP
    let value = u16::from_str_radix(raw_value, 16).unwrap();
    (label, value)
}

fn handle_instruction(program: &mut Program, line: &String) {
    let mut parts = line.split_ascii_whitespace();
    // XXX UNWRAP
    let instruction = parts.next().unwrap().to_lowercase();
    let operand_part = parts.next();
    let value: Value;
    let address_mode: AddressMode;

    if operand_part.is_none() {
        address_mode = AddressMode::Implied;
        value = Value::Null;
    } else {
        // XXX UNWRAP
        let (address_mode_tmp, value_tmp) = get_operand_type(operand_part.unwrap());
        address_mode = address_mode_tmp;
        value = value_tmp;
    }

    let entry = CodeTableEntry {
        address: program.counter,
        content: Content::Code(Code {
            mnemonic: str_to_mnemonic(instruction),
            address_mode,
            value,
        }),
    };

    program.code.push(entry);

    // Move our program counter to the next free location
    program.counter += address_mode_length(address_mode);
}

fn get_operand_type(operand: &str) -> (AddressMode, Value) {
    // TODO: use lazy_static somehow!
    // XXX UNWRAP
    let implied_re = Regex::new(r"^$").unwrap();
    let zeropage_re = Regex::new(r"^\$([0-9a-f]{2})$").unwrap();
    let zeropagex_re = Regex::new(r"^\$([0-9a-f]{2})\s*,\s*x$").unwrap();
    let zeropagey_re = Regex::new(r"^\$([0-9a-f]{2})\s*,\s*y$").unwrap();
    let immediate_re = Regex::new(r"^#\$([0-9a-f]{2})$").unwrap();
    let absolute_re = Regex::new(r"^\$([0-9a-f]{4})$").unwrap();
    let absolutex_re = Regex::new(r"^\$([0-9a-f]{4})\s*,\s*x$").unwrap();
    let absolutey_re = Regex::new(r"^\$([0-9a-f]{4})\s*,\s*y$").unwrap();
    let indirect_re = Regex::new(r"^\(\$([0-9a-f]{4})\)$").unwrap();
    let xindexed_re = Regex::new(r"^\(\$([0-9a-f]{2})\s*,\s*x\)$").unwrap();
    let yindexed_re = Regex::new(r"^\(\$([0-9a-f]{2})\)\s*,\s*y$").unwrap();
    // oh no... forgot about opcode $ab relative address mode...
    // for branch targets...

    // XXX UNWRAP
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
        let val = u8::from_str_radix(&caps[1], 16).unwrap();
        return (AddressMode::IndirectX, Value::U8(val));
    } else if yindexed_re.is_match(operand) {
        let caps = yindexed_re.captures(operand).unwrap();
        let val = u8::from_str_radix(&caps[1], 16).unwrap();
        return (AddressMode::IndirectY, Value::U8(val));
    }

    // Now do it all again for labels :(
    // Note: we don't quite know where the labels are in memory right now
    // TODO: Add #< and #> for lo byte and hi byte
    // XXX we have no real limit on the length of a label right now
    // XXX UNWRAP
    let l_absolute_re = Regex::new(r"^([a-z_][0-9a-z_]*)$").unwrap();
    let l_absolutex_re = Regex::new(r"^([a-z_][0-9a-z_]*)\s*,\s*x$").unwrap();
    let l_absolutey_re = Regex::new(r"^([a-z_][0-9a-z_]*)\s*,\s*y$").unwrap();
    let l_immediate_re = Regex::new(r"^#([a-z_][0-9a-z_]*)$").unwrap();
    let l_indirect_re = Regex::new(r"^\(([a-z_][0-9a-z_]*)\)$").unwrap();
    let l_xindexed_re = Regex::new(r"^\(([a-z_][0-9a-z_]*)\s*,\s*x\)$").unwrap();
    let l_yindexed_re = Regex::new(r"^\(([a-z_][0-9a-z_]*)\)\s*,\s*y$").unwrap();
    // also missing relative mode

    // XXX UNWRAP
    if l_absolute_re.is_match(operand) {
        let caps = l_absolute_re.captures(operand).unwrap();
        let label = String::from(&caps[1]);
        return (AddressMode::Absolute, Value::String(label));
    } else if l_absolutex_re.is_match(operand) {
        let caps = l_absolutex_re.captures(operand).unwrap();
        let label = String::from(&caps[1]);
        return (AddressMode::AbsoluteX, Value::String(label));
    } else if l_absolutey_re.is_match(operand) {
        let caps = l_absolutey_re.captures(operand).unwrap();
        let label = String::from(&caps[1]);
        return (AddressMode::AbsoluteY, Value::String(label));
    } else if l_immediate_re.is_match(operand) {
        let caps = l_immediate_re.captures(operand).unwrap();
        let label = String::from(&caps[1]);
        return (AddressMode::Immediate, Value::String(label));
    } else if l_indirect_re.is_match(operand) {
        let caps = l_indirect_re.captures(operand).unwrap();
        let label = String::from(&caps[1]);
        return (AddressMode::Indirect, Value::String(label));
    } else if l_xindexed_re.is_match(operand) {
        let caps = l_xindexed_re.captures(operand).unwrap();
        let label = String::from(&caps[1]);
        return (AddressMode::IndirectX, Value::String(label));
    } else if l_yindexed_re.is_match(operand) {
        let caps = l_yindexed_re.captures(operand).unwrap();
        let label = String::from(&caps[1]);
        return (AddressMode::IndirectY, Value::String(label));
    }

    warning!("Warning! Unknown addressing mode");
    (AddressMode::Unknown, Value::String(String::from(operand)))
}
