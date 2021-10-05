use core::fmt::Write;

pub struct UART<const ADDRESS: usize>;

pub const UART0: UART<0x10000000> = UART;

impl<const ADDRESS: usize> Write for UART<ADDRESS> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.as_bytes() {
            unsafe {
                (ADDRESS as *mut u8).write_volatile(*c);
            }
        }
        Ok(())
    }
}
