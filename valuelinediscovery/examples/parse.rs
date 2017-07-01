//! Parse serial info
//!

#![feature(const_fn)]
#![feature(used)]
#![no_std]

// Basic Cortex-M support
extern crate cortex_m_rt;

// Real time for the masses - scheduler (RTFM)
#[macro_use]
extern crate cortex_m_rtfm as rtfm;
use rtfm::{ Local, C1, P0, P1, T0, T1, TMax};
use rtfm::Resource; // data to share between tasks

// common datastructures backed by statically allocated memory
extern crate heapless;
use heapless::Vec;
use core::cell::Cell;

// board and chip specific crates
extern crate valuelinediscovery as dsc;
use dsc::led::{self, LEDS};
use dsc::serial::Serial;
use dsc::stm32f100::interrupt::{Usart1Irq,Tim7Irq};
use dsc::stm32f100;
use dsc::timer::Timer;

// casting, etc
extern crate cast;
use cast::{u8, usize};

// constants
const FREQUENCY: u32 = 2; // Hz
pub const BAUD_RATE: u32 = 115_200; // bits per second


// RESOURCES
// have to register all periphs that we're using
peripherals!(stm32f100, {
    GPIOA:  Peripheral {
        register_block: Gpioa,
        ceiling: C0, // kinda like a priority
    },
    GPIOC:  Peripheral {
        register_block: Gpioc,
        ceiling: C0, // kinda like a priority
    },
    RCC: Peripheral {
        register_block: Rcc,
        ceiling: C0,
    },
    USART1: Peripheral {
        register_block: Usart1,
        ceiling: C1,
    },
    TIM7: Peripheral {
        register_block: Tim7,
        ceiling: C1,
    },
});


// Initialisation
fn init(ref priority: P0, threshold: &TMax) {

    // common
    let rcc = RCC.access(priority, threshold);

    // stuff for serial loopback
    let gpioa = GPIOA.access(priority, threshold);
    let usart1 = USART1.access(priority, threshold);

    // stuff for LED roulette
    let gpioc = GPIOC.access(priority, threshold);
    let tim7 = TIM7.access(priority, threshold);

    // Initialise the serial port
    let serial = Serial(&usart1);
    serial.init(&gpioa, &rcc, BAUD_RATE);

    // Initialise LED roulette
    led::init(&gpioc, &rcc);
    let timer = Timer(&tim7);
    timer.init(&rcc, FREQUENCY);
    timer.resume();
}


fn idle(_priority: P0, _threshold: T0) -> ! {
    loop {
        //rtfm::wfi(); // this freezes JTAG, so don't
    }
}

// Data to share between tasks
struct State {
    mode: Cell<Mode>,
}

impl State {
    const fn new() -> Self {
        State {
            mode: Cell::new(Mode::Both),
        }
    }
}

#[derive(Clone,Copy)]
enum Mode {
    Both,
    Green,
    Blue,
}



// RTFM framework provides a `Resource` abstraction used to share
// memory between two or more tasks in a data race free manner.
static SHARED: Resource<State, C1> = Resource::new(State::new());

// TASKS
tasks!(stm32f100, {
    receive: Task {
        interrupt: Usart1Irq,
        priority: P1,
        enabled: true,
    },
    roulette: Task {
        interrupt: Tim7Irq,
        priority: P1,
        enabled: true,
    },
});

// Serial loopback handler
fn receive(mut task: Usart1Irq, ref priority: P1, ref threshold: T1) {

    // 16 byte buffer
    static BUFFER: Local<Vec<u8, [u8; 16]>, Usart1Irq> = {
        Local::new(Vec::new([0; 16]))
    };

    let usart1 = USART1.access(priority, threshold);
    let serial = Serial(&usart1);

    if let Ok(byte) = serial.read() {
        if serial.write(byte).is_err() {
            // As we are echoing bytes as soon as they arrive,
            // it should be impossible to have a TX buffer overrun
            #[cfg(debug_assertions)]
            unreachable!()
        }

        let buffer = BUFFER.borrow_mut(&mut task);

        if byte == b'\r' {
            // end of command

            // borrow the shared data
            let shared = SHARED.access(priority, threshold);
            match &**buffer {
                b"both" => shared.mode.set(Mode::Both),
                b"blue" => shared.mode.set(Mode::Blue),
                b"green" => shared.mode.set(Mode::Green),
                _ => {}
            }

            // clear the buffer and prepare for the next command
            buffer.clear();
        } else {
            // push the byte into the buffer
            if buffer.push(byte).is_err() {
                // error: buffer full
                // KISS: just clear it when it gets full
                buffer.clear();
            }
        }

    } else {
        // only reachabvle thru `rtfm::request(loopback)`
        #[cfg(debug_assertions)]
        unreachable!()
    }


}

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

        let shared = SHARED.access(priority, threshold);
        let mode = shared.mode.get();

        LEDS[usize(curr)].off();
    
        if next == 0 {
            match mode {
                Mode::Both | Mode::Blue => LEDS[usize(next)].on(),
                _ => LEDS[usize(next)].off(),
            }
        } else if next == 1 {
            match mode {
                Mode::Both | Mode::Green => LEDS[usize(next)].on(),
                _ => LEDS[usize(next)].off(),
            }
        }

        *state = next;
    } else {
        // only reachable thru `rtfm::request(periodic)
        #[cfg(debug_assertion)]
        unreachable!()
    }
}
