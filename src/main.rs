#![no_main]
#![no_std]

extern crate panic_halt;
extern crate riscv_rt;

mod uart;

use riscv_rt::entry;

#[entry]
fn main() -> ! {

    // This is a basic SiFive Uart interface
    let mut uart = uart::Uart::new(0x54_000_000);
    uart.init();
    println!("Hallo Slashwhy");

    // we have no support for the debug interface and can not signalize the programm is finished
    loop {}
}


// ///////////////////////////////////
// / RUST MACROS
// ///////////////////////////////////
#[macro_export]
macro_rules! print
{
    ($($args:tt)+) => ({
            use core::fmt::Write;
            let _ = write!(crate::uart::Uart::new(0x54_000_000), $($args)+);
    });
}

#[macro_export]
macro_rules! println
{
    () => ({
        print!("\r\n")
    });
    ($fmt:expr) => ({
        print!(concat!($fmt, "\r\n"))
    });
    ($fmt:expr, $($args:tt)+) => ({
        print!(concat!($fmt, "\r\n"), $($args)+)
    });
}