//! ----------------
//! GPIO Interface
//! ----------------
//! https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf
use crate::{mmio_read, mmio_write, MMIO_BASE};

pub use self::{gpio_fsel_registers::PinFunction, gpio_pud_ctr_registers::PullState};

pub const GPIO_BASE: u32 = MMIO_BASE + 0x200000;
pub const GPPUD: u32 = GPIO_BASE + 0x94;
pub const GPPUDCLK0: u32 = GPIO_BASE + 0x98;

// -----------------------------
// TODO: add missing registers
// -----------------------------
// const GP_SET0: u32 = GPIO_BASE + 0x1c; // GPIO Pin Output Set 0
// const GP_SET1: u32 = GPIO_BASE + 0x20; // GPIO Pin Output Set 1
// const GP_CLR0: u32 = GPIO_BASE + 0x28; // GPIO Pin Output Clear 0
// const GP_CLR1: u32 = GPIO_BASE + 0x2c; // GPIO Pin Output Clear 1
// const GP_LEV0: u32 = GPIO_BASE + 0x34; // GPIO Pin Level 0
// const GP_LEV1: u32 = GPIO_BASE + 0x38; // GPIO Pin Level 1
// const GP_EDS0: u32 = GPIO_BASE + 0x40; // GPIO Pin Event Detect Status 0
// const GP_EDS1: u32 = GPIO_BASE + 0x44; // GPIO Pin Event Detect Status 1
// const GP_REN0: u32 = GPIO_BASE + 0x4c; // GPIO Pin Rising Edge Detect Enable 0
// const GP_REN1: u32 = GPIO_BASE + 0x50; // GPIO Pin Rising Edge Detect Enable 1
// const GP_FEN0: u32 = GPIO_BASE + 0x58; // GPIO Pin Falling Edge Detect Enable 0
// const GP_FEN1: u32 = GPIO_BASE + 0x5c; // GPIO Pin Falling Edge Detect Enable 1
// const GP_HEN0: u32 = GPIO_BASE + 0x64; // GPIO Pin High Detect Enable 0
// const GP_HEN1: u32 = GPIO_BASE + 0x68; // GPIO Pin High Detect Enable 1
// const GP_LEN0: u32 = GPIO_BASE + 0x70; // GPIO Pin Low Detect Enable 0
// const GP_LEN1: u32 = GPIO_BASE + 0x74; // GPIO Pin Low Detect Enable 1
// const GP_AREN0: u32 = GPIO_BASE + 0x7c; // GPIO Pin Async. Rising Edge Detect 0
// const GP_AREN1: u32 = GPIO_BASE + 0x80; // GPIO Pin Async. Rising Edge Detect 1
// const GP_AFEN0: u32 = GPIO_BASE + 0x88; // GPIO Pin Async. Falling Edge Detect 0
// const GP_AFEN1: u32 = GPIO_BASE + 0x8c; // GPIO Pin Async. Falling Edge Detect 1

mod gpio_pud_ctr_registers {
    //! ----------------------------------------
    //! GPIO PullUp/PullDown Control Registers
    //! ----------------------------------------
    //! The GPIO Pull-up / Pull-down Registers control the actuation of the
    //! internal pull-up/down resistors. Reading these registers gives the
    //! current pull-state.
    //!
    //! +---------------------------------------------------------------------------------+
    //! |                               PUD Control Layout                                |
    //! +---------------------------------------------------------------------------------+
    //! | [00] [00] [00] [00] [00] [00] [00] [00] [00] [00] [00] [00] [00] [00] [00] [00] |
    //! | _p0  _p1  _p2  _p3  _p4  _p5  _p6  _p7  _p8  _p9  _p10 _p11 _p12 _p13 _p14 _p15 |
    //! +---------------------------------------------------------------------------------+
    //!
    //! ----------------
    //! Pull State Map
    //! ----------------
    //! With 2-bits of informations the maximum supported states is 4.
    //!
    //! +-------+-----+
    //! | State | Val |
    //! +-------+-----+
    //! | None  |  00 |
    //! | Up    |  01 |
    //! | Down  |  10 |
    //! | RSVD  |  11 |
    //! +-------+-----+
    use super::GPIO_BASE;

    /// PUD Control for pins 0-15
    pub const PUD_CTR0: u32 = GPIO_BASE + 0xe4;
    /// PUD Control for pins 16-31
    pub const PUD_CTR1: u32 = GPIO_BASE + 0xe8;
    /// PUD Control for pins 32-39
    pub const PUD_CTR2: u32 = GPIO_BASE + 0xec;

    /// the set of pull states
    pub enum PullState {
        None,
        Up,
        Down,
        Rsvd,
    }

    impl PullState {
        /// returns the u32 value of pull state
        pub const fn vaule(self) -> u32 {
            match self {
                PullState::None => 0b00,
                PullState::Up => 0b01,
                PullState::Down => 0b10,
                PullState::Rsvd => 0b11,
            }
        }
    }
}

mod gpio_fsel_registers {
    //! ----------------------------------
    //! GPIO Function Selector Registers
    //! ----------------------------------
    //! The gpio function selector registers set the function of the GPIO pins.
    //! Each function selector(FSEL) is 32-bits long, allocating 3-bits per pin
    //! with two reserve bits. A single FSEL supports ten pins, with the 3-bits
    //! per pin used to define the function of each pin.
    //!
    //! +------------------------------------------------------------------+
    //! |                   Function Selector Layout                       |
    //! +------------------------------------------------------------------+
    //! | [000] [000] [000] [000] [000] [000] [000] [000] [000] [000] [00] |
    //! | _pin0 _pin1 _pin2 _pin3 _pin4 _pin5 _pin6 _pin7 _pin8 _pin9  _R  |
    //! +------------------------------------------------------------------+
    //!  
    //! ---------------------
    //! Supported Functions
    //! ---------------------
    //! With 3-bits of informations the maximum supported functions is 8.
    //!
    //! +----------+-------+
    //! | Function | Vaule |
    //! +----------+-------+
    //! | Input    |   000 |
    //! | Ouput    |   001 |
    //! | AltFn 0  |   100 |
    //! | AltFn 1  |   101 |
    //! | AltFn 2  |   110 |
    //! | AltFn 3  |   111 |
    //! | AltFn 4  |   011 |
    //! | AltFn 5  |   010 |
    //! +----------+-------+
    //!
    //! You can learn more about alternate functions and what they mean [here]
    //! in section `5.3. Alternative Function Assignments`.
    //!
    //! [here]: https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf
    use super::GPIO_BASE;

    /// Function Selector for pins 0-9
    pub const FSEL0: u32 = GPIO_BASE;
    /// Function Selector for pins 10-19
    pub const FSEL1: u32 = GPIO_BASE + 0x04;
    /// Function Selector for pins 20-29
    pub const FSEL2: u32 = GPIO_BASE + 0x08;
    /// Function Selector for pins 30-39
    pub const FSEL3: u32 = GPIO_BASE + 0x0c;

    /// the set of pin functions
    #[derive(Debug, Clone, Copy)]
    pub enum PinFunction {
        Input,
        Ouput,
        AltFn0,
        AltFn1,
        AltFn2,
        AltFn3,
        AltFn4,
        AltFn5,
    }

    impl PinFunction {
        /// returns the u32 vaule of the pin function
        pub const fn value(self) -> u32 {
            match self {
                PinFunction::Input => 0b000,
                PinFunction::Ouput => 0b001,
                PinFunction::AltFn0 => 0b100,
                PinFunction::AltFn1 => 0b101,
                PinFunction::AltFn2 => 0b110,
                PinFunction::AltFn3 => 0b111,
                PinFunction::AltFn4 => 0b011,
                PinFunction::AltFn5 => 0b010,
            }
        }
    }
}

/// provides basic functions to set,get,write, and read the GPIO pins.
/// https://pinout.xyz/
#[derive(Debug, Clone, Copy)]
pub enum GPIO {
    Pin0,
    Pin1,
    Pin2,
    Pin3,
    Pin4,
    Pin5,
    Pin6,
    Pin7,
    Pin8,
    Pin9,
    Pin10,
    Pin11,
    Pin12,
    Pin13,
    /// --------------------------
    /// GPIO pin 14 UART Transmit
    /// --------------------------
    /// AltFn0 UART0 TXD
    /// AltFn1 SMI SD6
    /// AltFn2 DSI D10
    /// AltFn3 AVEOUT VID10
    /// AltFn4 AVEIN VID10
    /// AltFn5 UART1 TXD
    ///
    Pin14,
    /// -------------------------
    /// GPIO pin 14 UART Receive
    /// -------------------------
    /// AltFn0 UART0 RXD
    /// AltFn1 SMI SD7
    /// AltFn2 DPI D11
    /// AltFn3 AVEOUT VID11
    /// AltFn4 AVEIN VID11
    /// AltFn5 UART1 RXD
    Pin15,
    Pin16,
    Pin17,
    Pin18,
    Pin19,
    Pin20,
    Pin21,
    Pin22,
    Pin23,
    Pin24,
    Pin25,
    Pin26,
}

impl GPIO {
    /// set the FSEL of the given GPIO Pin
    pub fn set_fsel(self, func: PinFunction) -> Self {
        set_fsel(self, func);
        self
    }

    /// set the Pull Up/Down State of the given GPIO Pin
    pub fn set_pub_ctr(self, state: PullState) -> Self {
        set_pud_ctr(self, state);
        self
    }

    /// get the Pull Up/Down State of the given GPIO Pin
    pub fn get_pud_ctr_state(self) -> PullState {
        get_pud_ctr_state(self)
    }
}

/// set the FSEL of the given GPIO Pin
fn set_fsel(pin: GPIO, func: PinFunction) {
    let value = func.value();
    let fsel = get_fsel_reg(pin);
    let shift = (pin as u32 % 10) * 3;
    let clear_mask: u32 = 0b111 << shift;
    let val = (mmio_read(fsel) & !clear_mask) | (value << shift);
    mmio_write(fsel, val);
}

/// set the Pull Up/Down State of the given GPIO Pin
fn set_pud_ctr(pin: GPIO, state: PullState) {
    let value = state.vaule();
    let pud_ctr = get_pud_ctr_reg(pin);
    let shift = (pin as u32 % 16) * 2;
    let clear_mask: u32 = 0b11 << shift;
    let val = (mmio_read(pud_ctr) & !clear_mask) | (value << shift);
    mmio_write(pud_ctr, val);
}

/// get the Pull Up/Down State of the given GPIO Pin
fn get_pud_ctr_state(pin: GPIO) -> PullState {
    let register = get_pud_ctr_reg(pin);
    let shift = (pin as u32 % 16) * 2;
    let mask: u32 = 0b11 << shift;
    let val = (mmio_read(register) & mask) >> shift;
    match val {
        0b00 => PullState::None,
        0b01 => PullState::Up,
        0b10 => PullState::Down,
        _ => PullState::Rsvd,
    }
}

/// get the Pull Up/Down register for the given GPIO Pin
const fn get_pud_ctr_reg(pin: GPIO) -> u32 {
    const PUD_CTR_LOOKUP: [u32; 3] = [
        gpio_pud_ctr_registers::PUD_CTR0,
        gpio_pud_ctr_registers::PUD_CTR1,
        gpio_pud_ctr_registers::PUD_CTR2,
    ];
    // `N >> 4` is equivalent to dividing by 16
    PUD_CTR_LOOKUP[(pin as usize) >> 4]
}

/// get the Function Select register for the given GPIO Pin
const fn get_fsel_reg(pin: GPIO) -> u32 {
    const FSEL_LOOKUP: [u32; 4] = [
        gpio_fsel_registers::FSEL0,
        gpio_fsel_registers::FSEL1,
        gpio_fsel_registers::FSEL2,
        gpio_fsel_registers::FSEL3,
    ];
    // `(N * 13) >> 7` correctly divided by 10 until 69
    FSEL_LOOKUP[((pin as usize) * 13) >> 7]
}
