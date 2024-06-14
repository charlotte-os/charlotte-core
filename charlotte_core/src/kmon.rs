use crate::arch::Serial;
use core::fmt::Write;
use crate::log;

pub struct Kmon<T:Serial> {
    pub port: T,
    recv_buf_pos: usize,
    pub recv_buf: [char; 256],
    pub recv_buf_used: usize,
}

impl<T:Serial> Kmon<T> {
    fn is_ascii_printable(c: u8) -> bool {
        c >= 0x20 && c <= 0x7E
    }

    pub fn new(port: T) -> Self {
        Self {
            port,
            recv_buf_pos: 0,
            recv_buf_used: 0,
            recv_buf: ['\0'; 256],
        }
    }

    fn handle_line(&mut self) {
        self.recv_buf[self.recv_buf_pos] = '\0';
        self.recv_buf_used = self.recv_buf_pos;
        self.recv_buf_pos = 0;
        for i in 0..self.recv_buf_used {
            log!("{}", self.recv_buf[i]);
        }
    }

    fn handle_char(&mut self, c: char) {
        if Self::is_ascii_printable(c as u8) {
            log!("\x7F{}",c);
            log!("_\x08");
            self.recv_buf[self.recv_buf_pos] = c;
            self.recv_buf_pos += 1;
        } else if c == '\r' {
            self.handle_line();
        } else if c == '\x08' || c == '\x7F'{
            if self.recv_buf_pos > 0 {
                self.recv_buf[self.recv_buf_pos] = '\0';
                self.recv_buf_pos -= 1;
                log!("\x08\x7F");
            }
        } else {
            log!("Unknown character: {:x}\n", c as u8);
        }
    }

    pub fn repl_loop(&mut self) {
        loop {
            let c = self.port.read_char();
            self.handle_char(c);
        }
    }
}