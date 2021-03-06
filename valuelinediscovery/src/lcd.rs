//! 4x20 LCD Screen
//! JHD 204A

/// LCD     Pin     Direction   Function
/// RS      PC5     Output      H: Data Register L: Instruction Register
/// R/Wb    PC4     Output      H: Read L: Write (mostly write)
/// E       PC13    Output      Enable signal (falling edge)
/// DB4     PC0     Output      Data line
/// DB5     PC1     Output      Data line
/// DB6     PC2     Output      Data line
/// DB7     PC3     Output      Data line (MSB)

/// !!!FIXME!!! timing screws up in release build

use stm32f100::{Gpioc, Rcc};

use cortex_m::asm;

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

        // Power up GPIOC peripheral
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
                     .mode4().output_10mhz().cnf4().push_pull()
                     .mode5().output_10mhz().cnf5().push_pull()
                },
            );
        gpioc
            .crh
            .modify(
                |_,w| {
                    w.mode13().output_10mhz().cnf13().push_pull()
                },
            );


        /// tap 3 times to put LCD in a known state
        self.nibble(Register::Instruction, Operation::Write, 0x3);
        self.nibble(Register::Instruction, Operation::Write, 0x3);
        self.nibble(Register::Instruction, Operation::Write, 0x3);

        /// put it into 4-bit mode
        self.nibble(Register::Instruction, Operation::Write, 0x2);

        // from now on, 4-bit mode
        /// 2-line mode
        self.word(Register::Instruction, Operation::Write, 0x28);

        /// Clear display
        self.clear();

        // Switch it on for now
        self.word(Register::Instruction, Operation::Write, 0x0F);

        // write shit
        self.word(Register::Data, Operation::Write, 0x30);
        self.word(Register::Data, Operation::Write, 0x41);
        self.word(Register::Data, Operation::Write, 0x52);
        self.word(Register::Data, Operation::Write, 0x63);

        for c in 0x20..0x70 {
            self.word(Register::Data, Operation::Write, c as u8);
        }

    }



    /// Wiggle the pins appropriately to write a byte to the LCD ib 4-bit mode
    fn word(self, reg: Register, op: Operation, data: u8) {
        let gpioc = self.0;
        let mut cs = gpioc.odr.read().bits();
        cs &= 0xFFFF_DFC0;

        cs = match op {
            Operation::Read  => cs | 0x0000_0010,
            Operation::Write => cs & 0xFFFF_FFEF,
        };

        cs = match reg {
            Register::Data        => cs | 0x0000_0020,
            Register::Instruction => cs & 0xFFFF_FFDF,
        };

        // Send 4 MSBs
        let msbs = (data >> 4) as u32 & 0x0000_000F;
        cs |= msbs;
        unsafe { gpioc.odr.write( |w| w.bits(cs) ) };

        // Toggle Enable
        gpioc.bsrr.write(|w| w.bs13().set());
        gpioc.bsrr.write(|w| w.br13().reset());

        // Send 4 LSBs
        cs &= 0xFFFF_FFF0;
        let lsbs = data as u32 & 0x0000_000F;
        cs |= lsbs;
        unsafe { gpioc.odr.write( |w| w.bits(cs) ) };
        
        // Toggle Enable
        gpioc.bsrr.write(|w| w.bs13().set());
        gpioc.bsrr.write(|w| w.br13().reset());
    }



    /// Send a nibble to the LCD module
    fn nibble(self, reg: Register, op: Operation, data: u8) {
        let gpioc = self.0;
        let mut cs = gpioc.odr.read().bits();
        cs &= 0xFFFF_DFC0;

        cs = match op {
            Operation::Read  => cs | 0x0000_0010,
            Operation::Write => cs & 0xFFFF_FFEF,
        };

        cs = match reg {
            Register::Data        => cs | 0x0000_0020,
            Register::Instruction => cs & 0xFFFF_FFDF,
        };

        let d32 = data as u32 & 0x0000_000F;
        cs |= d32;

        // Setup cycle
        unsafe { gpioc.odr.modify( |_,w| w.bits(cs) ) };

        // Enable cycle
        cs |= 0x0000_F000; // set enable on C13
        unsafe { gpioc.odr.modify( |_,w| w.bits(cs) ) };

        // Leadout cycle
        cs &= 0xFFFF_0FFF;  // clear enable on C13
        unsafe { gpioc.odr.modify( |_,w| w.bits(cs) ) };

    }

    /// Clear the display
    pub fn clear(self) {
        self.word(Register::Instruction, Operation::Write, 0x01);
        for _ in 0..40 { // 1.6ms min
            asm::nop();
        }
    }

    /// Write an ascii string (`u8` slice) to the display
    /// The user can deal with the line-skipping if the `u8` slice
    /// is going to overflow an LCD line
    pub fn write(self, msg: &[u8]) {
        for c in msg {
            self.word(Register::Data, Operation::Write, *c);
        }
    }

    /// Set the position of the cursor - the position of the next char
    pub fn set_position(self, row:u8, col:u8) {
        // should assert or return an error or something?
        if col > 19 || row > 3 {
            return;
        }
        let mut adr:u8 = match row {
            0 => 0x00 + col,
            1 => 0x40 + col,
            2 => 0x14 + col,
            3 => 0x54 + col,
            _ => 0x00
        };
        adr &= 0x7F;
        adr |= 0x80;
        self.word(Register::Instruction, Operation::Write, adr);
    }
}

