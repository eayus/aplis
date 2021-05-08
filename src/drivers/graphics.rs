use core::cell::UnsafeCell;
use uefi::prelude::*;
use uefi::proto::console::gop::{GraphicsOutput, PixelFormat};


pub struct GraphicsDriver<const WIDTH: isize, const HEIGHT: isize> {
    buffer: *mut u8,
}


impl<const WIDTH: isize, const HEIGHT: isize> GraphicsDriver<WIDTH, HEIGHT> {

    pub fn new(boot_services: &BootServices) -> Self {

        let gop_cell: &UnsafeCell<GraphicsOutput> = boot_services
            .locate_protocol()
            .expect_success("Failed to get GOP.");

        let gop: &mut GraphicsOutput = unsafe { &mut *gop_cell.get() };

        let desired_mode = gop
            .modes()
            .map(|mode| mode.unwrap())
            .find(|mode| mode.info().resolution() == (WIDTH as usize, HEIGHT as usize)
                      && mode.info().pixel_format() == PixelFormat::Bgr
                      && mode.info().stride() == WIDTH as usize)
            .expect("Failed to find a graphics mode with the desired properties.");

        gop.set_mode(&desired_mode).expect_success("Failed to set GOP mode.");

        Self {
            buffer: gop.frame_buffer().as_mut_ptr(),
        }

    }

    pub fn go(&mut self) {
        unsafe {
            for i in 0..500 {
                *self.buffer.offset(i * 4 + 0) = 100;
                *self.buffer.offset(i * 4 + 1) = 10;
                *self.buffer.offset(i * 4 + 2) = 255;
            }
        }
    }

}
