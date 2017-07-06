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
        for _ in 0..6 {
            asm::nop();
        }
        self.nibble(Register::Instruction, Operation::Write, 0x3);
        for _ in 0..6 {
            asm::nop();
        }
        self.nibble(Register::Instruction, Operation::Write, 0x3);
        for _ in 0..6 {
            asm::nop();
        }

        /// put it into 4-bit mode
        self.nibble(Register::Instruction, Operation::Write, 0x2);
        for _ in 0..6 {
            asm::nop();
        }

        // from now on, 4-bit mode
        /// 2-line mode
        self.word(Register::Instruction, Operation::Write, 0x28);
        for _ in 0..6 {
            asm::nop();
        }

        /// Clear display
        self.word(Register::Instruction, Operation::Write, 0x01);
        for _ in 0..40 {
            asm::nop();
        }

        // Switch it on for now
        self.word(Register::Instruction, Operation::Write, 0x02);
        for _ in 0..40 {
            asm::nop();
        }
        //self.word(Register::Instruction, Operation::Write, 0x07);
        self.word(Register::Instruction, Operation::Write, 0x0F);
        for _ in 0..6 {
            asm::nop();
        }

        

        // write shit
        self.word(Register::Data, Operation::Write, 0x30);
        for _ in 0..16 {
            asm::nop();
        }
        self.word(Register::Data, Operation::Write, 0x41);
        for _ in 0..16 {
            asm::nop();
        }
        self.word(Register::Data, Operation::Write, 0x52);
        for _ in 0..16 {
            asm::nop();
        }
        self.word(Register::Data, Operation::Write, 0x63);
        for _ in 0..16 {
            asm::nop();
        }

        //self.nop_timer();
        //spam
        for c in 0x20..0x50 {
            self.word(Register::Data, Operation::Write, c as u8);
            for _ in 0..16 {
                asm::nop();
            }
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

        //
        // Toggle Enable
        //

        // tAS - Address Setup Time (140ns min)
        //asm:::nop(); // 125ns (8MHz)

        asm::nop();
        asm::nop();
        gpioc.bsrr.write(|w| w.bs13().set());
        asm::nop();
        asm::nop();
        gpioc.bsrr.write(|w| w.br13().reset());
        asm::nop();
        asm::nop();

        // Send 4 LSBs
        cs &= 0xFFFF_FFF0;
        let lsbs = data as u32 & 0x0000_000F;
        cs |= lsbs;
        unsafe { gpioc.odr.write( |w| w.bits(cs) ) };
        
        // Toggle Enable
        asm::nop();
        asm::nop();
        gpioc.bsrr.write(|w| w.bs13().set());
        asm::nop();
        asm::nop();
        gpioc.bsrr.write(|w| w.br13().reset());
        asm::nop();
        asm::nop();

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

    /*
    fn nop_timer(self) {
        let gpioc = self.0;

        gpioc.bsrr.write(|w| w.bs13().set());
        for _ in 0..4 {
            asm::nop();
        }
        gpioc.bsrr.write(|w| w.br13().reset());

        gpioc.bsrr.write(|w| w.bs13().set());
        for _ in 0..8 {
            asm::nop();
        }
        gpioc.bsrr.write(|w| w.br13().reset());


        gpioc.bsrr.write(|w| w.bs13().set());
        for _ in 0..16 {
            asm::nop();
        }
        gpioc.bsrr.write(|w| w.br13().reset());

        gpioc.bsrr.write(|w| w.bs13().set());
        for _ in 0..32 {
            asm::nop();
        }
        gpioc.bsrr.write(|w| w.br13().reset());

    }
    */
}
