use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::string::String;

use crate::errors::error;
use crate::errors::Error;
use crate::errors::ErrorCode;
use crate::pass2::MachineCode;
use crate::Config;

pub fn read_source(file: &str) -> Result<String, Error> {
    let path = Path::new(file);
    let display = path.display();
    let mut f = match File::open(path) {
        Err(err) => {
            return Err(error(
                ErrorCode::FileOpen,
                format!("Couldn't open {}: {}", display, err.to_string()),
            ))
        }
        Ok(f) => f,
    };

    let mut raw_source = String::new();
    if let Err(why) = f.read_to_string(&mut raw_source) {
        return Err(error(
            ErrorCode::FileRead,
            format!("Couldn't read {}: {}", display, why.to_string()),
        ));
    };

    Ok(raw_source)
}

pub fn write_out(config: &Config, output: MachineCode) -> Result<(), Error> {
    let path = Path::new(&config.out_file);
    let display = path.display();
    let mut f = match File::create(&path) {
        Err(why) => {
            return Err(error(
                ErrorCode::FileCreate,
                format!("Couldn't create {}: {}", display, why.to_string()),
            ))
        }
        Ok(f) => f,
    };

    let mut final_output: Vec<u8> = Vec::new();

    // Create out initial padding
    let mut padding = vec![0x00; config.padding as usize];
    final_output.append(&mut padding);

    // Write out code
    for code in output {
        final_output.push(code);
    }

    // Pad remainder of file to size required
    if config.size > 0 && final_output.len() > config.size as usize {
        warning!("Warning! Final output is larger than desired size");
    }
    for _ in final_output.len()..config.size as usize {
        final_output.push(0x00);
    }

    // Write the final output to the output file
    if let Err(why) = f.write_all(&final_output) {
        return Err(error(
            ErrorCode::FileWrite,
            format!("Couldn't write {}: {}", display, why.to_string()),
        ));
    };

    Ok(())
}
