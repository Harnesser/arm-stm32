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
//
// This runs at the maximum pre-emption threshold, essentially in 
// a `interrupt::free` context.
//
// Threshold ndicates the priority that a task must have to preempt the current
// context. A `threshold` of `T0` means that only tasks with a priority of `P1`
// or *higher* can preempt the current context.
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
        //rtfm::wfi(); // this freezes JTAG, so don't do it
    }
}


// TASKS
//
// In the RTFM framework tasks are implemented on top of interrupt handlers;
// in fact each task *is* an interrupt handler.
// This thing below says to call the function `periodic()` when the `Tim7Irq`
// interrupt fires.
//
// `P1` is the lowest priority a task can have, otherwise it won't be able to
// preempt `idle()`. 
tasks!(stm32f100, {
    periodic: Task {
        interrupt: Tim7Irq,
        priority: P1,
        enabled: true,
    },
});

// Interrupt handler, essentially.
// These must run to completion - no loops or lower priority task won't get
// a chance to run.
fn periodic(mut task: Tim7Irq, ref priority: P1, ref threshold: T1) {

    // Task local data
    // We can hold state for this function call using safe local data
    // abstraction `Local`.
    // The `task` token pins this data to this function. No other functions
    // can use it (cos they can't provide a matching token), and data races
    // are avoided.
    static STATE: Local<bool, Tim7Irq> = Local::new(false);

    // can access TIM7 without extra synchoronisation as the ceiling value
    // assigned to `TIM7` (`C1`) matches the task priority (`P1`) and 
    // preemption threshold.
    let tim7 = TIM7.access(priority, threshold);
    let timer = Timer(&tim7);

    // clear the interrupt flag
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

