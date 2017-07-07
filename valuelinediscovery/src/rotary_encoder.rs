//! examples/rotary-encoder.rs
//!
//! Driver to decode an incremental rotary encoder
//!
//! ## Pin connections
//!
//!              +-------+
//! (in, pullup) |       |
//! PA1    <---- o       o ----> PA2, input with pullup.
//!              |       o ----> GND
//! GND    <---- o       o ----> PA3, input with pullup.
//!              |       |
//!              +-------+
//!
//!
//! ## Waveforms for Clockwise (CW) rotation
//!          _____       ________
//!     PA3       |_____|
//!          ________       _____
//!     PA2          |_____|
//!
//! ## Waveforms for Counter-Clockwise (CCW) rotation
//!          ________       _____
//!     PA3          |_____|
//!          _____       ________
//!     PA2       |_____|
//!
//!
//! ## Decoding Method
//! 1. Set up a timer to poll PortA
//! 2. In the ISR:
//!   2.1 Read PortA
//!   2.2 Update state of thing
//!


use stm32f100::{Gpioa, Rcc};
//use cortex_m::asm;

#[derive(Copy, Clone, PartialEq)]
pub enum State {
    IDLE,
    CCW,
    CW,
    BUTTON,
}


#[derive(Copy, Clone, PartialEq)]
enum EncoderState {
    Idle,
    CW01,
    CW00,
    CW10,
    CCW10,
    CCW00,
    CCW01,
}

use self::EncoderState::*;

static LUT : [[EncoderState; 4]; 7] = [
    //           00       01       10       11
    /* Idle  */ [Idle,    CW01,    CCW10,   Idle],
    /* CW01  */ [CW00,    CW01,    CW01,    Idle],
    /* CW00  */ [CW00,    CW01,    CW10,    Idle],
    /* CW10  */ [CW00,    CW10,    CW10,    Idle],
    /* CCW10 */ [CCW00,   CCW10,   CCW10,   Idle],
    /* CCW00 */ [CCW00,   CCW01,   CCW10,   Idle],
    /* CCW01 */ [CCW00,   CCW01,   CCW01,   Idle],
    ];

static mut STATE_ENC01: EncoderState = Idle;

static GPIOA_MASK : u32 = 0x0000_000C;


///
/// # Rotary Encoder
///
#[derive(Clone,Copy)]
pub struct RotaryEncoder<'a>(pub &'a Gpioa);

impl <'a> RotaryEncoder<'a> {

    /// Initialise the pins for the rotary encoder
    pub fn init(self, rcc: &Rcc) {
        let gpioa = self.0;

        // Power up GPIOA peripheral
        rcc.apb2enr.modify(|_,w| w.iopaen().enabled());

        // set up the pin directions
        gpioa
            .crl
            .modify(
                |_,w| { // inputs. digital input with pull
                    w.mode1().input().cnf1().alt_push_pull()
                     .mode2().input().cnf2().alt_push_pull()
                     .mode3().input().cnf3().alt_push_pull()
                }
            );

        // enable the pullups
        gpioa
            .bsrr
            .write(
                |w| { // set to one for pullup
                    w.bs1().set()
                     .bs2().set()
                     .bs3().set()
                }
            );

    }


    /// Return an event
    /// This is to be called in a timer interrupt
    pub fn state(self) -> State {
        let gpioa = self.0;
        let value = gpioa.idr.read().bits() & GPIOA_MASK;
        let input = value >> 2;

        let mut state = State::IDLE;
        unsafe {
            let next_state = LUT[STATE_ENC01 as usize][input as usize];

            if (STATE_ENC01 == CW10) && (next_state == Idle) {
                state = State::CCW;
            } else if (STATE_ENC01 == CCW01) && ( next_state == Idle) {
                state = State::CW;
            }
            STATE_ENC01 = next_state;
        }
        state
    }
}

