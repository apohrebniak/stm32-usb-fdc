use crate::peripheral::Register;
use core::marker::PhantomData;
use paste::paste;

const GPIOA_ORIGIN: usize = 0x40010800;
const GPIOA_CRL: usize = GPIOA_ORIGIN;
const GPIOA_CRH: usize = GPIOA_ORIGIN + 0x04;
const GPIOA_BSRR: usize = GPIOA_ORIGIN + 0x10;

const GPIOB_ORIGIN: usize = 0x40010C00;
const GPIOB_CRL: usize = GPIOB_ORIGIN;
const GPIOB_CRH: usize = GPIOB_ORIGIN + 0x04;
const GPIOB_BSRR: usize = GPIOB_ORIGIN + 0x10;

const GPIOC_ORIGIN: usize = 0x40011000;
const GPIOC_CRL: usize = GPIOC_ORIGIN;
const GPIOC_CRH: usize = GPIOC_ORIGIN + 0x04;
const GPIOC_BSRR: usize = GPIOC_ORIGIN + 0x10;

pub trait RegisterAware {
    const CR: Register;
    const BSRR: Register;
}

pub struct PortALow {}
pub struct PortAHigh {}
pub struct PortBLow {}
pub struct PortBHigh {}
pub struct PortCLow {}
pub struct PortCHigh {}

impl RegisterAware for PortALow {
    const CR: Register = Register(GPIOA_CRL as *const usize);
    const BSRR: Register = Register(GPIOA_BSRR as *const usize);
}
impl RegisterAware for PortAHigh {
    const CR: Register = Register(GPIOC_CRH as *const usize);
    const BSRR: Register = Register(GPIOA_BSRR as *const usize);
}
impl RegisterAware for PortBLow {
    const CR: Register = Register(GPIOB_CRL as *const usize);
    const BSRR: Register = Register(GPIOB_BSRR as *const usize);
}
impl RegisterAware for PortBHigh {
    const CR: Register = Register(GPIOB_CRH as *const usize);
    const BSRR: Register = Register(GPIOB_BSRR as *const usize);
}
impl RegisterAware for PortCLow {
    const CR: Register = Register(GPIOC_CRL as *const usize);
    const BSRR: Register = Register(GPIOC_BSRR as *const usize);
}
impl RegisterAware for PortCHigh {
    const CR: Register = Register(GPIOC_CRH as *const usize);
    const BSRR: Register = Register(GPIOC_BSRR as *const usize);
}

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
    _marker_port: PhantomData<PORT>,
}

impl<MODE, PORT, const INDEX: u8> Pin<MODE, PORT, INDEX>
where
    PORT: RegisterAware,
{
    const fn new() -> Self {
        Self {
            _marker_mode: PhantomData,
            _marker_port: PhantomData,
        }
    }

    pub fn into_push_pull_output(self) -> Pin<Output, PORT, INDEX> {
        const CNF: u32 = 0b00;
        // default speed 2MHZ
        const MODE: u32 = 0b10;
        // bits to set
        const BITS: u32 = (CNF << 2) | MODE;

        let cr_offset: u32 = (4 * INDEX as u32) % 32;

        // reset pin
        <PORT as RegisterAware>::BSRR.write(1 << (16 + INDEX));

        // clear previous configuration for this pin. then set the new one
        <PORT as RegisterAware>::CR
            .write(<PORT as RegisterAware>::CR.bits() & !(0b1111 << cr_offset) | BITS << cr_offset);

        Pin::new()
    }

    pub fn into_intput(self) -> Pin<Input, PORT, INDEX> {
        Pin::new()
    }
}

impl<PORT, const INDEX: u8> OutputPin for Pin<Output, PORT, INDEX>
where
    PORT: RegisterAware,
{
    fn set_high(&mut self) {
        <PORT as RegisterAware>::BSRR.write(1 << INDEX);
    }

    fn set_low(&mut self) {
        <PORT as RegisterAware>::BSRR.write(1 << (16 + INDEX));
    }
}

// This macro creates a single GPIO struct with all pins from all ports
// Pins are constructed with MCU's default configuration
macro_rules! gpio {
    ( $( ($letter:tt $port:ty [ $($num:tt)+ ]) )* ) => {
        paste! {
            pub struct GPIO {
                $(
                    $(
                        pub [<p $letter $num>]: Pin<Input, $port, $num>,
                    )*
                )*
            }
        }

        paste! {
            impl GPIO {
                pub(in crate::peripheral) const fn new() -> GPIO {
                    GPIO {
                        $(
                            $(
                                [<p $letter $num>]: Pin::new(),
                            )*
                        )*
                    }
                }
            }
        }
    }
}

gpio! {
    (a PortALow [0 1 2 3 4 5 6 7])
    (a PortAHigh [8 9 10 11 12 13 14 15])
    (b PortBLow [0 1 2 3 4 5 6 7])
    (b PortBHigh [8 9 10 11 12 13 14 15])
    (c PortCLow [0 1 2 3 4 5 6 7])
    (c PortCHigh [8 9 10 11 12 13 14 15])
}
