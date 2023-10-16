#![no_std]
#![no_main]
use core::panic::PanicInfo;

use gravity_os::{uart1, wait_cycles};

#[link_section = ".text._kernel"]
#[no_mangle]
pub extern "C" fn _kernel() -> ! {
    uart1::init(50_000_000, 115200);
    uart1::send_str("Hello world\n");
    wait_cycles(150);
    loop {
        wait_cycles(100_000_000);
        uart1::send_str("test n\n");
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        wait_cycles(10_000);
    }
}
