use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use spin::Mutex;
use crate::log;
use core::fmt::Write;
use cpuio::Port;
use pic8259_simple::ChainedPics;
pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(32, 40) });


const PIC1_CMD: Port<u8> = unsafe { Port::new(0x20) };
const PIC1_DATA: Port<u8> = unsafe { Port::new(0x21) };
const PIC2_CMD: Port<u8> = unsafe { Port::new(0xA0) };
const PIC2_DATA: Port<u8> = unsafe { Port::new(0xA1) };


lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt[32].set_handler_fn(timer_interrupt_handler);
        idt
    };
}

pub fn load_idt() {
    IDT.load();
    init_pic();
    x86_64::instructions::interrupts::enable();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    log!("EXCEPTION: BREAKPOINT\n{:#?}\n", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    log!(".");

    PIC1_CMD.write(0x20);
}


fn init_pic() {

    unsafe { PICS.lock().initialize() };

    let new_mask = 0b11111110;
    PIC1_DATA.write(new_mask);

    let mask = PIC1_DATA.read();
    log!("PIC1 mask: {}\n", mask);

    /*let mut wait_port: Port<u8> = unsafe { Port::new(0x80) };
    let mut wait = || { wait_port.write(0) };

    PIC1_CMD.write(0x11); // 0x10?
    wait();

    PIC1_DATA.write(32);
    wait();

    PIC1_DATA.write(4);
    wait();

    PIC1_DATA.write(0x01);
    wait();*/
}
