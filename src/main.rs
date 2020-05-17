use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::string::String;

const OUTSIZE: usize = 16384;       // We're generating binaries for a 16KB EEPROM
const OUTFILE: &str = "a.out";      // A typical default

fn main() {
    println!("kasm");

    // Read source file
    let source = String::from("\
; This is my simple program

    lda #$00            ; Load 0 into the A register
    inc                 ; Add 1 to it
    sta $00             ; Store it in the zeropage at $00
");

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
