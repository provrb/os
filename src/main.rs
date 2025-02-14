#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use gdt::init_gdt;
use interrupts::init_idt;

mod gdt;
mod interrupts;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init_idt();
    init_gdt();
    println!("hello world");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print!("{}", _info);
    loop {}
}
