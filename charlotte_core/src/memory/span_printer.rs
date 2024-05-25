//! Print a span of memory starting at some address with some width

use core::{fmt::Write, usize};

use crate::{log, logln};

#[derive(Clone, Copy)]
pub struct MemorySpan {
    start: usize,
    width: usize,
}

#[derive(Clone, Copy)]
pub struct MemorySpanIter {
    span: MemorySpan,
    offset: usize,
}

impl MemorySpan {
    pub fn new(start: usize, width: usize) -> Self {
        MemorySpan { start, width }
    }

    pub fn print_span(&self, width: usize, raw_dump: bool) {
        let mut iter = self.iter();
        let mut idx = 0;
        logln!(
            "Printing span starting at {:X} {:X} wide",
            self.start,
            self.width
        );
        while let Some(byte) = iter.next() {
            if raw_dump {
                if idx % width == 0 {
                    if idx == 0 {
                        if byte < 15 {
                            log!("\n0{:X} ", byte);
                        } else {
                            log!("\n{:X} ", byte);
                        }
                    } else {
                        if byte < 15 {
                            log!("\n0{:X} ", byte);
                        } else {
                            log!("\n{:X} ", byte);
                        }
                    }
                } else {
                    if byte < 15 {
                        log!("0{:X} ", byte);
                    } else {
                        log!("{:X} ", byte);
                    }
                }
            } else {
                if idx % width == 0 {
                    if idx == 0 {
                        if byte < 15 {
                            log!("{:X} | 0{:X} ", self.start + idx, byte);
                        } else {
                            log!("{:X} | {:X} ", self.start + idx, byte);
                        }
                    } else {
                        if byte < 15 {
                            log!("\n{:X} | 0{:X} ", self.start + idx, byte);
                        } else {
                            log!("\n{:X} | {:X} ", self.start + idx, byte);
                        }
                    }
                } else {
                    if byte < 15 {
                        log!("0{:X} ", byte);
                    } else {
                        log!("{:X} ", byte);
                    }
                }
            }
            idx += 1;
        }
        logln!("\nEND OF SPAN");
    }

    pub fn iter(&self) -> MemorySpanIter {
        MemorySpanIter {
            span: *self,
            offset: 0,
        }
    }
}

impl Iterator for MemorySpanIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset <= self.span.width {
            let byte = Some(unsafe { *((self.span.start + self.offset) as *const u8) });
            self.offset += 1;
            byte
        } else {
            None
        }
    }
}
