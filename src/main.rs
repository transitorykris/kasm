use std::env;
use std::process;

mod errors;
pub use crate::errors::Error;

mod ascii;

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

const OUTFILE_DEFAULT: &str = "a.out"; // A typical default

struct Config {
    source_file: String,
    out_file: String,
}

fn main() {
    // Get our configuration
    let mut args: Vec<String> = env::args().collect();
    let config = Config::new(&mut args);

    // Read in the source file
    let source = match read_source(&config.source_file) {
        Ok(source) => source,
        Err(err) => {
            println!("{}", err.1);
            process::exit(err.0 as i32);
        }
    };

    // Scan in and clean up the raw
    let scanned = scanner(source);

    // Create a data structure containing details of our
    // instruction set
    let instruction_set = generate_instruction_set();

    // Create a data structure containing the instruction,
    // the addressing mode, and the value
    let pass1_code = match pass1(&scanned) {
        Ok(pass1_code) => pass1_code,
        Err(err) => {
            println!("{}", err.1);
            process::exit(err.0 as i32);
        }
    };

    // Create a new data structure of instructions by resolving
    // all the labels
    let output = match pass2(instruction_set, pass1_code) {
        Ok(output) => output,
        Err(err) => {
            println!("{}", err.1);
            process::exit(err.0 as i32);
        }
    };

    match write_out(&config.out_file.to_string(), output) {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err.1);
            process::exit(err.0 as i32);
        }
    };

    process::exit(Error::Good as i32);
}

impl Config {
    fn new(args: &mut Vec<String>) -> Config {
        // Process command line options
        let command = args[0].to_string();
        if args.len() < 2 {
            usage(&command);
            process::exit(Error::Usage as i32);
        }

        let mut args: Vec<String> = args.drain(1..).collect(); // Remove first arg
        let mut out_file = String::from(OUTFILE_DEFAULT);
        // XXX UNWRAP
        let source_file = args.pop().unwrap();
        let mut temp_val = String::new();

        while args.len() > 0 {
            // XXX UNWRAP
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
            println!("You really don't want to overwrite your source file");
            process::exit(Error::OverwriteSource as i32);
        }

        Config {
            source_file,
            out_file,
        }
    }
}

fn usage(cmd: &str) {
    println!("usage: {} [-o <outfile>] <source>", cmd);
}
