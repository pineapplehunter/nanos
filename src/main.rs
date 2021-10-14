#![no_std]
#![no_main]

use riscv::register::mhartid;

mod uart;

use crate::uart::UART0;

extern "C" {
    static __stack_start: core::cell::UnsafeCell<usize>;
    static __heap_start: core::cell::UnsafeCell<usize>;
}

#[no_mangle]
extern "C" fn main() {
    println!("hartid = {}", mhartid::read());
    println!("stack_start = {}", unsafe { __stack_start.get() as usize });
    println!("heap_start = {}", unsafe { __heap_start.get() as usize });
    let a = fib(50);
    println!("fib(10) = {}", a);
}

fn fib(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 1..n {
        let tmp = a + b;
        a = b;
        b = tmp;
    }
    return b;
}

extern "C" {
    fn abort() -> !;
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use core::fmt::Write;
    write!(UART0.lock(), "\nPANIC: {}\n", info).unwrap();
    unsafe { abort() }
}
