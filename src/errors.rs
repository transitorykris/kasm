// Stuff for hanlding errors encountered in the assembler
#![macro_use]

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

pub enum Error {
    None = 0,
    Usage,
    OverwriteSource,
    FileOpen,
    FileRead,
    FileCreate,
    FileWrite,
    NoValidOpcode,
    UnknownInstruction,
    UnknownEscapeCode,
}
