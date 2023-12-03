#![no_std]
#![no_main]

mod arch;

use core::fmt::Write;

use arch::{Arch, ArchApi};

#[no_mangle]
unsafe extern "C" fn main() -> ! {
        let mut logger = ArchApi::get_logger();
        write!(&mut logger, "Initializing BSP\n").unwrap();
        ArchApi::init_bsp();
        write!(&mut logger, "BSP Initialized\n").unwrap();
        ArchApi::halt()
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
        ArchApi::halt()
}