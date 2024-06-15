#![no_std]
#![no_main]
#![warn(missing_copy_implementations)]

use core::fmt::Write;

use arch::{Api, ArchApi};

use crate::kmon::Kmon;

mod acpi;
mod arch;
mod bootinfo;
mod framebuffer;
mod kmon;
mod memory;

/// This is the kernel entrypoint function,
/// the first thing it does is call: [isa_init](ArchApi::isa_init)
/// you should check the documentation on that function for details,
/// since that contains all the ISA specific initialization code.
#[no_mangle]
unsafe extern "C" fn main() -> ! {
    let arch_api = ArchApi::isa_init();

    logln!("Bring up finished, starting kernel interactive prompt");
    let port = arch_api.get_serial();
    let mut mon = Kmon::new(port);
    mon.repl_loop();
    loop {}
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    logln!("A kernel panic has occurred due to a Rust runtime panic.");
    logln!("PanicInfo: {:?}", _info);
    ArchApi::panic()
}
