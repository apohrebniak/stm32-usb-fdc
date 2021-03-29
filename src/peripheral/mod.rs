mod rcc;

use crate::peripheral::rcc::Rcc;
use core::mem::replace;
use core::ptr;

static mut PERIPHERAL: Option<Peripheral> = Some(Peripheral::new());

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
#[allow(non_snake_case)]
pub(crate) struct Peripheral {
    pub(crate) RCC: Rcc,
}

impl Peripheral {
    const fn new() -> Peripheral {
        Peripheral { RCC: Rcc::new() }
    }
}

pub(crate) fn take() -> Peripheral {
    let p = unsafe { replace(&mut PERIPHERAL, None) };
    p.unwrap()
}
