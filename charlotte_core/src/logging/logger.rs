use crate::framebuffer::colors::Color;
use crate::framebuffer::console::CONSOLE;
use core::fmt::Write;
use spin::{Mutex, lazy::Lazy};

#[derive(Clone)]
pub enum LogLevel {
    Unset,
    Info,
    Warn,
    Ok_,
    Fail,
    Panic,
}

/* TODO: Fix this hack */
static submodule_name: Mutex<&str> = Mutex::new("Unset");
pub struct Logger {
    submodule_status: LogLevel,
}

impl Logger {
    pub fn init() -> Self {
        /* Init files, etc... */
        Logger {
            submodule_status: LogLevel::Unset,
        }
    }

    pub fn log(&mut self, level: LogLevel, module: &str, message: &str) {
        /* Log to console, file, etc */
        self.log_to_console(level, message, module);
    }

    pub fn begin(&mut self, module: &str) {
        self.submodule_status = LogLevel::Ok_;
        CONSOLE.lock().write_str("[      ]  ", None, None);
        CONSOLE.lock().write_str(&submodule_name.lock(), None, None);
    }

    pub fn end(&mut self, module: &str) {
        self.print_lvl_badge(self.submodule_status.clone());
    }

    pub fn panic(&mut self, panic_info: &core::panic::PanicInfo) {
        self.print_lvl_badge(LogLevel::Panic);
        CONSOLE.lock().write_fmt(format_args!(
            "{} panicked!\nPanic info:\n{}",
            &submodule_name.lock(), panic_info
        ));
    }

    fn log_to_console(&mut self, level: LogLevel, message: &str, module: &str) {
        let fmodule = if module.len() > 10 {
            &module[module.len() - 10..]
        } else {
            module
        };

        let fmessage = if message.len() > 10 {
            /* Length of log level badge and the rust module lenght is 20 */
            /* Hence CONSOLE_WIDTH - 20 is the max message length */
            &message[message.len() - (CONSOLE.lock().get_width() - 20)..]
        } else {
            message
        };

        self.print_lvl_badge(level);
        CONSOLE.lock().write_fmt(format_args!(
            "{}{:padding_len$}{}",
            fmessage,
            fmodule,
            padding_len = CONSOLE.lock().get_width() - 20 - fmessage.len()
        ));

        if message.len() > (CONSOLE.lock().get_width() - 20) {
            /* Recursively write out info */
            self.log_to_console(
                LogLevel::Unset,
                &message[(CONSOLE.lock().get_width() - 20)..],
                fmodule,
            );
        }
    }

    fn print_lvl_badge(&mut self, level: LogLevel) {
        CONSOLE.lock().write_char('[', None, None);
        match level {
            LogLevel::Unset => CONSOLE.lock().write_str("      ", None, None),
            LogLevel::Info => CONSOLE.lock().write_str(" INFO ", Some(Color::BLUE), None),
            LogLevel::Warn => CONSOLE
                .lock()
                .write_str(" WARN ", Some(Color::YELLOW), None),
            LogLevel::Ok_ => CONSOLE.lock().write_str("  OK  ", Some(Color::GREEN), None),
            LogLevel::Fail => {
                CONSOLE.lock().write_str(" FAIL ", Some(Color::RED), None);
                self.submodule_status = LogLevel::Fail;
            }
            LogLevel::Panic => {
                CONSOLE
                    .lock()
                    .write_str("PANIC!", Some(Color::YELLOW), Some(Color::RED))
            }
        }
    }
}

static LOGGER: Lazy<Mutex<Logger>> = Lazy::new(|| Mutex::new(Logger::init()));

pub fn log(level: LogLevel, message: &str, module: &str) {
    LOGGER.lock().log(level, message, module);
}

pub fn log_begin(module: &str) {
    LOGGER.lock().begin(module);
}

pub fn log_end(module: &str) {
    LOGGER.lock().end(module);
}

pub fn log_panic(info: &core::panic::PanicInfo) {
    LOGGER.lock().panic(info);
}

#[macro_export]
macro_rules! debug {
    (msg) => ({
        $crate::logging::logger::log($crate::logging::logger::LogLevel::Debug, msg, module_path!());
    })
}

#[macro_export]
macro_rules! info {
    (msg) => ({
        $crate::logging::logger::log($crate::logging::logger::LogLevel::Info, msg, module_path!());
    })
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ({
        $crate::logging::logger::log($crate::logging::logger::LogLevel::Warn, &format!($($arg)*), module_path!());
    })
}

#[macro_export]
macro_rules! fail {
    ($($arg:tt)*) => ({
        $crate::logging::logger::log($crate::logging::logger::LogLevel::Fail, &format!($($arg)*), module_path!());
    })
}

#[macro_export]
macro_rules! log_begin {
    ($($arg:tt)*) => ({
        $crate::logging::logger::log_begin(&format!($($arg)*));
    })
}

#[macro_export]
macro_rules! log_end {
    () => {{
        $crate::logging::logger::log_end();
    }};
}

#[macro_export]
macro_rules! log_panic {
    ($($arg:tt)*) => ({
        $crate::logging::logger::log_panic($($arg)*);
    })
}
