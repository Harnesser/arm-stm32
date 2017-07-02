//! Detect the blue user button on the board (PA0)

use stm32f100::{ /*GPIOA,*/ Gpioa, Rcc};

static mut COUNT: u16 = 0;

///
/// # Button
///
#[derive(Clone,Copy)]
pub struct Button<'a>(pub &'a Gpioa);

impl<'a> Button<'a> {
    /// Initialises the button pin PA0 as an input with a pullup
    pub fn init(self, rcc: &Rcc) {
        let gpioa = self.0;

        // Power up GPIOA peripheral
        rcc.apb2enr.modify(|_,w| w.iopaen().enabled());

        // configure PA0 as an input
        gpioa
            .crl
            .modify(
                |_,w| {
                    w.mode0().input()
                        .cnf0().open_drain() // really, floating input
                },
            );
            
    }

    /// Check if the button is pressed
    pub fn is_pressed(self) -> bool {

        let gpioa = self.0;
        let masked_reg = gpioa.idr.read().bits() & 0x1;
        let mut new_in: u16 = 1;
        if masked_reg != 0 {
            new_in = 0;
        }
        unsafe {
            COUNT = (COUNT << 1) | new_in | 0xE000;
            COUNT == 0xF000
        }
        //new_in == 1
    }

}
