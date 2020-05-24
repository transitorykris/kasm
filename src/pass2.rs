pub use crate::instructions::AddressMode;
pub use crate::instructions::InstructionKey;
pub use crate::instructions::InstructionMap;
pub use crate::instructions::Mnemonic;
pub use crate::instructions::Value;
pub use crate::pass1;
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
        let instruction_key = InstructionKey {
            mnemonic: line.0,
            address_mode: line.1,
        };
        let machine_code = instruction_set.get(&instruction_key);
        output.push(*machine_code.unwrap());
        match line.2 {
            U8(val) => {
                output.push(val);
            }
            U16(val) => {
                let bytes = val.to_be_bytes();
                output.push(bytes[1]); // Note: little endian!
                output.push(bytes[0]);
            }
            Null => {}
            String(label) => {
                panic!("Found unresolved label: {}", label);
            }
        };
    }
    output
}
