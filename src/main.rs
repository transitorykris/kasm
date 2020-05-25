use std::env;

mod files;
pub use crate::files::read_source;
pub use crate::files::write_out;

mod instructions;
pub use crate::instructions::generate_instruction_set;

mod scanner;
pub use crate::scanner::scanner;

mod pass1;
pub use crate::pass1::pass1;

mod pass2;
pub use crate::pass2::pass2;

const OUTFILE: &str = "a.out"; // A typical default

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: {} <source>", &args[0]);
        return;
    }
    let source_file = &args[1];

    // Read in the source file
    let source = read_source(&source_file);

    // Scan in and clean up the raw
    let scanned = scanner(source);

    // Create a data structure containing details of our
    // instruction set
    let instruction_set = generate_instruction_set();

    // Create a data structure containing the instruction,
    // the addressing mode, and the value
    let pass1_code = pass1(&scanned);

    // Create a new data structure of instructions by resolving
    // all the labels
    let output = pass2(instruction_set, pass1_code);

    write_out(OUTFILE, output);
}
