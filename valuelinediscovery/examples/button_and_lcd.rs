//! Counts button presses and displays the result on the LCD

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
use dsc::lcd::Lcd;
use rtfm::{Local, P0, P1, T0, T1, TMax};

extern crate numtoa;
use numtoa::NumToA;

const FREQUENCY: u32 = 200; // Hz

// RESOURCES
// have to register all periphs that we're using
peripherals!(stm32f100, {
    GPIOA:  Peripheral {
        register_block: Gpioa,
        ceiling: C1, // kinda like a priority
    },
    GPIOC:  Peripheral {
        register_block: Gpioc,
        ceiling: C0,
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
    let rcc = RCC.access(priority, threshold);
    let gpioa = GPIOA.access(priority, threshold);
    let gpioc = GPIOC.access(priority, threshold);
    let tim6 = TIM6.access(priority, threshold);

    let timer6 = Timer6(&tim6);
    let button = Button(&gpioa);

    // configure the PCx pins as outputs
    button.init(&rcc);

    // configure timer for periodic update events
    timer6.init(&rcc, FREQUENCY);

    // start the LCD
    let lcd = Lcd(&gpioc);
    lcd.init(&rcc);
    lcd.clear();
    lcd.write(b"Times you pressed");
    lcd.set_position(1,0);
    lcd.write(b"the button:");

    // start the timer
    timer6.resume();
}

static mut COUNT: u16 = 0;

fn idle(ref priority: P0, ref threshold: T0) -> ! {
    loop {
        unsafe {
            let gpioc = GPIOC.access(priority, threshold);
            let lcd = Lcd(&gpioc);
            let mut bytes = [b'\0';10];
            let n = COUNT.numtoa(10, &mut bytes);
            lcd.set_position(3,0);
            lcd.write(&bytes[n..]);
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

