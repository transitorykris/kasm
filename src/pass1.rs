use std::cmp::Ordering;
use std::collections::HashMap;

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

pub fn pass1(source: &SourceTable) -> Program {
    let mut program = Program {
        symbol_table: LabelTable::new(),
        code: CodeTable::new(),
        counter: 0x1000, // Worry about zeropage a little later
    };

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
    program.code.sort();
    program
}

// TODO: implement labels!
fn handle_label(program: &mut Program, raw_label: String, line_number: Line) {
    let label = String::from(raw_label.trim_end_matches(":"));

    println!("Warning: labels are partially implemented: {}", label);

    if program.symbol_table.contains_key(&label) {
        panic!("Duplicate label found: {}", label);
    }

    println!("${:04x}: {}", program.counter, label);
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
    println!("Hanlding directive: {}", trimmed);

    //let mut split = trimmed.split_ascii_whitespace();
    let split: Vec<&str> = trimmed.splitn(2, " ").collect(); // Get two parts, the directive and data
    let dir = split[0];
    let value = String::from(split[1]);
    match dir {
        "org" => {
            // XXX this feels gross
            let value = value.trim_start_matches("$");
            let address = u16::from_str_radix(value, 16).unwrap();
            program.counter = address;
        }
        "byte" => {
            let (data, size) = parse_bytes(value);
            program.code.push(CodeTableEntry {
                address: program.counter,
                content: Content::Data(data),
            });
            program.counter += size;
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
        _ => panic!("Unknown directive: {}", raw_line),
    }
}

fn parse_bytes(bytes: String) -> (Data, u16) {
    let mut data = Vec::new();
    let parts = bytes.split_terminator(",");
    let mut size = 0;

    for part in parts {
        let raw_value = part.trim().trim_start_matches("$");
        let value = u8::from_str_radix(raw_value, 16).unwrap();
        data.push(value);
        size += 1;
    }

    (data, size)
}

fn ascii_to_bytes(ascii: String) -> (Data, u16) {
    let mut data = Vec::new();
    let mut size = 0;
    let mut in_escape = false; // track whether we're in an escape or not
    for ch in ascii.bytes() {
        if in_escape {
            data.push(unescape(ch));
            size += 1;
            println!("CHAR: {:02x}", unescape(ch));
            in_escape = false;
        } else if ch == 0x5c {
            // 0x5c is ascii backslash
            in_escape = true;
        } else {
            data.push(ch);
            size += 1;
            println!("CHAR: {:02x}", ch);
        }
    }
    (data, size)
}

fn unescape(ch: u8) -> u8 {
    // Standard C escape sequences
    match ch {
        0x61 => return 0x07, // \a Alert
        0x62 => return 0x08, // \b Backspace
        0x65 => return 0x1b, // \e Escape
        0x66 => return 0x0c, // \f Formfeed
        0x6e => return 0x0a, // \n Newline
        0x72 => return 0x0d, // \r Carriage Return
        0x74 => return 0x09, // \t Horizontal Tab
        0x7c => return 0x0b, // \v Vertical Tab
        0x5c => return 0x5c, // \\ Backslash
        0x27 => return 0x27, // \' Apostrophe
        0x22 => return 0x22, // \" Double Quotation Mark
        0x3f => return 0x34, // \? Question Mark
        _ => panic!("Unknown escape code: \\{}", ch),
    }
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
    let l_absolute_re = Regex::new(r"^([a-z_][0-9a-z_]*)$").unwrap();
    let l_absolutex_re = Regex::new(r"^([a-z_][0-9a-z_]*)\s*,\s*x$").unwrap();
    let l_absolutey_re = Regex::new(r"^([a-z_][0-9a-z_]*)\s*,\s*y$").unwrap();
    let l_immediate_re = Regex::new(r"^#([a-z_][0-9a-z_]*)$").unwrap();
    let l_indirect_re = Regex::new(r"^\(([a-z_][0-9a-z_]*)\)$").unwrap();
    let l_xindexed_re = Regex::new(r"^\(([a-z_][0-9a-z_]*)\s*,\s*x\)$").unwrap();
    let l_yindexed_re = Regex::new(r"^\(([a-z_][0-9a-z_]*)\)\s*,\s*y$").unwrap();
    // also missing relative mode

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

    println!("Warning! Unknown addressing mode");
    (AddressMode::Unknown, Value::String(String::from(operand)))
}
