// Helpers for handling ascii

pub fn ascii_to_bytes(ascii: String) -> (Vec<u8>, u16) {
    let mut data = Vec::new();
    let mut size = 0;
    let mut in_escape = false; // track whether we're in an escape or not
    for ch in ascii.bytes() {
        if in_escape {
            data.push(unescape(ch));
            size += 1;
            in_escape = false;
        } else if ch == 0x5c {
            // 0x5c is ascii backslash
            in_escape = true;
        } else {
            data.push(ch);
            size += 1;
        }
    }
    (data, size)
}

pub fn unescape(ch: u8) -> u8 {
    // Standard C escape sequences
    match ch {
        0x61 => return 0x07, // \a Alert
        0x62 => return 0x08, // \b Backspace
        0x65 => return 0x1b, // \e Escape
        0x66 => return 0x0c, // \f Formfeed
        0x6e => return 0x0a, // \n Newline
        0x72 => return 0x0d, // \r Carriage Return
        0x74 => return 0x09, // \t Horizontal Tab
        0x7c => return 0x0b, // \v Vertical Tab
        0x5c => return 0x5c, // \\ Backslash
        0x27 => return 0x27, // \' Apostrophe
        0x22 => return 0x22, // \" Double Quotation Mark
        0x3f => return 0x34, // \? Question Mark
        _ => {}
    }
    //warning!("Unknown escape code: \\{}", ch);
    ch
}

#[cfg(test)]
mod tests {
    use super::ascii_to_bytes;

    #[test]
    fn test_ascii_to_bytes() {
        // We escape the escapes because .to_string() unescapes
        let (a, s) = ascii_to_bytes("Hello\\nWorld!\\r".to_string());
        assert_eq!(s, 13);
        assert_eq!(a, vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x0a, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x0d]);
    }

    #[test]
    fn test_unknown_escape_to_bytes() {
        // We escape the escapes because .to_string() unescapes
        let (a, s) = ascii_to_bytes("\\H".to_string());
        assert_eq!(s, 1);
        assert_eq!(a, vec![0x48]);
    }

    use super::unescape;

    #[test]
    fn test_unescape() {
        let value = unescape(0x61);
        assert_eq!(0x07, value);
    }

    #[test]
    fn test_unknown_escape() {
        let value = unescape(0x00);
        assert_eq!(0x00, value);
    }
}
