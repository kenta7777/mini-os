#![no_std]
#![feature(abi_x86_interrupt)]

pub mod interrupts;
pub mod vga_buffer;

// With this function, the idt init routine can be shared between main.rs and lib.rs
pub fn init() {
    interrupts::init_idt();
}
