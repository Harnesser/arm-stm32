//! Prints "Hello" then "World" on the OpenOCD console

#![feature(used)]
#![no_std]

#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;

#[macro_use]
extern crate cortex_m_rtfm as rtfm;

extern crate valuelinediscovery as dsc;

use dsc::stm32f100;
use rtfm::{P0, T0, TMax};

tasks!(stm32f100, {});

fn init(_priority: P0, _threshold: &TMax) {
    hprintln!("Hello");
}

fn idle(_priority: P0, _threshold: T0) -> ! {
    hprintln!("World");

    loop {
        //rtfm::wfi(); // this freezes JTAG, so don't do it
    }
}

