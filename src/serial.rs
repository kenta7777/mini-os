use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    lazy_static! {
        pub static ref SERIAL1: Mutex<SerialPort> = {
            let mut serial_port = unsafe { SerialPort::new(0x3F8) };
            serial_port.init();
            Mutex::new(serial_port)
        };
    }

    interrupts::without_interrupts(|| {
        SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
    });
}