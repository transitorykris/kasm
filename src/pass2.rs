pub use crate::instructions::AddressMode;
pub use crate::instructions::InstructionKey;
pub use crate::instructions::InstructionMap;
pub use crate::instructions::Mnemonic;
pub use crate::instructions::Value;
pub use crate::pass1::CodeTableEntry;
pub use crate::pass1::Content::{Code, Data};
pub use crate::pass1::Program;
pub use crate::pass1::Value::{Null, String, U16, U8};

pub type MachineCode = Vec<u8>;

pub fn pass2(instruction_set: InstructionMap, program: Program) -> MachineCode {
    let mut output = MachineCode::new();

    let mut next_address = 0;

    for line in program.code {
        let mut address = line.address;
        match line.content {
            Code(code) => {
                for _ in next_address..address {
                    output.push(0);
                }

                print!("${:04x}: ", address);
                let instruction_key = InstructionKey {
                    mnemonic: code.mnemonic,
                    address_mode: code.address_mode,
                };

                match instruction_set.get(&instruction_key) {
                    Some(machine_code) => {
                        print!("{:02x} ", machine_code);
                        output.push(*machine_code);
                        address += 1;
                    }
                    None => panic!("Invalid instruction found"),
                }

                match code.value {
                    U8(val) => {
                        output.push(val);
                        address += 1;
                        println!("{:02x}", val);
                    }
                    U16(val) => {
                        let bytes = val.to_be_bytes();
                        output.push(bytes[1]); // Note: little endian!
                        output.push(bytes[0]);
                        address += 2;
                        println!("{:02x} {:02x}", bytes[1], bytes[0]);
                    }
                    Null => {
                        println!();
                    }
                    String(label) => {
                        match program.symbol_table.get(&label) {
                            Some(val) => {
                                let bytes = val.address.to_be_bytes();
                                output.push(bytes[1]); // Note: little endian!
                                output.push(bytes[0]);
                                address += 2;
                                println!("{:02x} {:02x}", bytes[1], bytes[0]);
                            }
                            // This should never happen
                            None => panic!("Unknown label: {}", label),
                        }
                    }
                };
            }
            Data(data) => {
                for byte in data {
                    output.push(byte);
                    address += 1;
                }
            }
        }

        next_address = address;
    }

    output
}
