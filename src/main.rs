use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::string::String;

const OUTSIZE: usize = 16384;       // We're generating binaries for a 16KB EEPROM
const OUTFILE: &str = "a.out";      // A typical default
const INFILE: &str = "example.s";   // Hardcode our source filename for the time being

fn main() {
    println!("kasm");

    // Read source file
    let path = Path::new(INFILE);
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
        Ok(f) => f,
    };
    println!("{}", source);

    // Parse source file
    
    // Resolve references

    let mut output: Vec<u8> = Vec::with_capacity(OUTSIZE);

    for _index in 1..OUTSIZE {       // Zero out our outout file
        output.push(0);
    }

    // Write output file
    let path = Path::new(OUTFILE);
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
