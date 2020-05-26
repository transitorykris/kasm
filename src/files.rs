use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::string::String;

pub use crate::errors::Error;
pub use crate::pass2::MachineCode;

pub fn read_source(file: &str) -> String {
    let path = Path::new(file);
    let display = path.display();
    let mut f = match File::open(path) {
        Err(why) => error!(
            Error::FileOpen,
            "Couldn't open {}: {}",
            display,
            why.to_string()
        ),
        Ok(f) => f,
    };

    let mut raw_source = String::new();
    let _ = match f.read_to_string(&mut raw_source) {
        Err(why) => error!(
            Error::FileRead,
            "Couldn't read {}: {}",
            display,
            why.to_string()
        ),
        Ok(raw_source) => raw_source,
    };

    raw_source
}

pub fn write_out(filename: &str, output: MachineCode) {
    let path = Path::new(filename);
    let display = path.display();
    let mut f = match File::create(&path) {
        Err(why) => error!(
            Error::FileCreate,
            "Couldn't create {}: {}",
            display,
            why.to_string()
        ),
        Ok(f) => f,
    };

    let _ = match f.write_all(&output) {
        Err(why) => error!(
            Error::FileWrite,
            "Couldn't write {}: {}",
            display,
            why.to_string()
        ),
        Ok(f) => f,
    };
}
