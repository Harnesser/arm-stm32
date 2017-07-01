//! Serial Loopback
//!

#![feature(const_fn)]
#![feature(used)]
#![no_std]

extern crate cortex_m_rt;
#[macro_use]
extern crate cortex_m_rtfm as rtfm;
extern crate valuelinediscovery as dsc;

use dsc::serial::Serial;
use dsc::stm32f100::interrupt::Usart1Irq;
use dsc::stm32f100;
use rtfm::{ P0, P1, T0, T1, TMax};

pub const BAUD_RATE: u32 = 115_200; // bits per second

// RESOURCES
// have to register all periphs that we're using
peripherals!(stm32f100, {
    GPIOA:  Peripheral {
        register_block: Gpioa,
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
});


// Initialisation
fn init(ref priority: P0, threshold: &TMax) {
    let gpioa = GPIOA.access(priority, threshold);
    let rcc = RCC.access(priority, threshold);
    let usart1 = USART1.access(priority, threshold);

    let serial = Serial(&usart1);
    serial.init(&gpioa, &rcc, BAUD_RATE);
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
