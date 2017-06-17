//! examples/rotary-encoder.rs
//!
//! Driver to decode an incremental rotary encoder
//!
//! ## Pin connections
//!
//!     +-------+
//!     |       |
//!     o       o ----> PA0, input with pullup.
//!     |       o ----> PA1, output, drive low
//!     o       o ----> PA2, input with pullup.
//!     |       |
//!     +-------+
//!
//!
//! ## Waveforms for Clockwise (CW) rotation
//!          _____       ________
//!     PA0       |_____|
//!          ________       _____
//!     PA2          |_____|
//!
//! ## Waveforms for Counter-Clockwise (CCW) rotation
//!          ________       _____
//!     PA0          |_____|
//!          _____       ________
//!     PA2       |_____|
//!
//!
//! ## Decoding Method
//! 1. Set up a timer to poll PortA
//! 2. In the ISR:
//!   2.1 Read PortA
//!   2.2 Update state of thing
//!


#![feature(used)]
#![no_std]

extern crate cast;
#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;

extern crate stm32f100;

use core::u16;
use cast::{u16,u32};

use cortex_m::asm;
use stm32f100::{GPIOA, RCC, TIM7, NVIC};
use stm32f100::interrupt;
use stm32f100::interrupt::{Interrupt};

mod frequency {
    /// Frequency of APB1 bus (TIM7 is connected to this)
    pub const APB1: u32 = 8_000_000; // Hz
}

/// Timer Frequency
const FREQUENCY: u32 = 100; // Hz

//static mut counter: i32 = 0;
//static mut call_counter: i32 = 0;

#[inline(never)]
fn main() {

    // critical section - this closure is non-preemptable 
    // which means, I think, that it can't be interrupted.
    cortex_m::interrupt::free(
        |cs| {

            // initialisation
            let rcc = RCC.borrow(cs); // R_eset and C_lock C_ontrol
            let gpioa = GPIOA.borrow(cs);
            let tim7 = TIM7.borrow(cs);
            let nvic = NVIC.borrow(cs);

            // power up the relevant peripherals
            rcc.apb2enr.modify(|_,w| w.iopaen().enabled());
            rcc.apb1enr.modify(|_,w| w.tim7en().enabled());


            // PA0 as input with a pull-up
            gpioa.crl.modify(|_,w| w.mode0().input());
            //gpioa.crl.modify(|_,w| w.cnf0().digital_input_pull());
            gpioa.crl.modify(|_,w| w.cnf0().alt_push_pull());
            gpioa.bsrr.write(|w| w.bs0().set()); // enables pullup


            // PA2 as input with pull-up
            gpioa.crl.modify(|_,w| w.mode2().input());
            //gpioa.crl.modify(|_,w| w.cnf0().digital_input_pull());
            gpioa.crl.modify(|_,w| w.cnf2().alt_push_pull());
            gpioa.bsrr.write(|w| w.bs2().set()); // enables pullup


            // configure TIM7 for periodic timeouts
            let ratio = frequency::APB1 / FREQUENCY;
            let psc = u16((ratio-1) / u32(u16::MAX)).unwrap();
            let arr = u16(ratio / u32(psc + 1)).unwrap();
            unsafe {
                // japaric didn't need unsafe here...
                tim7.psc.write(|w| w.psc().bits(psc));
                tim7.arr.write(|w| w.arr().bits(arr));
            }
            tim7.cr1.write(|w| w.opm().continuous());


            // Set up NVIC
            nvic.clear_pending(Interrupt::Tim7Irq);
            unsafe {
                nvic.set_priority(Interrupt::Tim7Irq, 55);
            }
            nvic.enable(Interrupt::Tim7Irq);
            
            // start the timer
            tim7.cr1.modify(|_,w| w.cen().enabled());

            hprintln!("Setup complete");
            }

    );

    unsafe {
        cortex_m::interrupt::enable();
    }

}

extern "C" fn rotary_encoder_handler(_ctxt: interrupt::Tim7Irq) {

    // have to clear the pending bit in the peripheral
    // don't have to clear pending bit in NVIC
    cortex_m::interrupt::free(
        |cs| {
            let tim7 = TIM7.borrow(cs);
            tim7.sr.modify(|_,w| w.uif().clear());
        }
    );

}


#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
static INTERRUPTS: interrupt::Handlers = interrupt::Handlers {
    Tim7Irq: rotary_encoder_handler,
    ..interrupt::DEFAULT_HANDLERS
};

