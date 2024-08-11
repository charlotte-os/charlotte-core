#![no_std]
#![no_main]

extern crate alloc;
extern crate spin;

use core::sync::atomic::{AtomicBool, Ordering};
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore));
    static ref SHIFT_PRESSED: AtomicBool = AtomicBool::new(false);
}

fn read_scancode() -> u8 {
    let mut port = Port::new(0x60);
    unsafe { port.read() }
}

fn handle_scancode(scancode: u8) {
    let mut keyboard = KEYBOARD.lock();
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => {
                    if key == pc_keyboard::KeyCode::LShift || key == pc_keyboard::KeyCode::RShift {
                        SHIFT_PRESSED.store(true, Ordering::SeqCst);
                    } else {
                        SHIFT_PRESSED.store(false, Ordering::SeqCst);
                    }
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn ps2_keyboard_handler() {
    let scancode = read_scancode();
    handle_scancode(scancode);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {

    }
}
