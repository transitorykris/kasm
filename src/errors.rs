// Stuff for hanlding errors encountered in the assembler
#![macro_use]

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

// TODO: Should this be a struct so we can derive(Debug)?
pub type Error = (ErrorCode, ErrorMsg);

pub enum ErrorCode {
    NoError = 0,
    Usage,
    OverwriteSource,
    FileOpen,
    FileRead,
    FileCreate,
    FileWrite,
    NoValidOpcode,
    UnknownInstruction,
    UnknownLabel,
    UnknownSyntax,
    DuplicateLabel,
    UnknownDirective,
    AddressExpected,
    HexExpected,
    MalformedEqu,
}

pub type ErrorMsg = String;

pub fn error(code: ErrorCode, msg: String) -> Error {
    (code, msg)
}
