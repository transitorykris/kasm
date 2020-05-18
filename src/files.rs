use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::string::String;

pub fn read_source(file: &str) -> String {
    let path = Path::new(file);
    let display = path.display();
    let mut f = match File::open(path) {
        Err(why) => {
            panic!("Couldn't open {}: {}", display, why.to_string())
        },
        Ok(f) => f,
    };

    let mut source = String::new();
    let _ = match f.read_to_string(&mut source) {
        Err(why) => {
            panic!("Couldn't read {}: {}", display, why.to_string())
        },
        Ok(source) => source,
    };
    source
}

pub fn write_out(filename: &str, output: Vec<u8>) {
    let path = Path::new(filename);
    let display = path.display();
    let mut f = match File::create(&path) {
        Err(why) => {
            panic!("Couldn't create {}: {}", display, why.to_string())
        },
        Ok(f) => f,
    };

    let _ = match f.write_all(&output) {
        Err(why) => {
            panic!("Couldn't write {}: {}", display, why.to_string())
        },
        Ok(f) => f,
    };
}