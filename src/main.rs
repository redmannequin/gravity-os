#![no_std]
#![no_main]
use core::panic::PanicInfo;

use gravity_os::{mbox::Mailbox, uart1, wait_cycles};

#[link_section = ".text._kernel"]
#[no_mangle]
pub extern "C" fn _kernel() -> ! {
    uart1::init(48000000, 115200);
    uart1::send_str("hello world\n");
    wait_cycles(150);
    Mailbox::init_frame_buffer();
    loop {
        wait_cycles(100_000_000);
        uart1::send_str("test\n");
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        wait_cycles(10_000);
    }
}
