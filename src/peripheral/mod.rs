mod rcc;

use crate::peripheral::rcc::Rcc;
use core::mem::replace;
use core::ptr;

pub(crate) static mut PERIPHERALS: Peripherals = Peripherals::new();

/// Wrapper around a memory location
struct Register(*const usize);

impl Register {
    /// Combines the "val" with the existing value using OR
    fn or(&mut self, val: usize) {
        unsafe { *(self.0 as *mut usize) |= val }
    }

    /// Completely rewrites the register
    fn write(&self, val: usize) {
        unsafe { *(self.0 as *mut usize) &= val }
    }
}

/// Wrapper around all controller's peripherals
pub(crate) struct Peripherals {
    rcc: Option<Rcc>,
}

impl Peripherals {
    const fn new() -> Peripherals {
        Peripherals {
            rcc: Some(Rcc::new()),
        }
    }

    pub(crate) fn take_rcc(&mut self) -> Rcc {
        let rcc_op = replace(&mut self.rcc, None);
        rcc_op.unwrap()
    }
}
