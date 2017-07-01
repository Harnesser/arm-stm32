//! Serial interface
//!
//! - TX - PA9
//! - RX - PA10

use core::ptr;

use cast::{u16, u8};
use stm32f100::{Gpioa, Rcc, Usart1};

use frequency;

/// Specialized `Result` type
pub type Result<T> = ::core::result::Result<T, Error>;

/// An error
pub struct Error {
    _0: (),
}

/// Serial interface
///
/// # Interrupts
///
/// - `Usart1Irq` - RXNE (RX buffer not empty)
#[derive(Clone, Copy)]
pub struct Serial<'a>(pub &'a Usart1);

impl<'a> Serial<'a> {
    /// Initializes the serial interface with a baud rate of `baut_rate` bits
    /// per second
    pub fn init(self, gpioa: &Gpioa, rcc: &Rcc, baud_rate: u32) {
        let usart1 = self.0;

        // Power up the peripherals
        rcc.apb2enr.modify(|_, w| w.usart1en().enabled());
        rcc.apb2enr.modify(|_, w| w.iopaen().enabled());

        // Table 22 in the Reference Manual
        // PA9 as TX
        gpioa 
            .crh
            .modify(|_, w| w.mode9().output_10mhz().cnf9().alt_push_pull() );
        // PA10 as RX
        gpioa
            .crh
            .modify(|_, w| w.mode10().input().cnf10().open_drain() );
            // (open drain, really floating input)

        // 8 data bits, 0 stop bits
        usart1.cr2.write(|w| unsafe { w.stop().bits(0b00) });

        // Disable hardware flow control
        usart1
            .cr3
            .write(|w| unsafe { w.rtse().bits(0).ctse().bits(0) });

        // set baud rate
        let brr = u16(frequency::APB2 / baud_rate).unwrap();
        let fraction = u8(brr & 0b1111).unwrap();
        let mantissa = brr >> 4;
        usart1
            .brr
            .write(
                |w| unsafe {
                    w.div_fraction()
                        .bits(fraction)
                        .div_mantissa()
                        .bits(mantissa)
                },
            );

        // enable peripheral, transmitter, receiver
        // enable RXNE event
        usart1
            .cr1
            .write(
                |w| unsafe {
                    w.ue()              // USART enable
                        .bits(1)
                        .re()           // Rx enable
                        .bits(1)
                        .te()           // Tx Enable
                        .bits(1)
                        .pce()          // Parity Control Enable
                        .bits(0)
                        //.over8()        // Oversampling mode
                        //.bits(0)
                        .rxneie()       // Rx Not Empty interrupt enable
                        .bits(1)
                },
            );
    }

    /// Reads a byte from the RX buffer
    ///
    /// Returns `None` if the buffer is empty
    pub fn read(self) -> Result<u8> {
        let usart1 = self.0;

        if usart1.sr.read().rxne().bits() == 1 {
            // NOTE(read_volatile) the register is 9 bits big but we'll only
            // work with the first 8 bits
            Ok(
                unsafe {
                    ptr::read_volatile(&usart1.dr as *const _ as *const u8)
                },
            )
        } else {
            Err(Error { _0: () })
        }
    }

    /// Writes byte into the TX buffer
    ///
    /// Returns `Err` if the buffer is already full
    pub fn write(self, byte: u8) -> Result<()> {
        let usart1 = self.0;

        if usart1.sr.read().txe().bits() == 1 {
            unsafe {
                ptr::write_volatile(&usart1.dr as *const _ as *mut u8, byte)
            }
            Ok(())
        } else {
            Err(Error { _0: () })
        }
    }
}
