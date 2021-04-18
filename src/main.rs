#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, World{}", "!!!!!");

    // Call the idt init routine in lib.rs
    mini_os::init();

    // Invokes a breakpoint exception
    x86_64::instructions::interrupts::int3();

    println!("It did not crash!");
    loop {}
}
