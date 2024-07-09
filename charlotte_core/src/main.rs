#![no_std]
#![no_main]
#![warn(missing_copy_implementations)]

use core::fmt::Write;
use core::panic::PanicInfo;

use arch::{Api, ArchApi};

use crate::kmon::Kmon;

mod acpi;
mod arch;
mod bootinfo;
mod framebuffer;
mod kmon;
mod memory;

#[no_mangle]
static mut TIMER_CALLS: u64 = 0;

/// This is the kernel entrypoint function,
/// the first thing it does is call: [isa_init](ArchApi::isa_init)
/// you should check the documentation on that function for details,
/// since that contains all the ISA specific initialization code.
#[no_mangle]
unsafe extern "C" fn main() -> ! {
    let mut arch_api = ArchApi::isa_init();
    logln!("Bring up finished, starting kernel interactive prompt");
    arch_api.set_interrupt_handler(handle_timer, 32);
    let port = arch_api.get_serial();
    let mut mon = Kmon::new(port);
    mon.repl_loop();

    #[allow(clippy::empty_loop)]
    loop {}
}

fn handle_timer() {
    unsafe {
        TIMER_CALLS += 1;
    }
    ArchApi::end_of_interrupt();
}

#[panic_handler]
fn rust_panic(_info: &PanicInfo) -> ! {
    logln!("A kernel panic has occurred due to a Rust runtime panic.");
    logln!("PanicInfo: {:?}", _info);
    ArchApi::panic()
}
