type Line = u16;

pub fn scanner(raw_source: String) -> Vec<(String, Line)> {
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
            let (code, _) = line.split_at(comment_find.unwrap());
            line = String::from(code);
            line = line.trim().to_string(); // May have space between instruction and comment
            println!("stripped comment {}", line);
        }

        if line.len() == 0 {
            println!("Empty line, skipping");
            continue;
        }

        if line.starts_with(";") {
            println!("Comment found, skipping");
            continue;
        }
        source.push((line, count));
    }

    source
}
