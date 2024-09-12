#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        write!(crate::framebuffer::console::CONSOLE.lock(), "{}", format_args!($($arg)*));
        write!(crate::uart::com1.lock(), "{}", format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! logln {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        writeln!(crate::framebuffer::console::CONSOLE.lock(), "{}", format_args!($($arg)*));
        writeln!(crate::uart::com1.lock(), "{}", format_args!($($arg)*));
    }};
}