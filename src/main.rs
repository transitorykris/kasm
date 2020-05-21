use std::env;

mod files;
pub use crate::files::read_source;
pub use crate::files::write_out;

mod instructions;
pub use crate::instructions::generate_instruction_set;
pub use crate::instructions::get_instruction;
pub use crate::instructions::Mnemonic;
pub use crate::instructions::AddressMode;
pub use crate::instructions::address_mode_length;
pub use crate::instructions::address_mode_name;

mod pass1;
pub use crate::pass1::pass1;

mod pass2;
pub use crate::pass2::pass2;

mod assemble;
pub use crate::assemble::assemble;

const OUTSIZE: usize = 16384;       // We're generating binaries for a 16KB EEPROM
const OUTFILE: &str = "a.out";      // A typical default

fn main() { 
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: {} <source>", &args[0]);
        return
    }
    let source_file = &args[1];

    // Read our source file
    let source = read_source(&source_file);

    // Get our instruction set
    let instruction_set = generate_instruction_set();

    println!("{:X}", get_instruction(&instruction_set, Mnemonic::LDA, AddressMode::Immediate));
    println!("{:X}", get_instruction(&instruction_set, Mnemonic::INC, AddressMode::Implied));
    println!("{:X}", get_instruction(&instruction_set, Mnemonic::STA, AddressMode::Zeropage));

    // Pass 1
    let pass1_code = pass1(&source);

    // Pass 2
    let pass2_code = pass2(pass1_code);

    // Assemble
    let output = assemble(instruction_set, pass2_code, OUTSIZE);

    // Write output file
    write_out(OUTFILE, output);
}
