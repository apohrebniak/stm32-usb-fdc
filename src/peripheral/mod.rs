pub(crate) mod gpio;
mod rcc;

use crate::peripheral::gpio::GPIO;
use crate::peripheral::rcc::Rcc;
use core::mem::replace;
use core::ptr;

static mut PERIPHERAL: Option<Peripheral> = Some(Peripheral::new());

/// Wrapper around a memory location
#[derive(Copy, Clone)]
pub struct Register(*const usize);

impl Register {
    fn write_memory_location(addr: usize, val: u32) {
        unsafe { *(addr as *mut usize) = val as usize }
    }

    fn read_memory_location(addr: usize) -> u32 {
        unsafe { *(addr as *const u32) }
    }

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
pub struct Peripheral {
    pub RCC: Rcc,
    pub GPIO: GPIO,
}

impl Peripheral {
    const fn new() -> Peripheral {
        Peripheral {
            RCC: Rcc::new(),
            GPIO: GPIO::new(),
        }
    }
}

pub fn take() -> Peripheral {
    let p = unsafe { replace(&mut PERIPHERAL, None) };
    p.unwrap()
}
