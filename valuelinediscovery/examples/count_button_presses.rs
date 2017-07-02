//! Counts button presses

#![feature(const_fn)]
#![feature(used)]
#![no_std]

#[macro_use]
extern crate cortex_m;

extern crate cortex_m_rt;
#[macro_use]
extern crate cortex_m_rtfm as rtfm;
extern crate valuelinediscovery as dsc;

use dsc::stm32f100::interrupt::Tim6DacIrq;
use dsc::stm32f100;
use dsc::timer::Timer6;
use dsc::button::{Button};
use rtfm::{Local, P0, P1, T0, T1, TMax};


const FREQUENCY: u32 = 200; // Hz

// RESOURCES
// have to register all periphs that we're using
peripherals!(stm32f100, {
    GPIOA:  Peripheral {
        register_block: Gpioa,
        ceiling: C1, // kinda like a priority
    },
    RCC: Peripheral {
        register_block: Rcc,
        ceiling: C1,
    },
    TIM6: Peripheral {
        register_block: Tim6,
        ceiling: C1,
    },
});


// Initialisation
fn init(ref priority: P0, threshold: &TMax) {
    let gpioa = GPIOA.access(priority, threshold);
    let rcc = RCC.access(priority, threshold);
    let tim6 = TIM6.access(priority, threshold);

    let timer6 = Timer6(&tim6);
    let button = Button(&gpioa);

    // configure the PCx pins as outputs
    button.init(&rcc);

    // configure timer7 for periodic update events
    timer6.init(&rcc, FREQUENCY);

    // start the timer
    timer6.resume();
}

static mut COUNT: u16 = 0;
static mut PREV: u16 = 0;

fn idle(_priority: P0, _threshold: T0) -> ! {
    loop {
        unsafe {
        if PREV != COUNT {
            hprintln!("{}", COUNT);
        }
        PREV = COUNT;
        }
        //rtfm::wfi(); // this freezes JTAG, so don't do it
    }
}


// TASKS
tasks!(stm32f100, {
    inputs: Task {
        interrupt: Tim6DacIrq,
        priority: P1,
        enabled: true,
    },
});

// Interrupt handler, essentially.
fn inputs(mut task: Tim6DacIrq, ref priority: P1, ref threshold: T1) {

    // Task local data
    //static COUNT: Local<u16, Tim6DacIrq> = Local::new(0);

    let gpioa = GPIOA.access(priority, threshold);
    let tim6 = TIM6.access(priority, threshold);

    let timer6 = Timer6(&tim6);
    let button = Button(&gpioa);

    // clear the interrupt flag
    if timer6.clear_update_flag().is_ok() {
        if button.is_pressed() {
                unsafe {COUNT += 1} ;
        }


    } else {
        // only reachable thru `rtfm::request(periodic)
        #[cfg(debug_assertion)]
        unreachable!()
    }
}

