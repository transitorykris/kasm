#[macro_export]
macro_rules! str_to_u16 {
    ($fmt:expr) => {
        u16::from_str_radix($fmt, 16)
    };
}

macro_rules! str_to_u8 {
    ($fmt:expr) => {
        u8::from_str_radix($fmt, 16)
    };
}
