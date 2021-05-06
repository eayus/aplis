use core::fmt::{Write, Result};
use uart_16550::SerialPort;
use lazy_static::lazy_static;
use spin::Mutex;

// uefi::proto::console::serial::Serial
// is an alternative to uart_16550 library that looks a bit more advanced


const SERIAL_IO_PORT: u16 = 0x3F8;


lazy_static!{
    pub static ref LOGGER: Mutex<SerialLogger> = Mutex::new(SerialLogger::new());
}


pub struct SerialLogger {
    port: SerialPort,
}


impl SerialLogger {
    pub fn new() -> Self {
        let mut port = unsafe { SerialPort::new(SERIAL_IO_PORT) };
        port.init();

        SerialLogger { port }
    }
}


impl Write for SerialLogger {
    fn write_str(&mut self, s: &str) -> Result {
        for b in s.bytes() {
            self.port.send(b);
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => (write!($crate::serial_debug::LOGGER.lock(), $($arg)*).unwrap());
}
