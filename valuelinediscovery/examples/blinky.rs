//! Blinks an LED

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

const FREQUENCY: u32 = 1; // Hz

// RESOURCES
peripherals!(stm32f100, {
    GPIOC:  Peripheral {
        register_block: Gpioc,
        ceiling: C0,
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
        rtfm::wfi();
    }
}


// TASKS
tasks!(stm32f100, {
    periodic: Task {
        interrupt: Tim7Irq,
        priority: P1,
        enabled: true,
    },
});

fn periodic(mut task: Tim7Irq, ref priority: P1, ref threshold: T1) {
    // task local data

    static STATE: Local<bool, Tim7Irq> = Local::new(false);

    let tim7 = TIM7.access(priority, threshold);
    let timer = Timer(&tim7);

    if timer.clear_update_flag().is_ok() {
        let state = STATE.borrow_mut(&mut task);

        *state = !*state;

        if *state {
            LEDS[0].on();
        } else {
            LEDS[0].off();
        }
    } else {
        // only reachable thru `rtfm::request(periodic)
        #[cfg(debug_assertion)]
        unreachable!()
    }
}

