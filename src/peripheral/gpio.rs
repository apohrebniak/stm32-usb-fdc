use crate::peripheral::Register;

const GPIOC_ORIGIN: usize = 0x40011000;
const GPIOC_CRL: usize = GPIOC_ORIGIN;
const GPIOC_CRH: usize = GPIOC_ORIGIN + 0x04;
// const GPIOC_ODR: usize = GPIOC_ORIGIN + 0x0C;
const GPIOC_BSRR: usize = GPIOC_ORIGIN + 0x10;

pub(crate) enum State {
    Low,
    High,
}

pub(crate) struct Pc13 {}

impl Pc13 {
    pub(crate) fn set_low(&mut self) {
        self.set_state(State::Low);
    }

    pub(crate) fn set_high(&mut self) {
        self.set_state(State::High);
    }

    pub(crate) fn set_state(&mut self, state: State) {
        match state {
            State::Low => GPIOC::bsrr().write(1 << (16 + 13)),
            State::High => GPIOC::bsrr().write(1 << 13),
        }
    }

    // pub(crate)fn read_state(&self) -> State {
    //
    // }

    pub(crate) fn into_push_pull_output(self, cr: Register) -> Pc13 {
        const CR_OFFSET: u32 = (4 * 13) % 32;
        const CNF: u32 = 0b00;
        // default speed 2MHZ
        const MODE: u32 = 0b10;
        // bits to set
        const BITS: u32 = (CNF << 2) | MODE;

        // reset pin
        GPIOC::bsrr().write(1 << (16 + 13));

        // clear previous configuration for this pin. then set the new one
        cr.write(cr.bits() & !(0b1111 << CR_OFFSET) | BITS << CR_OFFSET);

        Pc13 {}
    }

    // pub(crate)fn set_speed(self) -> Pc13 {
    //
    // }
}

pub(crate) struct GPIOC {
    pub(crate) crl: Register,
    pub(crate) crh: Register,
    pub(crate) p13: Pc13,
}

impl GPIOC {
    pub(in crate::peripheral) const fn new() -> GPIOC {
        GPIOC {
            crl: Register(GPIOC_CRL as *const usize),
            crh: Register(GPIOC_CRH as *const usize),
            p13: Pc13 {},
        }
    }
}

impl GPIOC {
    const fn bsrr() -> Register {
        Register(GPIOC_BSRR as *const usize)
    }

    // const fn idr() -> Register {
    //
    // }
}
