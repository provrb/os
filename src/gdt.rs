use core::arch::asm;

use lazy_static::lazy_static;
use x86_64::structures::gdt::SegmentSelector;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

pub struct Selectors {
    pub code_selector: SegmentSelector,
    pub tss_selector: SegmentSelector,
    pub user_code: SegmentSelector,
    pub user_data: SegmentSelector,
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        // Kernel stack to transition from ring 3 to ring 0
        tss.privilege_stack_table[0] = {
            static mut KERNEL_STACK: [u8; 4096 * 5] = [0; 4096 * 5];
            let start = VirtAddr::from_ptr(&raw const KERNEL_STACK);
            let end = start + (4096 * 5) as u64;
            end
        };

        // Double fault handler stack
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let start = VirtAddr::from_ptr(&raw const STACK);
            let end = start + STACK_SIZE as u64;
            end
        };

        tss
    };
}

lazy_static! {
    pub static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.append(Descriptor::kernel_code_segment());
        let tss_selector = gdt.append(Descriptor::tss_segment(&TSS));
        let user_code = gdt.append(Descriptor::user_code_segment());
        let user_data = gdt.append(Descriptor::user_data_segment());

        (
            gdt,
            Selectors {
                code_selector,
                tss_selector,
                user_code,
                user_data,
            },
        )
    };
}

pub fn init_gdt() {
    use x86_64::instructions::segmentation::{Segment, CS};
    use x86_64::instructions::tables::load_tss;

    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}

use x86_64::registers::rflags::RFlags;

use crate::{gdt, println};

#[no_mangle]
#[link_section = ".user_text"]
pub extern "C" fn user_main() -> ! {
    // User mode code.
    loop {}
}

/// Switch to user mode using iretq
pub fn enter_user_mode() -> ! {
    use crate::mem::*;

    let user_cs = GDT.1.user_code.0; // User mode CS
    let user_ds = GDT.1.user_data.0; // User mode DS/SS

    let user_main_addr = user_main as u64;

    println!("user_main is at: {:#x}", user_main_addr);

    unsafe {
        asm!(
            "mov rsp, {0}",      // Set RSP to user stack top
            "push {1}",          // Push User Data Segment
            "push {0}",          // Push User Stack Pointer
            "pushfq",            // Push RFLAGS
            "push {2}",          // Push User Code Segment
            "push {3}",          // Push User Entry (RIP)
            "iretq",             // Interrupt return (switch to user mode)
            in(reg) USER_STACK_TOP,
            in(reg) user_ds,  // Load User Data Segment dynamically
            in(reg) user_cs,  // Load User Code Segment dynamically
            in(reg) USER_ENTRY,
            options(noreturn)
        );
    }
    
}
