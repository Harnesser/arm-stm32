//! 4x20 LCD Screen
//! JHD 204A

/// LCD     Pin     Direction   Function
/// RS      PC15    Output      H: Data Register L: Instruction Register
/// R/Wb    PC14    Output      H: Read L: Write (mostly write)
/// E       PC13    Output      Enable signal (falling edge)
/// DB4     PC0     Output      Data line
/// DB5     PC1     Output      Data line
/// DB6     PC2     Output      Data line
/// DB7     PC3     Output      Data line (MSB)

use stm32f100::{Gpioc, Rcc};

/// LCD Module Register Type
#[derive(Copy,Clone,PartialEq)]
pub enum Register {
    /// Instruction Register
    Instruction,
    /// Data Register
    Data,
}

/// LCD Operation Type
#[derive(Copy,Clone,PartialEq)]
pub enum Operation {
    /// Write Operation
    Write,
    /// Read Operation
    Read,
}

///
/// # Liquid Crystal Display Driver
///
#[derive(Clone,Copy)]
pub struct Lcd<'a>(pub &'a Gpioc);


impl<'a> Lcd<'a> {

    /// Initialise the LCD Driver, and the LCD itself
    pub fn init(self, rcc: &Rcc) {
        let gpioc = self.0;

        // Power up GPIOA peripheral
        rcc.apb2enr.modify(|_,w| w.iopcen().enabled());

        // set up the pin directions
        gpioc
            .crl
            .modify(
                |_,w| {
                    w.mode0().output_10mhz().cnf0().push_pull()
                     .mode1().output_10mhz().cnf1().push_pull()
                     .mode2().output_10mhz().cnf2().push_pull()
                     .mode3().output_10mhz().cnf3().push_pull()
                },
            );
        gpioc
            .crh
            .modify(
                |_,w| {
                    w.mode13().output_10mhz().cnf13().push_pull()
                     .mode14().output_10mhz().cnf14().push_pull()
                     .mode15().output_10mhz().cnf15().push_pull()
                },
            );



        /// tap 3 times to put LCD in a known state
        self.nibble(Register::Instruction, Operation::Write, 0x03);
        self.nibble(Register::Instruction, Operation::Write, 0x03);
        self.nibble(Register::Instruction, Operation::Write, 0x03);
    }

/*
    /// Wiggle the pins appropriately to write a byte to the LCD
    fn write(self, op: Operation, data: u8) {
        self.nibble(op, data >> 4);
        self.nibble(op, data);
    }
*/

    /// Send a nibble to the LCD module
    fn nibble(self, reg: Register, op: Operation, data: u8) {
        let gpioc = self.0;
        let mut cs = gpioc.odr.read().bits();
        cs &= 0xFFFF_1FF0;

        if op == Operation::Read {
            cs |= 0x0000_4000; // C14
        } else {
            cs &= 0xFFFF_BFFF; // C14
        }

        if reg == Register::Data {
            cs |= 0x0000_8000; // C15
        } else {
            cs &= 0xFFFF_7FFF;
        }

        let d32 = data as u32 & 0x0000_000F;
        cs |= d32;

        // Setup cycle
        unsafe { gpioc.odr.write( |w| w.bits(cs) ) };

        // Enable cycle
        cs |= 0x0000_2000; // set enable on C13
        unsafe { gpioc.odr.write( |w| w.bits(cs) ) };

        // Leadout cycle
        cs &= 0xFFFF_DFFF;  // clear enable on C13
        unsafe { gpioc.odr.write( |w| w.bits(cs) ) };

    }

    /*
    /// Set the LCD driver to the idle state
    fn idle(self){
        let gpioc = self.0;
        gpioc
            .bsrr
            .write(|w|
                w.br().

    }
*/

}

