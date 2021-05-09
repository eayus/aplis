use core::fmt::{Write, Result};
use cpuio::Port;
use spin::Mutex;

/*
 * This module safely encapsulates logging to the serial port. Unlike other drivers, this actually
 * uses a global static. We permit this because the logger is a debugging tool, and is not actually
 * integral to the functioning of the application.
 *
 * Safety:
 *   - The logger uses a spin lock and so it is easy to create deadlocks if it is used in an
 *     interrupt/exception handler.
 */

const SERIAL_IO_PORT: u16 = 0x3F8;

pub static LOGGER: Mutex<Logger> = Mutex::new(Logger::new());


pub struct Logger {
    port: Port<u8>,
}


impl Logger {
    const fn new() -> Self {
        let port = unsafe { Port::new(SERIAL_IO_PORT) };
        Logger { port }
    }
}


impl Write for Logger {
    // TODO: Not a fan of this returning a result, since we dont have a failure case anyway.
    fn write_str(&mut self, s: &str) -> Result {
        for b in s.bytes() {
            self.port.write(b);
        }

        Ok(())
    }
}


#[macro_export(local_inner_macros)]
macro_rules! log {
    ($($arg:tt)*) => {
        x86_64::instructions::interrupts::without_interrupts(|| {
            core::write!($crate::logger::LOGGER.lock(), $($arg)*).unwrap();
        })
    };
}
