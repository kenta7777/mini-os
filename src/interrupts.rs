#![no_std]

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
//use crate::println;

pub fn init_idt() {
    // Create interrupt descriptor table
    let mut idt = InterruptDescriptorTable::new();
}

// Breakpoint exception handler function
extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame)
{
    //TODO: Implement vga buffer
    //println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
