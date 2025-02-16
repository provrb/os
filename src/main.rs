#![no_std]
#![no_main]

#![feature(abi_x86_interrupt)]

pub mod gdt;
pub mod interrupts;
pub mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::{set_print_color, ColorDesc};
use crate::gdt::init_gdt;
use crate::interrupts::{init_pic, init_idt};

pub fn init() {
    init_pic();
    init_gdt();
    init_idt();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();

    set_print_color(ColorDesc::new(vga_buffer::Color::White, vga_buffer::Color::Black));
    println!("welcome.");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("panic! {_info}");

    loop {}
}
