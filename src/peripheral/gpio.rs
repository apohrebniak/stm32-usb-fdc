use crate::peripheral::Register;
use core::marker::PhantomData;
use paste::paste;

const GPIOA_ORIGIN: usize = 0x40010800;
const GPIOA_CRL: usize = GPIOA_ORIGIN;
const GPIOA_CRH: usize = GPIOA_ORIGIN + 0x04;
const GPIOA_BSRR: usize = GPIOA_ORIGIN + 0x10;
const GPIOA_IDR: usize = GPIOA_ORIGIN + 0x08;

const GPIOB_ORIGIN: usize = 0x40010C00;
const GPIOB_CRL: usize = GPIOB_ORIGIN;
const GPIOB_CRH: usize = GPIOB_ORIGIN + 0x04;
const GPIOB_BSRR: usize = GPIOB_ORIGIN + 0x10;
const GPIOB_IDR: usize = GPIOB_ORIGIN + 0x08;

const GPIOC_ORIGIN: usize = 0x40011000;
const GPIOC_CRL: usize = GPIOC_ORIGIN;
const GPIOC_CRH: usize = GPIOC_ORIGIN + 0x04;
const GPIOC_BSRR: usize = GPIOC_ORIGIN + 0x10;
const GPIOC_IDR: usize = GPIOC_ORIGIN + 0x08;

/// Maximum output speed
pub enum OutputSpeed {
    Speed2MHz = 0b10,
    Speed10MHz = 0b01,
    Speed50MHz = 0b11,
}

pub trait RegisterAware {
    const CR: Register;
    const BSRR: Register;
    const IDR: Register;
}

// Marker types to indicate which CRL or CRH this pin corresponds to
pub struct PortALow {}
pub struct PortAHigh {}
pub struct PortBLow {}
pub struct PortBHigh {}
pub struct PortCLow {}
pub struct PortCHigh {}

impl RegisterAware for PortALow {
    const CR: Register = Register(GPIOA_CRL as *const usize);
    const BSRR: Register = Register(GPIOA_BSRR as *const usize);
    const IDR: Register = Register(GPIOA_IDR as *const usize);
}
impl RegisterAware for PortAHigh {
    const CR: Register = Register(GPIOA_CRH as *const usize);
    const BSRR: Register = Register(GPIOA_BSRR as *const usize);
    const IDR: Register = Register(GPIOA_IDR as *const usize);
}

impl RegisterAware for PortBLow {
    const CR: Register = Register(GPIOB_CRL as *const usize);
    const BSRR: Register = Register(GPIOB_BSRR as *const usize);
    const IDR: Register = Register(GPIOB_IDR as *const usize);
}
impl RegisterAware for PortBHigh {
    const CR: Register = Register(GPIOB_CRH as *const usize);
    const BSRR: Register = Register(GPIOB_BSRR as *const usize);
    const IDR: Register = Register(GPIOB_IDR as *const usize);
}

impl RegisterAware for PortCLow {
    const CR: Register = Register(GPIOC_CRL as *const usize);
    const BSRR: Register = Register(GPIOC_BSRR as *const usize);
    const IDR: Register = Register(GPIOC_IDR as *const usize);
}
impl RegisterAware for PortCHigh {
    const CR: Register = Register(GPIOC_CRH as *const usize);
    const BSRR: Register = Register(GPIOC_BSRR as *const usize);
    const IDR: Register = Register(GPIOC_IDR as *const usize);
}

// Marker types for pins in in/out mode
pub struct Input {}
pub struct Output {}

/// Available actions for input pin
pub trait InputPin {
    fn is_high(&self) -> bool;
    fn is_low(&self) -> bool;
}

/// Available actions for output pin
pub trait OutputPin {
    fn set_high(&mut self);
    fn set_low(&mut self);
    fn set_speed(&mut self, speed: OutputSpeed);
}

/// Pin typed with it's IO mode, control register and index
pub struct Pin<MODE, PORT, const INDEX: u8> {
    _marker_mode: PhantomData<MODE>,
    _marker_port: PhantomData<PORT>,
}

impl<MODE, PORT, const INDEX: u8> Pin<MODE, PORT, { INDEX }>
where
    PORT: RegisterAware,
{
    const fn new() -> Self {
        Self {
            _marker_mode: PhantomData,
            _marker_port: PhantomData,
        }
    }

    pub fn into_push_pull_output(self) -> Pin<Output, PORT, { INDEX }> {
        const CNF: u32 = 0b00;
        // default speed 2MHZ
        const MODE: u32 = 0b10;

        self.configure(CNF, MODE);
        self.reset();

        Pin::new()
    }

    pub fn into_open_drain_output(self) -> Pin<Output, PORT, { INDEX }> {
        const CNF: u32 = 0b01;
        // default speed 2MHZ
        const MODE: u32 = 0b10;

        self.configure(CNF, MODE);
        self.reset();

        Pin::new()
    }

    pub fn into_alt_push_pull_output(self) -> Pin<Output, PORT, { INDEX }> {
        const CNF: u32 = 0b10;
        // default speed 2MHZ
        const MODE: u32 = 0b10;

        self.configure(CNF, MODE);
        self.reset();

        Pin::new()
    }

    pub fn into_alt_open_drain_output(self) -> Pin<Output, PORT, { INDEX }> {
        const CNF: u32 = 0b11;
        // default speed 2MHZ
        const MODE: u32 = 0b10;

        self.configure(CNF, MODE);
        self.reset();

        Pin::new()
    }

    pub fn into_floating_input(self) -> Pin<Input, PORT, { INDEX }> {
        const CNF: u32 = 0b01;
        // default speed 2MHZ
        const MODE: u32 = 0b00;

        self.configure(CNF, MODE);
        self.reset();

        Pin::new()
    }

    pub fn into_pull_down_input(self) -> Pin<Input, PORT, { INDEX }> {
        const CNF: u32 = 0b10;
        // default speed 2MHZ
        const MODE: u32 = 0b00;

        self.configure(CNF, MODE);
        self.reset();

        Pin::new()
    }

    pub fn into_pull_up_input(self) -> Pin<Input, PORT, { INDEX }> {
        const CNF: u32 = 0b10;
        // default speed 2MHZ
        const MODE: u32 = 0b00;

        self.configure(CNF, MODE);
        self.set();

        Pin::new()
    }

    /// Set provided CNF and MODE
    #[inline(always)]
    fn configure(&self, cnf: u32, mode: u32) {
        // bits to set
        let bits: u32 = (cnf << 2) | mode;

        let cr_offset: u32 = (4 * INDEX as u32) % 32;

        // clear previous configuration for this pin. then set the new one
        <PORT as RegisterAware>::CR
            .write(<PORT as RegisterAware>::CR.bits() & !(0b1111 << cr_offset) | bits << cr_offset);
    }

    /// Set BSy as 1
    #[inline(always)]
    fn set(&self) {
        <PORT as RegisterAware>::BSRR.write(1 << (INDEX));
    }

    /// Set BRy as 1
    #[inline(always)]
    fn reset(&self) {
        <PORT as RegisterAware>::BSRR.write(1 << (16 + INDEX));
    }
}

impl<PORT, const INDEX: u8> OutputPin for Pin<Output, PORT, { INDEX }>
where
    PORT: RegisterAware,
{
    fn set_high(&mut self) {
        self.set();
    }

    fn set_low(&mut self) {
        self.reset();
    }

    fn set_speed(&mut self, speed: OutputSpeed) {
        let cr_offset: u32 = (4 * INDEX as u32) % 32;

        // reset pin
        <PORT as RegisterAware>::BSRR.write(1 << (16 + INDEX));

        // clear previous configuration for this pin. then set the new one
        <PORT as RegisterAware>::CR.write(
            <PORT as RegisterAware>::CR.bits() & !(0b11 << cr_offset) | (speed as u32) << cr_offset,
        );
    }
}

impl<PORT, const INDEX: u8> InputPin for Pin<Input, PORT, { INDEX }>
where
    PORT: RegisterAware,
{
    fn is_high(&self) -> bool {
        <PORT as RegisterAware>::IDR.bits() & (0b1 << INDEX) > 0
    }

    fn is_low(&self) -> bool {
        <PORT as RegisterAware>::IDR.bits() & (0b1 << INDEX) == 0
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
