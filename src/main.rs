#![no_std]
#![no_main]
#![feature(global_asm)]

mod asm;
mod uart;
use core::fmt::Write;

use crate::uart::UART0;

#[no_mangle]
extern "C" fn main() {
    write!(UART0, "Hello World! My name is {}!", "Shogo").unwrap();
    todo!("cause a panic!");
}

extern "C" {
    fn abort() -> !;
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    write!(UART0, "a panic occured: {}", info).unwrap();
    unsafe { abort() }
}
