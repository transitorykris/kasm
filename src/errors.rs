// Stuff for hanlding errors encountered in the assembler
#![macro_use]

// We don't have anything to clean up in this assembler
// bailing anywhere in the code should be safe
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

pub fn error(code: ErrorCode, msg: String) -> (ErrorCode, ErrorMsg) {
    (code, msg)
}
