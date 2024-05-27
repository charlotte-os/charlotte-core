use core::fmt::{self, Write};
use spin::Mutex;
use crate::framebuffer::framebuffer::FRAMEBUFFER;
use crate::framebuffer::colors::Color;

pub struct Logger;

impl Logger {
    pub fn init() {
        // Initialization code if needed. For now, there's nothing specific to initialize.
    }

    pub fn log(&self, message: &str) {
        let mut framebuffer = FRAMEBUFFER.lock();
        writeln!(framebuffer, "{}", message).unwrap();
    }
}

impl Write for Logger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut framebuffer = FRAMEBUFFER.lock();
        framebuffer.write_str(s)
    }
}

static LOGGER: Mutex<Logger> = Mutex::new(Logger {});

pub fn log(message: &str) {
    LOGGER.lock().log(message);
}

#[macro_export]
macro_rules! logln {
    ($($arg:tt)*) => ({
        $crate::logging::logger::log(&format!($($arg)*));
    })
}
