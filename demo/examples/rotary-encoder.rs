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

use cortex_m::asm;
use stm32f100::{GPIOA, RCC, EXTI, AFIO};
use stm32f100::interrupt;


#[inline(never)]
fn main() {

    // critical section - this closure is non-preemptable 
    // which means, I think, that it can't be interrupted.
    cortex_m::interrupt::free(
        |cs| {

            // initialisation
            let rcc = RCC.borrow(cs); // R_eset and C_lock C_ontrol
            let gpioa = GPIOA.borrow(cs);
            let exti = EXTI.borrow(cs);
            let afio = AFIO.borrow(cs);

            // power up the relevant peripherals
            rcc.apb2enr.modify(|_,w| w.iopaen().enabled());


            // configure PA0 as an input with a pull-up
            gpioa.crl.modify(|_,w| w.mode0().input());
            //gpioa.crl.modify(|_,w| w.cnf0().digital_input_pull());
            gpioa.crl.modify(|_,w| w.cnf0().alt_push_pull());
            gpioa.bsrr.write(|w| w.bs0().set()); // enables pullup

            // configure PA1 as an output and drive low
            // how much current can this sink?
            gpioa.crl.modify(|_,w| w.mode1().output_10mhz());
            gpioa.crl.modify(|_,w| w.cnf1().push_pull());
            gpioa.bsrr.write(|w| w.br1().reset());

            // PA2 as input with pull-up
            gpioa.crl.modify(|_,w| w.mode2().input());
            //gpioa.crl.modify(|_,w| w.cnf0().digital_input_pull());
            gpioa.crl.modify(|_,w| w.cnf2().alt_push_pull());
            gpioa.bsrr.write(|w| w.bs2().set()); // enables pullup


            // set up posedge interrupt on PA0
            exti.imr.modify(|_,w| w.mr0().enabled());
            unsafe {
                afio.exticr1.modify(|_,w| w.exti0().bits(0));
            }

            }
    );

    unsafe {
        cortex_m::interrupt::enable();
    }

    let mut count = 0;
    loop {
        count += 1; 
    }

}

extern "C" fn rotary_encoder_handler(_ctxt: interrupt::Exti0Irq) {
    asm::bkpt();
}


#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
static INTERRUPTS: interrupt::Handlers = interrupt::Handlers {
    Exti0Irq: rotary_encoder_handler,
    ..interrupt::DEFAULT_HANDLERS
};

/*
extern "C" fn default_handler() {
    asm::bkpt();
}
*/
