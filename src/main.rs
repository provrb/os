#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

pub mod gdt;
pub mod interrupts;
pub mod mem;
pub mod time;
pub mod vga_buffer;
mod cmos;

use crate::gdt::init_gdt;
use crate::interrupts::{init_idt, init_pic};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use mem::BootInfoFrameAllocator;
use time::DateTime;
use vga_buffer::{set_print_color, ColorDesc};
use x86_64::{PrivilegeLevel, VirtAddr};

pub fn init(boot_info: &'static BootInfo) {
    init_pic();
    init_gdt();
    init_idt();

    // init heap with boot info
    let phy_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::new(&boot_info.memory_map) };
    let mut mapper = unsafe { mem::new_offset_page_table(phy_mem_offset) };
    mem::heap_init(&mut mapper, &mut frame_allocator).expect("hello");
}

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);

    // set_print_color(ColorDesc::new(
    //     vga_buffer::Color::White,
    //     vga_buffer::Color::Black,
    // ));



    println!("{:?}", DateTime::now());

    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("panic! {info}");

    loop {
        x86_64::instructions::hlt();
    }
}
