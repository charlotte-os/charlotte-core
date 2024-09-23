/// A struct representing a serial port address
/// This can be either a memory-mapped IO address (linear address) or an IO port
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct SerialAddr {
    addr: u64,
}

impl SerialAddr {
    /// # Create a new SerialAddr from a linear address
    /// ## Arguments
    /// * `addr` - The linear address of the serial port
    /// ## Returns
    /// A new SerialAddr
    pub const fn new(addr: u64) -> Self {
        SerialAddr { addr }
    }
    /// # Read a byte from the serial port
    /// ## Returns
    /// The byte read from the serial port
    pub fn read(&self) -> u8 {
        // Safety: The serial port is tested in the constructor
        unsafe { core::ptr::read_volatile(self.addr as *const u8) }
    }
    /// # Write a byte to the serial port
    /// ## Arguments
    /// * `data` - The byte to write to the serial port
    pub fn write(&self, data: u8) {
        // Safety: The serial port is tested in the constructor
        unsafe { core::ptr::write_volatile(self.addr as *mut u8, data) }
    }
}

impl core::ops::Add<usize> for SerialAddr {
    type Output = Self;

    fn add(self, rhs: usize) -> Self {
        SerialAddr::new(self.addr + rhs as u64)
    }
}
