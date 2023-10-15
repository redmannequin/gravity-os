#![no_std]
#![no_main]
use core::panic::PanicInfo;

use gravity_os::{uart1, wait_cycles};

#[link_section = ".text._start"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    uart1::init(5_0000_0000, 115200);
    uart1::send_str("Hello world\n");
    loop {
        wait_cycles(100_000);
        uart1::send_str("test n\n");
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        wait_cycles(10_000);
    }
}
