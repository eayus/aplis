#![feature(abi_x86_interrupt)]
#![feature(abi_efiapi)]

#![no_std]
#![no_main]

mod logger;
mod drivers;
mod interrupts;
mod keyboard;

use uefi::prelude::*;
use uefi::table::boot::{MemoryDescriptor, AllocateType, MemoryType};
use core::panic::PanicInfo;
use core::mem::size_of;
use core::fmt::Write;
use drivers::graphics::GraphicsDriver;

fn num_pages(num_bytes: usize) -> usize {
    // Essentially perform a ceiling division of num_bytes / page_size
    let page_size = 4 * 1024;
    let min_pages = num_bytes / page_size;

    if num_bytes % page_size == 0 {
        min_pages
    } else {
        min_pages + 1
    }
}


#[entry]
fn efi_main(handle: Handle, system_table: SystemTable<Boot>) -> Status {
    // NOTE: If we do not exit boot services within 5 minutes, then the
    // system will automatically reset. To prevent this, disable the
    // watchdog timer (https://docs.rs/uefi/0.8.0/uefi/table/boot/struct.BootServices.html#method.set_watchdog_timer)

    // NOTE: philopp tutorial has a much more advanced target configuration, which disables loads
    // of things like "redzone" or something. It's possible bugs could be caused by this, so it may
    // be worth moving to that config at some point.

    log!("Loading Aplis...\n");


    let mut graphics = GraphicsDriver::<1600, 900>::new(system_table.boot_services());


    // Memory map and exit boot services...

    let estimated_memory_map_size = system_table.boot_services().memory_map_size()
                                  + 4 * size_of::<MemoryDescriptor>();

    log!("Estimated memory map size: {} bytes\n", estimated_memory_map_size);

    let memory_map_addr = system_table
        .boot_services()
        .allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, num_pages(estimated_memory_map_size))
        .unwrap()
        .unwrap();

    log!("Memory map address: {}\n", memory_map_addr);

    let memory_map_buffer = unsafe { core::slice::from_raw_parts_mut(memory_map_addr as *mut u8, estimated_memory_map_size) };

    log!("About to exit boot services...\n");


    let (_system_table, _memory_descriptors) = system_table.exit_boot_services(handle, memory_map_buffer).unwrap().unwrap();

    log!("Succesfully exited boot services.\n");

    log!("About to load IDT.\n");
    interrupts::load_idt();

    log!("IDT Loaded, running breakpoint.\n");

    x86_64::instructions::interrupts::int3();


    log!("Breakpoint done, running graphics test.\n");

    // Graphics test...
    graphics.go();

    log!("Key 'a' state: {}\n", keyboard::KEYBOARD.letter_a.is_pressed());


    loop {}
}

#[panic_handler]
fn panic(panic_info: &PanicInfo<'_>) -> ! {
    log!("{}", panic_info);
    loop {}
}
