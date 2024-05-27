use core::fmt::{self, Write};
use spin::Mutex;
use crate::framebuffer::framebuffer::FRAMEBUFFER;
use crate::framebuffer::colors::Color;

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

pub struct Logger {
    level: LogLevel,
}

impl Logger {
    pub fn init(level: LogLevel) {
        *LOGGER.lock() = Logger { level };
    }

    pub fn log(&self, level: LogLevel, message: &str) {
        if level as u8 <= self.level as u8 {
            let mut framebuffer = FRAMEBUFFER.lock();
            writeln!(framebuffer, "[{:?}] {}", level, message).unwrap();
        }
    }
}

impl Write for Logger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut framebuffer = FRAMEBUFFER.lock();
        framebuffer.write_str(s)
    }
}

static LOGGER: Mutex<Logger> = Mutex::new(Logger { level: LogLevel::Info });

pub fn log(level: LogLevel, message: &str) {
    LOGGER.lock().log(level, message);
}

#[macro_export]
macro_rules! logln {
    ($level:expr, $($arg:tt)*) => ({
        $crate::logging::logger::log($level, &format!($($arg)*));
    })
}