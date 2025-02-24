#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod cmos;
pub mod gdt;
pub mod interrupts;
pub mod mem;
pub mod time;
pub mod vga_buffer;

use crate::gdt::init_gdt;
use crate::interrupts::{init_idt, init_pic};
use bootloader::{entry_point, BootInfo};
use gdt::user_main;
use x86_64::structures::paging::mapper::MapToError;
use x86_64::structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB, Translate};
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
    let virt_addr: VirtAddr = VirtAddr::new(user_main as *const () as u64);

    
    let frame = frame_allocator
        .allocate_frame()
        .unwrap();
    let flags =
        PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE | PageTableFlags::WRITABLE;
    let page = Page::containing_address(virt_addr);

    unsafe {
        mapper.map_to(page, frame, flags, &mut frame_allocator).unwrap().flush();
    }

    println!("mapped");

    mem::heap_init(&mut mapper, &mut frame_allocator).expect("hello");




    if mapper.translate_addr(virt_addr).is_some() {
        panic!("user_main is not mapped! {:?}", virt_addr);
    }
}

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);
    println!("hello");

    gdt::enter_user_mode();

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
