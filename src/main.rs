use std::env;
use std::process;

mod ascii;

mod errors;
pub use crate::errors::Error;

mod files;
pub use crate::files::read_source;
pub use crate::files::write_out;

mod instructions;
pub use crate::instructions::generate_instruction_set;

mod pass1;
pub use crate::pass1::pass1;

mod pass2;
pub use crate::pass2::pass2;

mod scanner;
pub use crate::scanner::scanner;

const OUTFILE: &str = "a.out"; // A typical default

fn usage(cmd: &str) {
    println!("usage: {} [-o <outfile>] <source>", cmd);
}

fn main() {
    // Process command line options
    let mut raw_args: Vec<String> = env::args().collect();
    let command = raw_args[0].to_string();
    if raw_args.len() < 2 {
        usage(&command);
        process::exit(Error::Usage as i32);
    }

    let mut args: Vec<String> = raw_args.drain(1..).collect(); // Remove first arg
    let mut out_file = String::from(OUTFILE);
    let source_file = args.pop().unwrap();
    let mut temp_val = String::new();

    while args.len() > 0 {
        let val = args.pop().unwrap();
        if val == "-o" {
            if temp_val == "" {
                // We don't have a file name!
                usage(&command);
                process::exit(Error::Usage as i32);
            }
            out_file = temp_val.to_string();
            temp_val = String::from("");
            continue;
        }
        temp_val = val;
    }

    // If there's a value in temp_val we're missing a flag!
    if temp_val != "" {
        usage(&command);
        process::exit(Error::Usage as i32);
    }

    if source_file == out_file {
        error!(
            Error::OverwriteSource as i32,
            "You really don't want to overwrite your source file"
        );
    }

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

    write_out(&out_file.to_string(), output);
}
