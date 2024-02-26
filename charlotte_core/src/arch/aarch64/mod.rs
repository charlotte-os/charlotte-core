use core::arch::asm;

pub mod memory;
pub mod uart;

use uart::Uart;

pub struct Api;

/// Provide the implementation of the Api trait for the Api struct
impl crate::arch::Api for Api {
    /// Define the logger type
    type DebugLogger = Uart;

    /// Get a new logger instance
    fn get_logger() -> Self::DebugLogger {
        Uart::new()
    }
    /// Get the number of significant physical address bits supported by the current CPU
    fn get_paddr_width() -> u8 {
        *memory::PADDR_SIG_BITS
    }
    /// Get the number of significant virtual address bits supported by the current CPU
    fn get_vaddr_width() -> u8 {
        *memory::VADDR_SIG_BITS
    }
    /// Halt the calling LP
    fn halt() -> ! {
        // TODO: + disable IRQs, when they are ready
        unsafe { asm!("wfi") };
        loop {}
    }
    /// Kernel Panic
    fn panic() -> ! {
        Self::halt()
    }

    /// Read a byte from the specified port
    fn inb(_port: u16) -> u8 {
        todo!()
    }

    /// Write a byte to the specified port
    fn outb(_port: u16, _val: u8) {
        todo!()
    }
    /// Initialize the bootstrap processor (BSP)
    fn init_bsp() {}
    ///
    ///  Initialize the application processors (APs)
    fn init_ap() {
        //! This routine is run by each application processor to initialize itself prior to being handed off to the scheduler.
        todo!()
    }
}
