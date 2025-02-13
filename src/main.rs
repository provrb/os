#![no_std]
#![no_main]

use core::panic::PanicInfo;

static MSG: &[u8] = b"Hello world!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in MSG.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // actual text
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // color, light cyan
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}