use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::string::String;

const OUTSIZE: usize = 16384;       // We're generating binaries for a 16KB EEPROM
const OUTFILE: &str = "a.out";      // A typical default
const INFILE: &str = "example.s";   // Hardcode our source filename for the time being

fn main() {
    println!("kasm");

    // Read our source file
    let source = read_source(INFILE);
    println!("{}", source);

    // Pass 1
    pass1(&source);

    // Pass 2
    pass2();

    // Assemble
    let output = assemble();

    // Write output file
    write_out(OUTFILE, output);
}

fn pass1(source: &str) {
}

fn pass2() {
}

fn assemble() -> Vec<u8> {
    let mut output = vec![0; OUTSIZE];
    output
}

fn read_source(file: &str) -> String {
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

fn write_out(filename: &str, output: Vec<u8>) {
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
