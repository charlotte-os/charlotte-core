use core::arch::asm;
use core::ops::Add;

/// The standard IO ports for the serial ports on the PC platform
#[allow(unused)]
pub enum ComPort {
    COM1 = 0x3F8,
    COM2 = 0x2F8,
    COM3 = 0x3E8,
    COM4 = 0x2E8,
    COM5 = 0x5F8,
    COM6 = 0x4F8,
    COM7 = 0x5E8,
    COM8 = 0x4E8,
}

/// A struct representing an IO port
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct IoPort {
    port: u16,
}

impl IoPort {
    /// # Create an IoPort struct
    /// ## Arguments
    /// * `port` - The port number
    /// ## Returns
    /// A new IoPort struct
    pub fn new(port: u16) -> Self {
        IoPort { port }
    }
    /// # Read a byte from an IO port
    /// ## Returns
    /// The byte read from the port
    pub unsafe fn read(&self) -> u8 {
        let result: u8;
        unsafe {
            asm!("in al, dx",
                in("dx") self.port,
                out("al") result);
        }
        result
    }
    /// # Write a byte to an IO port
    /// ## Arguments
    /// * `&self` - The IO port to write to
    /// * `value` - The byte to write to the port
    pub unsafe fn write(&self, value: u8) {
        unsafe {
            asm!("out dx, al",
                in("dx") self.port,
                in("al") value);
        }
    }
    /// # Get the port number
    /// ## Returns
    /// The port number
    pub fn number(&self) -> u16 {
        self.port
    }
}

/// A struct representing a serial port address
/// This can be either a memory-mapped IO address (linear address) or an IO port
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SerialAddr {
    Mmio(*mut u8),
    IoPort(IoPort),
}

impl SerialAddr {
    /// # Read a byte from the given address or IO port
    /// ## Returns
    /// The byte read from the address or port
    pub fn read(&mut self) -> u8 {
        match self {
            SerialAddr::Mmio(addr) => unsafe { addr.read_volatile() },
            SerialAddr::IoPort(port) => unsafe { port.read() },
        }
    }
    /// # Write a byte to the given address or IO port
    /// ## Arguments
    /// * `&mut self` - The address or port to write to
    /// * `value` - The byte to write to the address or port
    pub fn write(&mut self, value: u8) {
        match self {
            SerialAddr::Mmio(addr) => unsafe { addr.write_volatile(value) },
            SerialAddr::IoPort(port) => unsafe { port.write(value) },
        }
    }
}

impl Add<u16> for SerialAddr {
    /// The output type of adding a u16 to a SerialAddr
    type Output = SerialAddr;
    /// # Add a u16 to a SerialAddr
    /// ## Arguments
    /// * `self` - The SerialAddr to add to
    /// * `rhs` - The u16 to add
    /// ## Returns
    /// A new SerialAddr with the offset added
    fn add(self, rhs: u16) -> Self::Output {
        match self {
            SerialAddr::Mmio(addr) => {
                SerialAddr::Mmio((addr as *mut u8).wrapping_add(rhs as usize))
            }
            SerialAddr::IoPort(port) => SerialAddr::IoPort(IoPort::new(port.number() + rhs)),
        }
    }
}
