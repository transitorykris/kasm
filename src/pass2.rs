pub use crate::instructions::AddressMode;
pub use crate::instructions::InstructionKey;
pub use crate::instructions::InstructionMap;
pub use crate::instructions::Mnemonic;
pub use crate::instructions::Value;
pub use crate::pass1;
pub use crate::pass1::Value::{Null, String, U16, U8};
pub use crate::pass1::CodeTableEntry::{Code, Label, Directive};

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
                let machine_code = instruction_set.get(&instruction_key);
                print!("{:02x} ", machine_code.unwrap());
                output.push(*machine_code.unwrap());
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
                    Null => {println!();}
                    String(label) => {
                        panic!("Found unresolved label: {}", label);
                    }
                };
            },
            Label(address, symbol) => println!("Labels not yet implemented in pass2"),
            Directive(address, directive) => println!("Directives not yet implemented in pass2"),
        }
    }

    output
}
