// examples/blinky.rs

#![feature(used)]
#![no_std]

extern crate cast;
extern crate cortex_m;
extern crate cortex_m_rt;

extern crate stm32f100;

use core::u16;
use cast::{u16,u32};

use cortex_m::asm;
use stm32f100::{GPIOC, RCC, TIM7};

// LED blue: PC8
// LED green: PC9

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
            let gpioc = GPIOC.borrow(cs);
            let rcc = RCC.borrow(cs); // R_eset and C_lock C_ontrol
            let tim7 = TIM7.borrow(cs);

            // power up the relevant peripherals
            rcc.apb2enr.modify(|_,w| w.iopcen().enabled());
            rcc.apb1enr.modify(|_,w| w.tim7en().enabled());

            // configure PC9 as an output
            // 0=A 1=B 2=C
            gpioc.crh.modify(|_,w| w.mode9().output_10mhz());

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

            // start the timer
            tim7.cr1.modify(|_,w| w.cen().enabled());

            let mut state = false;
            loop {
                // wait for an update event
                while tim7.sr.read().uif().is_no_update() {}

                // clear the update flag
                tim7.sr.modify(|_,w| w.uif().clear());

                // toggle the state
                state = !state;

                // blink the LED
                if state {
                    gpioc.bsrr.write(|w| w.bs7().set());
                } else {
                    gpioc.bsrr.write(|w| w.br7().reset());
                }
            }

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

