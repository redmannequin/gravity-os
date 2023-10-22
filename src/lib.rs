#![no_std]
use core::{arch::asm, ptr};

pub mod boot;
pub mod fb;
pub mod gpio;
pub mod mailbox;
pub mod print;
pub mod uart1;

pub const MMIO_BASE_RPI_3: u32 = 0x3F00_0000;
pub const MMIO_BASE_RPI_4: u32 = 0xFE00_0000;
pub const MMIO_BASE: u32 = MMIO_BASE_RPI_4;

pub fn mmio_write(register: u32, value: u32) {
    unsafe { ptr::write_volatile(register as *mut u32, value) }
}

pub fn mmio_read(register: u32) -> u32 {
    unsafe { ptr::read_volatile(register as *const u32) }
}

pub fn wait_cycles(n: u32) {
    for _ in 0..n {
        unsafe {
            asm!("NOP");
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Instan {
    pub tick: u32,
}

impl Instan {
    pub fn now() -> Self {
        let tick: u32;
        unsafe { asm!("mrs {0:x}, cntpct_el0", out(reg) tick) }
        Self { tick }
    }

    pub fn ticks_elapsed(self) -> u32 {
        let curr_tick = Instan::now();
        curr_tick.tick - self.tick
    }

    pub fn ms_elapsed(self) -> u32 {
        let ticks_elapsed = self.ticks_elapsed();
        let freq: u32;
        unsafe { asm!("mrs {0:x}, cntfrq_el0", out(reg) freq) };
        (ticks_elapsed * 1000 * 1000) / freq
    }
}
