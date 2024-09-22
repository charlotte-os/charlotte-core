///! Logging macros for the kernel.

/// Log a message to the console and the UART.
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        use ignore_result::Ignore;

        write!(crate::framebuffer::console::CONSOLE.lock(), "{}", format_args!($($arg)*)).ignore();
        #[cfg(target_arch = "x86_64")]
        write!(crate::uart::com1.lock(), "{}", format_args!($($arg)*)).ignore();
    }};
}
/// Log a message to the console and the UART with a newline appended.
#[macro_export]
macro_rules! logln {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        use ignore_result::Ignore;

        writeln!(crate::framebuffer::console::CONSOLE.lock(), "{}", format_args!($($arg)*)).ignore();
        #[cfg(target_arch = "x86_64")]
        writeln!(crate::uart::com1.lock(), "{}", format_args!($($arg)*)).ignore();
    }};
}