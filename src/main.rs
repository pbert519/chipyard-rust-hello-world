#![no_main]
#![no_std]

extern crate panic_halt;
extern crate riscv_rt;

#[no_mangle]
static mut tohost: u64 = 0;
#[no_mangle]
static mut fromhost: u64 = 0;

mod uart;

use riscv_rt::entry;

#[entry]
fn main() -> ! {

    // This is a basic SiFive Uart interface
    let mut uart = uart::Uart::new(0x54_000_000);
    uart.init();
    println!("Hello World");


    loop {
        exit(1);
    }
}

/// Helper function to signal verilator stopping the simulation
/// Only usable with chipyard created simulations
fn exit(exitcode: u64) {
    unsafe {
        // signals the simulator to stop the application
        tohost = (exitcode << 1) | 1;
        // just a dummy to keep the symbol
        fromhost = 1;
     }
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