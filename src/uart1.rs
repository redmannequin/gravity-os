use crate::{
    gpio::{PinFunction, PullState, GPIO},
    mmio_read, mmio_write, wait_cycles,
};

pub mod register {
    //! AUX registers
    use crate::MMIO_BASE;

    /// auxiliary base address
    pub const AUX_BASE: u32 = MMIO_BASE + 0x0021_5000;
    /// auxiliary inrerrupt status
    pub const AUX_IQR: u32 = AUX_BASE;
    /// auxiliary enables
    pub const AUX_ENABLES: u32 = AUX_BASE + 0x04;
    /// mini UART io data
    pub const AUX_MU_IO_REG: u32 = AUX_BASE + 0x40;
    /// mini UART interrupt enable
    pub const AUX_MU_IER_REG: u32 = AUX_BASE + 0x44;
    /// mini UART interrupt idenify
    pub const AUX_MU_IIR_REG: u32 = AUX_BASE + 0x48;
    /// mini UART line control
    pub const AUX_MU_LCR_REG: u32 = AUX_BASE + 0x4C;
    /// mini UART modem control
    pub const AUX_MU_MCR_REG: u32 = AUX_BASE + 0x50;
    /// mini UART line status
    pub const AUX_MU_LSR_REG: u32 = AUX_BASE + 0x54;
    /// mini UART modem status
    pub const AUX_MU_MSR_REF: u32 = AUX_BASE + 0x58;
    /// mini UART scratch
    pub const AUX_MU_SCRATCH: u32 = AUX_BASE + 0x5C;
    /// mini UART extra control
    pub const AUX_MU_CNTL_REG: u32 = AUX_BASE + 0x60;
    /// mini UART extra status
    pub const AUX_MU_STAT_REG: u32 = AUX_BASE + 0x64;
    /// mini UART baudrate
    pub const AUX_MU_BAUD_REG: u32 = AUX_BASE + 0x68;
}

use register::*;

pub fn init(clock_rate: u32, baud_rate: u32) {
    mmio_write(AUX_ENABLES, 0x01);
    mmio_write(AUX_MU_IER_REG, 0x00);
    mmio_write(AUX_MU_CNTL_REG, 0x00);
    mmio_write(AUX_MU_LCR_REG, 0x03);
    mmio_write(AUX_MU_MCR_REG, 0x00);
    mmio_write(AUX_MU_IER_REG, 0x00);
    mmio_write(AUX_MU_IIR_REG, 0xC6);
    mmio_write(AUX_MU_BAUD_REG, clock_rate / (baud_rate * 8) - 1);

    GPIO::Pin14
        .set_pub_ctr(PullState::None)
        .set_fsel(PinFunction::AltFn5);
    GPIO::Pin15
        .set_pub_ctr(PullState::None)
        .set_fsel(PinFunction::AltFn5);

    mmio_write(AUX_MU_CNTL_REG, 0x03);
}

pub fn send_char(ch: char) {
    while (mmio_read(AUX_MU_LSR_REG) & 0x20) != 0 {
        wait_cycles(10);
    }
    mmio_write(AUX_MU_IO_REG, ch as _);
}

pub fn send_str(s: &str) {
    for ch in s.chars() {
        if ch == '\n' {
            send_char('\r');
        }
        send_char(ch);
    }
}
