use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::string::String;

type Line = u16;

pub fn read_source(file: &str) -> Vec<(String, Line)> {
    let path = Path::new(file);
    let display = path.display();
    let mut f = match File::open(path) {
        Err(why) => {
            panic!("Couldn't open {}: {}", display, why.to_string())
        },
        Ok(f) => f,
    };

    let mut raw_source = String::new();
    let _ = match f.read_to_string(&mut raw_source) {
        Err(why) => {
            panic!("Couldn't read {}: {}", display, why.to_string())
        },
        Ok(raw_source) => raw_source,
    };

    let mut source = Vec::new();
    let mut count = 0;

    for raw_line in raw_source.split('\n') {
        let mut line = String::from(raw_line);
        println!("raw line {}", line);
        line = line.trim().to_string();
        println!("trimmed line {}", line);

        // We count all the lines to help the programmer with
        // finding errors later
        count = count + 1;

        // Strip out comments
        // TODO: take care to handle semicolons in strings
        let comment_find = line.find(';');
        if comment_find.is_some() {
            let (code, comment) = line.split_at(comment_find.unwrap());
            line = String::from(code);
            line = line.trim().to_string();   // May have space between instruction and comment
            println!("stripped comment {}", line);
        }

        if line.len() == 0 {
            println!("Empty line, skipping");
            continue
        }

        if line.starts_with(";") {
            println!("Comment found, skipping");
            continue
        }
        source.push((line, count));
    }

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