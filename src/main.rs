#![no_std]
#![no_main]

mod serial;
mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    mini_os::hlt_loop();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, World{}", "!!!!!");

    // Call the idt init routine in lib.rs
    mini_os::init();

    // fn stack_overflow() {
    //     stack_overflow();
    // }

    // stack_overflow();

    println!("It did not crash!");
    // loop {
    //     use mini_os::print;
    //     print!("-");
    // }
    mini_os::hlt_loop();
}
