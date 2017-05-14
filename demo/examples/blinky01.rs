// examples/blinky.rs

#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;

extern crate stm32f100;

use cortex_m::asm;
use stm32f100::{GPIOE, RCC, TIM7};


#[inline(never)]
fn main() {

    // critical section - this closure is non-preemptable 
    // which means, I think, that it can't be interrupted.
    cortex_m::interrupt::free(
        |cs| {


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

