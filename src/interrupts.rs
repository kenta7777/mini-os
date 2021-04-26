#![no_std]

use crate::gdt;
use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        // Set break point handler function in interrupt descriptor table
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            // Specify interrupt stack index for double fault in interrupt stack table
            idt.double_fault.set_handler_fn(double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

// Breakpoint exception handler function
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    // Pretty-prints interrupt stack frame
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

// Double fault exception handler function
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
