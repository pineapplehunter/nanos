#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]
#![feature(alloc_error_handler)]

extern crate alloc;

use riscv::register::mhartid;

mod nanos_alloc;
mod uart;

use crate::uart::UART0;

extern "C" {
    static __stack_start: core::cell::UnsafeCell<usize>;
    static __heap_start: core::cell::UnsafeCell<usize>;
}

#[no_mangle]
pub extern "C" fn entry() {
    main();
    println!("\nEnd of Program!");
}
fn main() {
    println!("hartid = {}", mhartid::read());
    println!("stack_start = {}", unsafe { __stack_start.get() as usize });
    println!("heap_start = {}", unsafe { __heap_start.get() as usize });
    let a = fib(10);
    println!("fib(10) = {}", a);

    let mut v = alloc::vec::Vec::<i32>::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);

    let mut b = alloc::vec::Vec::new();
    for i in 5..100 {
        b.push(i);
    }
    println!("{:?}", b);
    for i in 5..100 {
        v.push(i);
    }
    println!("{:?}", v);
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
