use crate::peripheral::Register;
use core::marker::PhantomData;

const GPIOC_ORIGIN: usize = 0x40011000;
const GPIOC_CRL: usize = GPIOC_ORIGIN;
const GPIOC_CRH: usize = GPIOC_ORIGIN + 0x04;
const GPIOC_BSRR: usize = GPIOC_ORIGIN + 0x10;

pub trait RegisterAware {
    const BSRR: Register;
}

pub struct PortA {}
pub struct PortB {}
pub struct PortC {}

impl RegisterAware for PortC{ const BSRR: Register = Register(GPIOC_BSRR as *const usize); }

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

pub struct Pin<MODE, PORT, const INDEX: u8> {
    _marker_mode: PhantomData<MODE>,
    _marker_port: PhantomData<PORT>
}

impl<MODE, PORT, const INDEX: u8> Pin<MODE, PORT, INDEX> where PORT: RegisterAware {

    const fn new() -> Self {
        Self { _marker_mode: PhantomData, _marker_port: PhantomData }
    }

    pub fn into_push_pull_output(self, cr: Register) -> Pin<Output, PORT, INDEX> {
        const CNF: u32 = 0b00;
        // default speed 2MHZ
        const MODE: u32 = 0b10;
        // bits to set
        const BITS: u32 = (CNF << 2) | MODE;

        let cr_offset: u32 = (4 * INDEX as u32) % 32;

        // reset pin
        <PORT as RegisterAware>::BSRR.write(1 << (16 + INDEX));

        // clear previous configuration for this pin. then set the new one
        cr.write(cr.bits() & !(0b1111 << cr_offset) | BITS << cr_offset);

        Pin::new()
    }

    pub fn into_intput(self) -> Pin<Input, PORT, INDEX> {
        Pin::new()
    }

}

impl<PORT, const INDEX: u8> OutputPin for Pin<Output, PORT, INDEX> where PORT: RegisterAware {
    fn set_high(&mut self) {
        <PORT as RegisterAware>::BSRR.write(1 << INDEX);
    }

    fn set_low(&mut self) {
        <PORT as RegisterAware>::BSRR.write(1 << (16 + INDEX));
    }
}

pub struct GPIO {
    pub crl: Register,
    pub crh: Register,
    pub pc13: Pin<Input, PortC, 13>,
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

// impl GPIO {
//
//
//     fn bsrr<const PORT: char>()  -> Register  {
//         Register(GPIOC_BSRR as *const usize)
//     }
//
// }
