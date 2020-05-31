use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::string::String;

use crate::errors::error;
use crate::errors::Error;
use crate::errors::ErrorCode;
use crate::pass2::MachineCode;

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

pub fn write_out(filename: &str, output: MachineCode) -> Result<(), Error> {
    let path = Path::new(filename);
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

    if let Err(why) = f.write_all(&output) {
        return Err(error(
            ErrorCode::FileWrite,
            format!("Couldn't write {}: {}", display, why.to_string()),
        ));
    };

    Ok(())
}
