use crate::arch::Serial;
use crate::log;
use core::fmt::Write;

pub struct Kmon<T: Serial> {
    pub port: T,
    recv_buf_pos: usize,
    pub recv_buf: [char; 256],
}

impl<T: Serial> Kmon<T> {
    fn is_ascii_printable(c: u8) -> bool {
        c >= 0x20 && c <= 0x7E
    }

    pub fn new(port: T) -> Self {
        Self {
            port,
            recv_buf_pos: 0,
            recv_buf: ['\0'; 256],
        }
    }

    fn handle_line(&mut self) {
        self.recv_buf[self.recv_buf_pos] = '\0';
        for i in 0..self.recv_buf_pos {
            log!("{}", self.recv_buf[i]);
        }
        log!("\n");
        self.recv_buf_pos = 0;
        self.print_term_begin();
    }

    fn handle_char(&mut self, c: char) {
        if Self::is_ascii_printable(c as u8) {
            log!("\x7F{}", c);
            log!("_\x08");
            self.recv_buf[self.recv_buf_pos] = c;
            self.recv_buf_pos += 1;
        } else if c == '\r' {
            /* Clear _ */
            log!(" \x08\n");
            if self.recv_buf_pos == 0 {
                self.print_term_begin();
            } else {
                self.handle_line();
            }
        } else if c == '\x08' || c == '\x7F' {
            if self.recv_buf_pos > 0 {
                self.recv_buf[self.recv_buf_pos] = '\0';
                self.recv_buf_pos -= 1;
                /* Ugliest hack in the world to fix backspace... */
                log!("\x7F \x08\x08 \x08_\x08");
            }
        } else {
            log!("\nUnknown character: {:x}\n", c as u8);

            /* Reset written content */
            self.recv_buf = ['\0'; 256];
            self.recv_buf_pos = 0;
            self.print_term_begin();
        }
    }

    pub fn repl_loop(&mut self) {
        log!("=================== [Serial Prompt v1.0] ===================\n");
        self.print_term_begin();
        loop {
            let c = self.port.read_char();
            self.handle_char(c);
        }
    }

    fn print_term_begin(&self) {
        log!(">>> _\x08");
    }
}
