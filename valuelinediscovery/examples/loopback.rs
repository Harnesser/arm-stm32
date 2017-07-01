//! Serial Loopback
//!

#![feature(const_fn)]
#![feature(used)]
#![no_std]

extern crate cortex_m_rt;
#[macro_use]
extern crate cortex_m_rtfm as rtfm;
extern crate valuelinediscovery as dsc;

//use dsc::led::{self, LEDS};
use dsc::serial::Serial;
use dsc::stm32f100::interrupt::Usart1Irq;
use dsc::stm32f100;
//use dsc::timer::Timer;
//use rtfm::{ Local, P0, P1, T0, T1, TMax};
use rtfm::{ P0, P1, T0, T1, TMax};

//extern crate cast;
//use cast::{u8, usize};

//const FREQUENCY: u32 = 2; // Hz
pub const BAUD_RATE: u32 = 115_200; // bits per second

// RESOURCES
// have to register all periphs that we're using
peripherals!(stm32f100, {
    GPIOA:  Peripheral {
        register_block: Gpioa,
        ceiling: C0, // kinda like a priority
    },
//    GPIOC:  Peripheral {
//        register_block: Gpioc,
//        ceiling: C0, // kinda like a priority
//    },
    RCC: Peripheral {
        register_block: Rcc,
        ceiling: C0,
    },
    USART1: Peripheral {
        register_block: Usart1,
        ceiling: C1,
    },
//    TIM7: Peripheral {
//        register_block: Tim7,
//        ceiling: C1,
//    },
});


// Initialisation
fn init(ref priority: P0, threshold: &TMax) {
    let gpioa = GPIOA.access(priority, threshold);
    let rcc = RCC.access(priority, threshold);
    let usart1 = USART1.access(priority, threshold);

    let serial = Serial(&usart1);
    serial.init(&gpioa, &rcc, BAUD_RATE);

/*
    let gpioc = GPIOC.access(priority, threshold);
    let tim7 = TIM7.access(priority, threshold);
    let timer = Timer(&tim7);

    // configure the PCx pins as outputs
    led::init(&gpioc, &rcc);

    // configure timer7 for periodic update events
    timer.init(&rcc, FREQUENCY);

    // start the timer
    timer.resume();
    */
}


fn idle(_priority: P0, _threshold: T0) -> ! {
    loop {
        //rtfm::wfi(); // this freezes JTAG, so don't
    }
}


// TASKS
tasks!(stm32f100, {
    loopback: Task {
        interrupt: Usart1Irq,
        priority: P1,
        enabled: true,
    },
});

// Serial loopback handler
fn loopback(_task: Usart1Irq, ref priority: P1, ref threshold: T1) {
    let usart1 = USART1.access(priority, threshold);
    let serial = Serial(&usart1);

    if let Ok(byte) = serial.read() {
        if serial.write(byte).is_err() {
            // As we are echoing bytes as soon as they arrive,
            // it should be impossible to have a TX buffer overrun
            #[cfg(debug_assertions)]
            unreachable!()
        }
    } else {
        // only reachabvle thru `rtfm::request(loopback)`
        #[cfg(debug_assertions)]
        unreachable!()
    }


}

/*
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
*/
