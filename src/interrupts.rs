#![no_std]

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        // Set break point handler function in interrupt descriptor table
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

// Breakpoint exception handler function
extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame
)
{
    // Pretty-prints interrupt stack frame
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
