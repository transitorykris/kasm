use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::string::String;

pub use crate::pass2::MachineCode;

pub fn read_source(file: &str) -> String {
    let path = Path::new(file);
    let display = path.display();
    let mut f = match File::open(path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.to_string()),
        Ok(f) => f,
    };

    let mut raw_source = String::new();
    let _ = match f.read_to_string(&mut raw_source) {
        Err(why) => panic!("Couldn't read {}: {}", display, why.to_string()),
        Ok(raw_source) => raw_source,
    };

    raw_source
}

pub fn write_out(filename: &str, output: MachineCode) {
    let path = Path::new(filename);
    let display = path.display();
    let mut f = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why.to_string()),
        Ok(f) => f,
    };

    let _ = match f.write_all(&output) {
        Err(why) => panic!("Couldn't write {}: {}", display, why.to_string()),
        Ok(f) => f,
    };
}
