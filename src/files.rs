use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::string::String;

use crate::errors::error;
use crate::errors::Error;
use crate::errors::ErrorMsg;
use crate::pass2::MachineCode;

pub fn read_source(file: &str) -> Result<String, (Error, ErrorMsg)> {
    let path = Path::new(file);
    let display = path.display();
    let mut f = match File::open(path) {
        Err(err) => {
            return Err(error(
                Error::FileOpen,
                format!("Couldn't open {}: {}", display, err.to_string()),
            ))
        }
        Ok(f) => f,
    };

    let mut raw_source = String::new();
    match f.read_to_string(&mut raw_source) {
        Err(why) => {
            return Err(error(
                Error::FileRead,
                format!("Couldn't read {}: {}", display, why.to_string()),
            ))
        }
        Ok(_) => {}
    };

    Ok(raw_source)
}

pub fn write_out(filename: &str, output: MachineCode) -> Result<(), (Error, ErrorMsg)> {
    let path = Path::new(filename);
    let display = path.display();
    let mut f = match File::create(&path) {
        Err(why) => {
            return Err(error(
                Error::FileCreate,
                format!("Couldn't create {}: {}", display, why.to_string()),
            ))
        }
        Ok(f) => f,
    };

    match f.write_all(&output) {
        Err(why) => {
            return Err(error(
                Error::FileWrite,
                format!("Couldn't write {}: {}", display, why.to_string()),
            ))
        }
        Ok(_) => {}
    };

    Ok(())
}
