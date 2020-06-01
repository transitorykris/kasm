use crate::errors::error;
use crate::errors::Error;
use crate::errors::ErrorCode;
use crate::instructions::InstructionKey;
use crate::instructions::InstructionMap;
use crate::instructions::Value::{Null, String, U16, U8};
use crate::pass1::Content::{Code, Data};
use crate::pass1::Program;

pub type MachineCode = Vec<u8>;

pub fn pass2(instruction_set: InstructionMap, program: Program) -> Result<MachineCode, Error> {
    let mut output = MachineCode::new();

    let mut next_address = 0;

    for line in program.code {
        let mut address = line.address;

        for _ in next_address..address {
            output.push(0);
        }
        verbose!("${:04x}: ", address);

        match line.content {
            Code(code) => {
                let instruction_key = InstructionKey {
                    mnemonic: code.mnemonic,
                    address_mode: code.address_mode,
                };

                match instruction_set.get(&instruction_key) {
                    Some(machine_code) => {
                        verbose!("{:02x} ", machine_code);
                        output.push(*machine_code);
                        address += 1;
                    }
                    None => {
                        return Err(error(
                            ErrorCode::UnknownInstruction,
                            "Invalid instruction found".to_string(),
                        ))
                    }
                }

                match code.value {
                    U8(val) => {
                        output.push(val);
                        address += 1;
                        verbose!("{:02x}", val);
                    }
                    U16(val) => {
                        let bytes = val.to_be_bytes();
                        output.append(&mut vec![bytes[1], bytes[0]]); // Note: little endian!
                        address += 2;
                        verbose!("{:02x} {:02x}", bytes[1], bytes[0]);
                    }
                    Null => {}
                    String(label) => {
                        match program.symbol_table.get(&label) {
                            Some(val) => {
                                let bytes = val.address.to_be_bytes();
                                if bytes.len() == 1 {
                                    output.append(&mut vec![bytes[0]]); // Note: little endian!
                                    verbose!("{:02x}", bytes[0]);
                                } else {
                                    output.append(&mut vec![bytes[1], bytes[0]]); // Note: little endian!
                                    verbose!("{:02x} {:02x}", bytes[1], bytes[0]);
                                }
                                address += bytes.len() as u16;
                            }
                            // This should never happen
                            None => panic!("Found an unknown label {}", label),
                        }
                    }
                };
            }
            Data(data) => {
                for byte in data {
                    output.push(byte);
                    address += 1;
                    verbose!("{:02x} ", byte);
                }
            }
        }
        verboseln!("");
        next_address = address;
    }

    Ok(output)
}
