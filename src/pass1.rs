use std::cmp::Ordering;
use std::collections::HashMap;

use crate::ascii::ascii_to_bytes;
use crate::errors::error;
use crate::errors::Error;
use crate::errors::ErrorCode;
use crate::instructions::address_mode_length;
use crate::instructions::str_to_mnemonic;
use crate::instructions::AddressMode;
use crate::instructions::Mnemonic;
use crate::instructions::Value;
use crate::scanner::SourceTable;

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

impl Default for Program {
    fn default() -> Self {
        Program::new()
    }
}

pub fn pass1(source: &SourceTable) -> Result<Program, Error> {
    let mut program = Program::new();

    // TODO:
    // - Handle the zeropage! Labels are funky here.
    // zeropage is addresses $00 through to $ff

    for line in source {
        let first_char = match line.line.chars().next() {
            Some(first_char) => first_char,
            None => panic!("Got a line of source with not characters!"),
        };
        if line.line.ends_with(':') {
            match handle_label(&mut program, line.line.to_string(), line.line_number) {
                Ok(()) => (),
                Err(err) => return Err(err),
            }
        } else if first_char == '.' {
            match handle_directive(&mut program, &line.line) {
                Ok(()) => (),
                Err(err) => return Err(err),
            }
        } else if first_char.is_ascii_alphabetic() {
            match handle_instruction(&mut program, &line.line) {
                Ok(()) => (),
                Err(err) => return Err(err),
            }
        } else {
            return Err(error(
                ErrorCode::UnknownSyntax,
                format!(
                    "Unknown syntax: {} at line: {}",
                    line.line, line.line_number
                ),
            ));
        }
    }

    program.code.sort();
    Ok(program)
}

// TODO: finish implement labels!
fn handle_label(program: &mut Program, raw_label: String, line_number: Line) -> Result<(), Error> {
    let label = String::from(raw_label.trim_end_matches(':'));

    if program.symbol_table.contains_key(&label) {
        return Err(error(
            ErrorCode::DuplicateLabel,
            format!("Duplicate label found: {}", label),
        ));
    }

    program.symbol_table.insert(
        label,
        Label {
            address: program.counter,
            line: line_number,
        },
    );

    Ok(())
}

// TODO: .equ directive
fn handle_directive(program: &mut Program, raw_line: &str) -> Result<(), Error> {
    let trimmed = raw_line.trim().trim_start_matches('.');

    let split: Vec<&str> = trimmed.splitn(2, ' ').collect(); // Get two parts, the directive and data
    let dir = split[0];
    let value = String::from(split[1]);
    match dir {
        "org" => {
            let value = value.trim_start_matches('$');
            let address = match str_to_u16!(value) {
                Ok(address) => address,
                Err(_) => {
                    return Err(error(
                        ErrorCode::AddressExpected,
                        format!("Expected address for label, found {}", value),
                    ))
                }
            };
            program.counter = address;
        }
        "byte" => {
            let data = match parse_bytes(value) {
                Ok(data) => data,
                Err(err) => return Err(err),
            };
            program.counter += data.len() as u16;
            program.code.push(CodeTableEntry {
                address: program.counter,
                content: Content::Data(data),
            });
        }
        "ascii" => {
            let trimmed = String::from(value.trim_start_matches('\"').trim_end_matches('\"'));
            let (data, size) = ascii_to_bytes(trimmed);
            program.code.push(CodeTableEntry {
                address: program.counter,
                content: Content::Data(data),
            });
            program.counter += size;
        }
        "equ" => {
            let (label, value) = match parse_equ(value) {
                Ok((label, value)) => (label, value),
                Err(err) => return Err(err),
            };
            println!("Found an EQU: {} {:04x}", label, value);
            if program.symbol_table.contains_key(&label) {
                return Err(error(
                    ErrorCode::DuplicateLabel,
                    format!("Duplicate label found: {}", label),
                ));
            }

            program.symbol_table.insert(
                label,
                Label {
                    address: value,
                    line: program.counter,
                },
            );
        }
        _ => {
            return Err(error(
                ErrorCode::UnknownDirective,
                format!("Unknown directive: {}", raw_line),
            ))
        }
    }
    Ok(())
}

fn parse_bytes(bytes: String) -> Result<Data, Error> {
    let mut data = Vec::new();
    let parts = bytes.split_terminator(',');

    for part in parts {
        let raw_value = part.trim().trim_start_matches('$');
        let value = match str_to_u8!(raw_value) {
            Ok(value) => value,
            Err(_) => {
                return Err(error(
                    ErrorCode::HexExpected,
                    format!("Expected hexadecimal but found {}", raw_value),
                ))
            }
        };
        data.push(value);
    }

    Ok(data)
}

fn parse_equ(equ: String) -> Result<(String, u16), Error> {
    // TODO:
    // - handle multiple kinds of values
    let mut parts = equ.split('=');
    let label = match parts.next() {
        Some(label) => label.trim().to_string(),
        None => {
            return Err(error(
                ErrorCode::MalformedEqu,
                format!(".equ is not properly formatted {}", equ),
            ))
        }
    };

    let raw_value = match parts.next() {
        Some(raw_value) => raw_value.trim().trim_start_matches('$'),
        None => {
            return Err(error(
                ErrorCode::MalformedEqu,
                format!(".equ is not properly formatted {}", equ),
            ))
        }
    };

    let value = match str_to_u16!(raw_value) {
        Ok(value) => value,
        Err(_) => {
            return Err(error(
                ErrorCode::HexExpected,
                format!("Expected hex but found {}", raw_value),
            ))
        }
    };
    Ok((label, value))
}

fn handle_instruction(program: &mut Program, line: &str) -> Result<(), Error> {
    let mut parts = line.split_ascii_whitespace();

    let instruction = match parts.next() {
        Some(instruction) => instruction.to_lowercase(),
        None => panic!("Did not receive an instruction!"),
    };

    let value: Value;
    let address_mode: AddressMode;

    match parts.next() {
        Some(operand_part) => {
            let (address_mode_tmp, value_tmp) = get_operand_type(operand_part);
            address_mode = address_mode_tmp;
            value = value_tmp;
        }
        None => {
            address_mode = AddressMode::Implied;
            value = Value::Null;
        }
    };

    let mnemonic = match str_to_mnemonic(instruction) {
        Ok(string) => string,
        Err(err) => return Err(err),
    };
    let entry = CodeTableEntry {
        address: program.counter,
        content: Content::Code(Code {
            mnemonic,
            address_mode,
            value,
        }),
    };

    program.code.push(entry);

    // Move our program counter to the next free location
    program.counter += address_mode_length(address_mode);

    Ok(())
}

fn get_operand_type(operand: &str) -> (AddressMode, Value) {
    // TODO: use lazy_static somehow!
    // We use unwrap here and know we're good because we tested the code :)
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

    // We use unwrap below but the regexes guarantee we got something sane
    if operand.is_empty() {
        return (AddressMode::Implied, Value::Null);
    } else if zeropage_re.is_match(operand) {
        let caps = zeropage_re.captures(operand).unwrap();
        let val = str_to_u8!(&caps[1]).unwrap();
        return (AddressMode::Zeropage, Value::U8(val));
    } else if zeropagex_re.is_match(operand) {
        let caps = zeropagex_re.captures(operand).unwrap();
        let val = str_to_u8!(&caps[1]).unwrap();
        return (AddressMode::ZeropageX, Value::U8(val));
    } else if zeropagey_re.is_match(operand) {
        let caps = zeropagey_re.captures(operand).unwrap();
        let val = str_to_u8!(&caps[1]).unwrap();
        return (AddressMode::ZeropageY, Value::U8(val));
    } else if immediate_re.is_match(operand) {
        let caps = immediate_re.captures(operand).unwrap();
        let val = str_to_u8!(&caps[1]).unwrap();
        return (AddressMode::Immediate, Value::U8(val));
    } else if absolute_re.is_match(operand) {
        let caps = absolute_re.captures(operand).unwrap();
        let val = str_to_u16!(&caps[1]).unwrap();
        return (AddressMode::Absolute, Value::U16(val));
    } else if absolutex_re.is_match(operand) {
        let caps = absolutex_re.captures(operand).unwrap();
        let val = str_to_u16!(&caps[1]).unwrap();
        return (AddressMode::AbsoluteX, Value::U16(val));
    } else if absolutey_re.is_match(operand) {
        let caps = absolutey_re.captures(operand).unwrap();
        let val = str_to_u16!(&caps[1]).unwrap();
        return (AddressMode::AbsoluteY, Value::U16(val));
    } else if indirect_re.is_match(operand) {
        let caps = indirect_re.captures(operand).unwrap();
        let val = str_to_u16!(&caps[1]).unwrap();
        return (AddressMode::Indirect, Value::U16(val));
    } else if xindexed_re.is_match(operand) {
        let caps = xindexed_re.captures(operand).unwrap();
        let val = str_to_u8!(&caps[1]).unwrap();
        return (AddressMode::IndirectX, Value::U8(val));
    } else if yindexed_re.is_match(operand) {
        let caps = yindexed_re.captures(operand).unwrap();
        let val = str_to_u8!(&caps[1]).unwrap();
        return (AddressMode::IndirectY, Value::U8(val));
    }

    // Now do it all again for labels :(
    // Note: we don't quite know where the labels are in memory right now
    // TODO: Add #< and #> for lo byte and hi byte
    // XXX we have no real limit on the length of a label right now
    // We use unwrap here and know we're good because we tested the code :)
    let l_absolute_re = Regex::new(r"^([a-z_][0-9a-z_]*)$").unwrap();
    let l_absolutex_re = Regex::new(r"^([a-z_][0-9a-z_]*)\s*,\s*x$").unwrap();
    let l_absolutey_re = Regex::new(r"^([a-z_][0-9a-z_]*)\s*,\s*y$").unwrap();
    let l_immediate_re = Regex::new(r"^#([a-z_][0-9a-z_]*)$").unwrap();
    let l_indirect_re = Regex::new(r"^\(([a-z_][0-9a-z_]*)\)$").unwrap();
    let l_xindexed_re = Regex::new(r"^\(([a-z_][0-9a-z_]*)\s*,\s*x\)$").unwrap();
    let l_yindexed_re = Regex::new(r"^\(([a-z_][0-9a-z_]*)\)\s*,\s*y$").unwrap();
    // also missing relative mode

    // We use unwrap below but the regexes guarantee we got something sane
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
