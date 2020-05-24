pub use crate::instructions::InstructionKey;
pub use crate::instructions::InstructionMap;
pub use crate::pass1::Value::{Null, String, U16, U8};
pub use crate::pass2::Program;

pub fn assemble(instruction_set: InstructionMap, program: Program, size: usize) -> Vec<u8> {
    let mut output = Vec::new();
    let mut count = 0;
    for line in program.code {
        count = count + 1;
        println!("{}", count);
        let instruction_key = InstructionKey {
            mnemonic: line.0,
            address_mode: line.1,
        };
        let machine_code = instruction_set.get(&instruction_key);
        print!("{:02x}", *machine_code.unwrap());
        output.push(*machine_code.unwrap());
        match line.2 {
            U8(val) => {
                println!("{:02x}", val);
                output.push(val);
            }
            U16(val) => {
                println!("{:04x}", val);
                let bytes = val.to_be_bytes();
                output.push(bytes[1]); // Note: little endian!
                output.push(bytes[0]);
            }
            Null => {
                println!("");
            }
            String(label) => {
                panic!("Found unresolved label: {}", label);
            }
        };
    }
    output
}
