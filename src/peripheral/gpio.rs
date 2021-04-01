use crate::peripheral::Register;
use core::marker::PhantomData;

const GPIOC_ORIGIN: usize = 0x40011000;
const GPIOC_CRL: usize = GPIOC_ORIGIN;
const GPIOC_CRH: usize = GPIOC_ORIGIN + 0x04;
const GPIOC_BSRR: usize = GPIOC_ORIGIN + 0x10;

pub struct Input {}
pub struct Output {}

pub trait InputPin {
    fn is_high(&self);
    fn is_low(&self);
}

pub trait OutputPin {
    fn set_high(&mut self);
    fn set_low(&mut self);
}

trait PortAware {
    const BSRR: Register;
}

pub struct Pin<MODE, const PORT: char, const INDEX: u8> {
    _marker: PhantomData<MODE>
}

impl<MODE, const PORT: char, const INDEX: u8> Pin<MODE, PORT, INDEX> {

    const fn new() -> Self {
        Self { _marker: PhantomData }
    }

    pub fn into_push_pull_output(self, cr: Register) -> Pin<Output, PORT, INDEX> {
        const CNF: u32 = 0b00;
        // default speed 2MHZ
        const MODE: u32 = 0b10;
        // bits to set
        const BITS: u32 = (CNF << 2) | MODE;

        let cr_offset: u32 = (4 * INDEX as u32) % 32;

        // reset pin
        <Self as PortAware>::BSRR.write(1 << (16 + INDEX));

        // clear previous configuration for this pin. then set the new one
        cr.write(cr.bits() & !(0b1111 << cr_offset) | BITS << cr_offset);

        Pin::new()
    }

}

impl<const PORT: char, const INDEX: u8> OutputPin for Pin<Output, PORT, INDEX> {
    fn set_high(&mut self) {
        // self.bsrr().write(1 << INDEX);
    }

    fn set_low(&mut self) {
        // self.bsrr().write(1 << (16 + INDEX));
    }
}

pub struct GPIO {
    pub crl: Register,
    pub crh: Register,
    pub pc13: Pin<Input, 'c', 13>,
}

impl GPIO {
    pub(in crate::peripheral) const fn new() -> GPIO {
        GPIO {
            crl: Register(GPIOC_CRL as *const usize),
            crh: Register(GPIOC_CRH as *const usize),
            pc13: Pin::new(),
        }
    }
}

impl<MODE, const INDEX: u8> PortAware for Pin<MODE, 'c', INDEX> {
    const BSRR: Register = Register(GPIOC_BSRR as *const usize);
}

// impl GPIO {
//
//
//     fn bsrr<const PORT: char>()  -> Register  {
//         Register(GPIOC_BSRR as *const usize)
//     }
//
// }
