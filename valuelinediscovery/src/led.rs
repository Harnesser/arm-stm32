//! User LEDs

use stm32f100::{GPIOC, Gpioc, Rcc};

/// All the user LEDs
pub static LEDS: [Led; 2] = [
    Led { i: 8 },
    Led { i: 9 },
];

/// An LED
pub struct Led {
    i: u8,
}

impl Led {
    /// Turns off the LED
    pub fn off(&self) {
        // NOTE(safe) atomic write
        unsafe { (*GPIOC.get()).bsrr.write(|w| w.bits(1 << (self.i + 16))) }
    }

    /// Turns on the LED
    pub fn on(&self) {
        // NOTE(safe) atomic write
        unsafe { (*GPIOC.get()).bsrr.write(|w| w.bits(1 << self.i)) }
    }
}

/// Initializes all the user LEDs
pub fn init(gpioc: &Gpioc, rcc: &Rcc) {
    // Power up peripherals
    rcc.apb2enr.modify(|_, w| w.iopcen().enabled());

    // Configure pins 8-9 as outputs
    gpioc
        .crh
        .modify(
            |_, w| {
                w.mode8().output_10mhz()
                    .cnf8().push_pull()
                    .mode9().output_10mhz()
                    .cnf9().push_pull()
            },
        );
}
