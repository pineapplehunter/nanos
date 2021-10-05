#![no_std]
#![no_main]
#![feature(global_asm)]

mod asm;

const UART_ADDRESS: usize = 0x10000000;
const HELLO: &[u8] = b"Hello World";

#[no_mangle]
unsafe extern "C" fn main() {
    for c in HELLO {
        (UART_ADDRESS as *mut u8).write_volatile(*c);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
