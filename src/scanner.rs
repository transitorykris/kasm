type Line = u16;

pub struct SourceLine {
    pub line: String,
    pub line_number: Line,
}

pub type SourceTable = Vec<SourceLine>;

pub fn scanner(raw_source: String) -> SourceTable {
    let mut source = SourceTable::new();
    let mut line_number = 0;

    for raw_line in raw_source.split('\n') {
        let mut line = String::from(raw_line);
        line = line.trim().to_string();

        // We count all the lines to help the programmer with
        // finding errors later
        line_number = line_number + 1;

        // Strip out comments
        // TODO: take care to handle semicolons in strings
        let comment_find = line.find(';');
        if comment_find.is_some() {
            let (code, _) = line.split_at(comment_find.unwrap());
            line = String::from(code).trim().to_string();
        }

        if line.len() == 0 {
            continue;
        }

        if line.starts_with(";") {
            continue;
        }

        source.push(SourceLine { line, line_number });
    }

    source
}