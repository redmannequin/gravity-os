#![no_std]
#![no_main]
use core::panic::PanicInfo;

use gravity_os::{
    fb::{init_fb, run},
    println,
    uart1::UART1,
    wait_cycles,
};

#[link_section = ".text._kernel"]
#[no_mangle]
pub extern "C" fn _kernel() -> ! {
    UART1::init(48000000, 115200);
    println!("hello world");

    wait_cycles(100000);

    let mut fb = init_fb(800, 600);
    wait_cycles(10000000);
    run(&mut fb);

    loop {
        wait_cycles(100_000_000);
        println!("test");
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        wait_cycles(10_000);
    }
}
