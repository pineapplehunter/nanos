use lazy_static::lazy_static;
use spin::Mutex;

pub struct Uart<const ADDRESS: usize>;

lazy_static! {
    pub static ref UART0: Mutex<Uart<0x10000000>> = Mutex::new(Uart);
}

impl<const ADDRESS: usize> core::fmt::Write for Uart<ADDRESS> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.as_bytes() {
            unsafe {
                (ADDRESS as *mut u8).write_volatile(*c);
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::uart::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    UART0.lock().write_fmt(args).unwrap();
}
