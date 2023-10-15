#![no_std]
use core::{
    arch::asm,
    ptr::{read_volatile, write_volatile},
};

pub mod gpio;
pub mod uart1;

pub const MMIO_BASE_RPI_3: u32 = 0x3F00_0000;
pub const MMIO_BASE_RPI_4: u32 = 0xFE00_0000;
pub const MMIO_BASE: u32 = MMIO_BASE_RPI_4;

pub fn mmio_write(register: u32, value: u32) {
    unsafe { write_volatile(register as *mut u32, value) }
}

pub fn mmio_read(register: u32) -> u32 {
    unsafe { read_volatile(register as *mut u32) }
}

pub fn wait_cycles(n: u32) {
    for _ in 0..n {
        unsafe {
            asm!("NOP");
        }
    }
}
