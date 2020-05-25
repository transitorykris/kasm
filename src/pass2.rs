pub use crate::instructions::AddressMode;
pub use crate::instructions::InstructionKey;
pub use crate::instructions::InstructionMap;
pub use crate::instructions::Mnemonic;
pub use crate::instructions::Value;
pub use crate::pass1;
pub use crate::pass1::CodeTableEntry::{Code, Directive, Label};
pub use crate::pass1::Value::{Null, String, U16, U8};

pub type MachineCode = Vec<u8>;

pub fn pass2(instruction_set: InstructionMap, program: pass1::Program) -> MachineCode {
    // Determine the location of each of the labels
    // Update each reference to the symbol
    // For now we're not worrying about labels
    let mut output = MachineCode::new();
    let mut count = 0;

    for line in program.code {
        count = count + 1;
        match line {
            Code(address, line) => {
                print!("${:04x}: ", address);
                let instruction_key = InstructionKey {
                    mnemonic: line.mnemonic,
                    address_mode: line.address_mode,
                };
                match instruction_set.get(&instruction_key) {
                    Some(machine_code) => {
                        print!("{:02x} ", machine_code);
                        output.push(*machine_code);
                    }
                    None => panic!("Invalid instruction found"),
                }
                match line.value {
                    U8(val) => {
                        output.push(val);
                        println!("{:02x}", val);
                    }
                    U16(val) => {
                        let bytes = val.to_be_bytes();
                        output.push(bytes[1]); // Note: little endian!
                        output.push(bytes[0]);
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
                                println!("{:02x} {:02x}", bytes[1], bytes[0]);
                            }
                            // This should never happen
                            None => panic!("Unknown label: {}", label),
                        }
                    }
                };
            }
            // XXX this shouldn't be necessary in the second pass
            Label(_address, _symbol) => println!("Labels not yet implemented in pass2"),
            Directive(_address, _directive) => println!("Directives not yet implemented in pass2"),
        }
    }

    output
}
