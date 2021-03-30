mod gpio;
mod rcc;

use crate::peripheral::gpio::GPIOC;
use crate::peripheral::rcc::Rcc;
use core::mem::replace;
use core::ptr;

static mut PERIPHERAL: Option<Peripheral> = Some(Peripheral::new());

/// Wrapper around a memory location
pub(crate) struct Register(*const usize);

impl Register {
    /// Combines the "val" with the existing value using OR
    fn or(&mut self, val: u32) {
        unsafe { *(self.0 as *mut u32) |= val }
    }

    /// Completely rewrites the register
    fn write(&self, val: u32) {
        unsafe { *(self.0 as *mut u32) = val }
    }

    fn bits(&self) -> u32 {
        unsafe { *self.0 as u32 }
    }
}

/// Wrapper around all controller's peripherals
#[allow(non_snake_case)]
pub(crate) struct Peripheral {
    pub(crate) RCC: Rcc,
    pub(crate) GPIOC: GPIOC,
}

impl Peripheral {
    const fn new() -> Peripheral {
        Peripheral {
            RCC: Rcc::new(),
            GPIOC: GPIOC::new(),
        }
    }
}

pub(crate) fn take() -> Peripheral {
    let p = unsafe { replace(&mut PERIPHERAL, None) };
    p.unwrap()
}
