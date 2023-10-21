#![no_std]
#![no_main]
use core::panic::PanicInfo;

use gravity_os::{
    fb::{self, init_fb},
    println,
    uart1::UART1,
    wait_cycles,
};

#[link_section = ".text._kernel"]
#[no_mangle]
pub extern "C" fn _kernel() -> ! {
    UART1::init(500000000, 115200);
    wait_cycles(1000);
    let mut fb = init_fb(1920, 1080);
    fb::run(&mut fb);
    loop {
        wait_cycles(1_000_000);
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel panic!\n\n{}\n", info);
    loop {
        wait_cycles(10_000);
    }
}
