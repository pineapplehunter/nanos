#![no_std]
#![no_main]

const UART_ADDRESS: usize = 0x10000000;

#[no_mangle]
unsafe extern "C" fn main() {
    let s = b"Hello World";
    for c in s {
        (UART_ADDRESS as *mut u8).write_volatile(*c);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
