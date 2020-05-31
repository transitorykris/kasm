type Line = u16;

pub struct SourceLine {
    pub line: String,
    pub line_number: Line,
}

pub type SourceTable = Vec<SourceLine>;

pub fn scanner(raw_source: String) -> SourceTable {
    let mut source = SourceTable::new();

    for (mut line_number, raw_line) in raw_source.split('\n').enumerate() {
        //for raw_line in raw_source.split('\n') {
        let mut line = String::from(raw_line);
        line = line.trim().to_string();

        // We count all the lines to help the programmer with
        // finding errors later
        line_number += 1;

        // Strip out comments
        // TODO: take care to handle semicolons in strings
        if let Some(comment_find) = line.find(';') {
            let (code, _) = line.split_at(comment_find);
            line = String::from(code).trim().to_string();
        }

        if line.is_empty() {
            continue;
        }

        if line.starts_with(';') {
            continue;
        }

        source.push(SourceLine {
            line,
            line_number: line_number as u16,
        });
    }

    source
}
