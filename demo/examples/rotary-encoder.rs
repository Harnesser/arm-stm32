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
#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;

extern crate stm32f100;

use cortex_m::asm;
use stm32f100::{GPIOA, RCC, EXTI, AFIO, NVIC};
use stm32f100::interrupt;
use stm32f100::interrupt::{Interrupt};

static mut counter: i32 = 0;

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
            let nvic = NVIC.borrow(cs);

            // power up the relevant peripherals
            rcc.apb2enr.modify(|_,w| w.iopaen().enabled());
            rcc.apb2enr.modify(|_,w| w.afioen().enabled());


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
            exti.rtsr.modify(|_,w| w.tr0().enabled());

            // unmask the interrupt on PA0
            exti.imr.modify(|_,w| w.mr0().enabled());

            // wire PA0 posedge to interrupt EXTI0
            unsafe {
                afio.exticr1.modify(|_,w| w.exti0().bits(0));
            }


            // And set up NVIC - do I have enable the NVIC periph too? No.
            nvic.clear_pending(Interrupt::Exti0Irq);
            unsafe {
                nvic.set_priority(Interrupt::Exti0Irq, 13);
            }
            nvic.enable(Interrupt::Exti0Irq);


            // test-fire of interrupt
            //hprintln!("Test-firing interrupt");
            //nvic.set_pending(Interrupt::Exti0Irq); // don't have to clear


            hprintln!("Setup complete asdf asdf");
            }

    );

    unsafe {
        cortex_m::interrupt::enable();
    }

    loop {}

}

extern "C" fn rotary_encoder_handler(_ctxt: interrupt::Exti0Irq) {
    //asm::bkpt();

    // have to clear the pending bit in the peripheral
    // don't have to clear pending bit in NVIC
    cortex_m::interrupt::free(
        |cs| {
            let exti = EXTI.borrow(cs);
            let gpioa = GPIOA.borrow(cs);
            // write one to clear
            unsafe {
                exti.pr.modify(|_,w| w.pr0().bits(1)); 
                if gpioa.idr.read().idr2().bits() == 0 {
                    counter += 1;
                } else {
                    counter -= 1;
                }
            }

        }
    );

    unsafe {
        hprintln!("{}", counter);
    }

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
