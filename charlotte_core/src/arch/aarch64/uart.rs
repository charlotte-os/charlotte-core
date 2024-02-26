use core::fmt::Write;

pub struct Uart {}

impl Uart {
    pub fn new() -> Self {
        Self {}
    }
}

// ToDo: pl011 uart when vmm is done
impl Write for Uart {
    fn write_str(&mut self, _s: &str) -> core::fmt::Result {
        Ok(())
    }
    fn write_char(&mut self, _c: char) -> core::fmt::Result {
        Ok(())
    }
}
