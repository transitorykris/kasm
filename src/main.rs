mod files;
pub use crate::files::read_source;
pub use crate::files::write_out;

mod instructions;
pub use crate::instructions::generate_instruction_set;
pub use crate::instructions::get_instruction;
pub use crate::instructions::Mnemonic;
pub use crate::instructions::AddressMode;

const OUTSIZE: usize = 16384;       // We're generating binaries for a 16KB EEPROM
const OUTFILE: &str = "a.out";      // A typical default
const INFILE: &str = "example.s";   // Hardcode our source filename for the time being

fn main() {
    println!("kasm");

    // Read our source file
    let source = read_source(INFILE);
    println!("{}", source);

    // Get our instruction set
    let instruction_set = generate_instruction_set();

    println!("{:X}", get_instruction(&instruction_set, Mnemonic::LDA, AddressMode::Immediate));
    println!("{:X}", get_instruction(&instruction_set, Mnemonic::INC, AddressMode::Implied));
    println!("{:X}", get_instruction(&instruction_set, Mnemonic::STA, AddressMode::Zeropage));

    // Pass 1
    pass1(&source);

    // Pass 2
    pass2();

    // Assemble
    let output = assemble();

    // Write output file
    write_out(OUTFILE, output);
}

fn pass1(source: &str) {
    // Iterate over each line of source file
    // Identify directives, labels, and symbols
    // Strip comments -- Careful with string literals
    // Store labels in a label table to have their addresses determined later
    // Determine length in bytes the symbol requires
}

fn pass2() {
    // Determine the location of each of the labels
    // Update each reference to the symbol
}

fn assemble() -> Vec<u8> {
    let mut output = vec![0; OUTSIZE];
    output
}
