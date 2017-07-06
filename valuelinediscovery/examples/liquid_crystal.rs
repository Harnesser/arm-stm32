//! Writes to a Liquid Crystal Display

#![feature(const_fn)]
#![feature(used)]
#![no_std]

extern crate cortex_m_rt;
#[macro_use]
extern crate cortex_m_rtfm as rtfm;
extern crate valuelinediscovery as dsc;

use dsc::stm32f100;
use dsc::lcd::Lcd;

use rtfm::{P0, T0, TMax};


// RESOURCES
// have to register all periphs that we're using
peripherals!(stm32f100, {
    GPIOC:  Peripheral {
        register_block: Gpioc,
        ceiling: C0, // kinda like a priority
    },
    RCC: Peripheral {
        register_block: Rcc,
        ceiling: C0,
    },
});


// Initialisation
fn init(ref priority: P0, threshold: &TMax) {
    let gpioc = GPIOC.access(priority, threshold);
    let rcc = RCC.access(priority, threshold);

    let lcd = Lcd(&gpioc);

    // configure the PCx pins as outputs
    lcd.init(&rcc);

    lcd.clear();
    lcd.set_position(0,0);
    lcd.write(b"Marty");
    lcd.set_position(1,0);
    lcd.write(b"is");
    lcd.set_position(2,0);
    lcd.write(b"kinda");
    lcd.set_position(3,4);
    lcd.write(b"CLASS!!!!!");

}


fn idle(_priority: P0, _threshold: T0) -> ! {
    loop {
        //rtfm::wfi(); // this freezes JTAG, so don't do it
    }
}


// TASKS
tasks!(stm32f100, {}
);

