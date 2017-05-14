// examples/blinky.rs

#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;

extern crate stm32f100;

use cortex_m::asm;
use stm32f100::{GPIOC, RCC, TIM7};

// LED blue: PC8
// LED green: PC9

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
            gpioc.moder.modify(|_,w| w.moder9().output());


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

