//! LED Roulette
//! But since we've 2 LEDs, it's more like ping-pong

#![feature(const_fn)]
#![feature(used)]
#![no_std]

extern crate cortex_m_rt;
#[macro_use]
extern crate cortex_m_rtfm as rtfm;
extern crate valuelinediscovery as dsc;

use dsc::led::{self, LEDS};
use dsc::stm32f100::interrupt::Tim7Irq;
use dsc::stm32f100;
use dsc::timer::Timer;
use rtfm::{Local, P0, P1, T0, T1, TMax};

extern crate cast;
use cast::{u8, usize};

const FREQUENCY: u32 = 2; // Hz

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
    TIM7: Peripheral {
        register_block: Tim7,
        ceiling: C1,
    },
});


// Initialisation
fn init(ref priority: P0, threshold: &TMax) {
    let gpioc = GPIOC.access(priority, threshold);
    let rcc = RCC.access(priority, threshold);
    let tim7 = TIM7.access(priority, threshold);
    let timer = Timer(&tim7);

    // configure the PCx pins as outputs
    led::init(&gpioc, &rcc);

    // configure timer7 for periodic update events
    timer.init(&rcc, FREQUENCY);

    // start the timer
    timer.resume();
}


fn idle(_priority: P0, _threshold: T0) -> ! {
    loop {
        //rtfm::wfi(); // this freezes JTAG, so don't
    }
}


// TASKS
tasks!(stm32f100, {
    roulette: Task {
        interrupt: Tim7Irq,
        priority: P1,
        enabled: true,
    },
});


// Roulette
fn roulette(mut task: Tim7Irq, ref priority: P1, ref threshold: T1) {

    // Task local data
    static STATE: Local<u8, Tim7Irq> = Local::new(0);

    let tim7 = TIM7.access(priority, threshold);
    let timer = Timer(&tim7);

    // clear the interrupt flag
    if timer.clear_update_flag().is_ok() {
        let state = STATE.borrow_mut(&mut task);

        let curr = *state;
        let next = (curr + 1) % u8(LEDS.len()).unwrap();

        LEDS[usize(curr)].off();
        LEDS[usize(next)].on();

        *state = next;
    } else {
        // only reachable thru `rtfm::request(periodic)
        #[cfg(debug_assertion)]
        unreachable!()
    }
}

