// Stuff for hanlding errors encountered in the assembler
#![macro_use]

use std::fmt;

// We don't have anything to clean up in this assembler
// bailing anywhere in the code should be safe
#[macro_export]
macro_rules! error {
    ($error:expr, $fmt:expr) => {
        {
            use std::process;
            println!(concat!($fmt));
            process::exit($error as i32);
        }
    };
    ($error:expr, $fmt:expr, $($arg:tt)*) => {
        {
            use std::process;
            println!(concat!($fmt), $($arg)*);
            process::exit($error as i32);
        }
    };
}

#[macro_export]
macro_rules! verbose {
    ($fmt:expr) => (print!(concat!($fmt)));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt), $($arg)*));
}

#[macro_export]
macro_rules! verboseln {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! warning {
    ($fmt:expr) => (println!(concat!($fmt)));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt), $($arg)*));
}

pub enum Error {
    Good = 0,
    Usage,
    OverwriteSource,
    FileOpen,
    FileRead,
    FileCreate,
    FileWrite,
    NoValidOpcode,
    UnknownInstruction,
    UnknownEscapeCode,
    UnknownLabel,
    UnknownSyntax,
    DuplicateLabel,
    UnknownDirective,
}

pub type Msg = String;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This is my string")
    }
}

fn error_to_string(code: Error) -> String {
    match code {
        Good => String::from(""),
        Usage => String::from(""),
        OverwriteSource => String::from(""),
        FileOpen => String::from(""),
        FileRead => String::from(""),
        FileCreate => String::from(""),
        FileWrite => String::from(""),
        NoValidOpcode => String::from(""),
        UnknownInstruction => String::from(""),
        UnknownEscapeCode => String::from(""),
        UnknownLabel => String::from(""),
        UnknownSyntax => String::from(""),
        DuplicateLabel => String::from(""),
        UnknownDirective => String::from(""),
    }
}
