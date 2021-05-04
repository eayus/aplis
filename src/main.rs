#![no_std]
#![no_main]


#![feature(abi_efiapi)]
use uefi::prelude::*;

use core::panic::PanicInfo;

#[entry]
fn efi_main(handle: Handle, system_table: SystemTable<Boot>) -> Status {
    Status::SUCCESS
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
