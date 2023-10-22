#![no_std]
#![no_main]
use core::panic::PanicInfo;

use gravity_os::{
    fb::{self, init_fb},
    println,
    uart1::UART1,
    wait_cycles, Instan,
};

#[link_section = ".text._kernel"]
#[no_mangle]
pub extern "C" fn _kernel() -> ! {
    UART1::init(500000000, 115200);
    wait_cycles(1000);
    let mut fb = init_fb(1920, 1080);

    let mut x_offset = 0;
    let mut y_offset = 0;
    loop {
        let dt = Instan::now();
        fb::render_gradient(&mut fb, x_offset, y_offset);
        x_offset = (x_offset + 1) % 1920;
        y_offset = (y_offset + 1) % 1080;
        println!("ms: {}", dt.ms_elapsed());
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel panic!\n\n{}\n", info);
    loop {
        wait_cycles(10_000);
    }
}
