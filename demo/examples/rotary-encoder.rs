//! examples/rotary-encoder.rs
//!
//! Driver to decode an incremental rotary encoder
//!
//! ## Pin connections
//!
//!     +-------+
//!     |       |
//!     o       o ----> PA0, input with pullup, interrupt on rising edge.
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
//! Using `PA0` as the clock, we'll put a rising-edge interrupt on it. We'll
//! keep a count to store the value (signed?). When the interrupt triggers
//! the count will be:
//! * incremented if `PA2` is `LOW`
//! * decremented if `PA2` is `HIGH`
//!
//! ## Interrupts
//! Do we have interrupts on PA? Yup, I think so. They have Schmitt triggers
//! too.
//!
//! ## Debouncing
//! How am I going to debounce? Delay re-enabling of handler?

#![feature(used)]
#![no_std]

extern crate cast;
extern crate cortex_m;
extern crate cortex_m_rt;

extern crate stm32f100;

use core::u16;
use cast::{u16,u32};

use cortex_m::asm;
use stm32f100::{GPIOA, RCC, TIM7};

mod frequency {
    /// Frequency of APB1 bus (TIM7 is connected to this)
    pub const APB1: u32 = 8_000_000; // Hz
}

/// Timer Frequency
const FREQUENCY: u32 = 1; // Hz

#[inline(never)]
fn main() {

    // critical section - this closure is non-preemptable 
    // which means, I think, that it can't be interrupted.
    cortex_m::interrupt::free(
        |cs| {

            // initialisation
            let gpioa = GPIOA.borrow(cs);



            }
    );
}


#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}

