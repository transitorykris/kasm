pub mod errors;
use errors::Error;
use errors::ErrorCode;

#[macro_use]
mod strings;

mod ascii;

mod files;
use files::read_source;
use files::write_out;

pub mod instructions;
use instructions::generate_instruction_set;

pub mod pass1;
use pass1::pass1;

pub mod pass2;
use pass2::pass2;

mod scanner;
use scanner::scanner;

const OUTFILE_DEFAULT: &str = "a.out"; // A typical default

pub struct Config {
    source_file: String,
    out_file: String,

    // These config options are added because the assembler is
    // generally used for producing a file to burn to EEPROM
    padding: u16, // The number of 0x00 bytes to pad at start of file
    size: u16,    // The total size of file, pads 0x00 at end
}

pub fn usage(cmd: &str) {
    println!(
        "usage: {} [-o <outfile>] [-p <padding>] [-s <size>] <source>",
        cmd
    );
}

impl Config {
    pub fn new(args: &mut Vec<String>) -> Result<Config, (ErrorCode, &'static str)> {
        // Process command line options
        if args.len() < 2 {
            return Err((ErrorCode::Usage, "Missing arguments"));
        }

        let mut args: Vec<String> = args.drain(1..).collect(); // Remove first arg
        let mut out_file = String::from(OUTFILE_DEFAULT);
        let source_file = args.pop().unwrap();
        let mut temp_val = String::new();
        let mut padding = 0;
        let mut size = 0;

        while !args.is_empty() {
            let val = args.pop().unwrap();
            if val == "-o" {
                if temp_val == "" {
                    return Err((ErrorCode::Usage, "No output filename provided"));
                }
                out_file = temp_val.to_string();
                temp_val = String::from("");
                continue;
            } else if val == "-p" {
                if temp_val == "" {
                    return Err((ErrorCode::Usage, "No padding size provided provided"));
                }
                match u16::from_str_radix(temp_val.as_str(), 10) {
                    Ok(pad_val) => {
                        temp_val = String::from("");
                        padding = pad_val;
                        continue;
                    }
                    Err(_) => return Err((ErrorCode::Usage, "Invalid padding size")),
                };
            } else if val == "-s" {
                if temp_val == "" {
                    return Err((ErrorCode::Usage, "No file size provided provided"));
                }
                match u16::from_str_radix(temp_val.as_str(), 10) {
                    Ok(size_val) => {
                        temp_val = String::from("");
                        size = size_val;
                        continue;
                    }
                    Err(_) => return Err((ErrorCode::Usage, "Invalid file size")),
                };
            }
            temp_val = val;
        }

        // If there's a value in temp_val we're missing a flag!
        if temp_val != "" {
            return Err((ErrorCode::Usage, "Missing arguments"));
        }

        if source_file == out_file {
            return Err((
                ErrorCode::OverwriteSource,
                "You really don't want to overwrite your source file",
            ));
        }

        Ok(Config {
            source_file,
            out_file,
            padding: padding,
            size: size,
        })
    }
}

pub fn run(config: &Config) -> Result<(), Error> {
    // Read in the source file
    let source = match read_source(&config.source_file) {
        Ok(source) => source,
        Err(err) => return Err(err),
    };

    // Scan in and clean up the raw
    let scanned = scanner(source);

    // Create a data structure containing details of our
    // instruction set
    let instruction_set = generate_instruction_set();

    // Create a data structure containing the instruction,
    // the addressing mode, and the value
    let pass1_code = match pass1(scanned) {
        Ok(pass1_code) => pass1_code,
        Err(err) => return Err(err),
    };

    // Create a new data structure of instructions by resolving
    // all the labels
    let output = match pass2(instruction_set, pass1_code) {
        Ok(output) => output,
        Err(err) => return Err(err),
    };

    match write_out(&config, output) {
        Ok(_) => {}
        Err(err) => return Err(err),
    };

    Ok(())
}
