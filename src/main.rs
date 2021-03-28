#![no_std]
#![no_main]

use core::mem::replace;
use core::panic::PanicInfo;

mod peripheral;
mod startup;

const GPIOC_CRH: *mut usize = 0x40011004 as *mut usize;
const GPIOC_ODR: *mut usize = 0x4001100C as *mut usize;

const GPIOC13: usize = 1 << 13;

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn main() -> ! {
    let mut rcc = unsafe { peripheral::PERIPHERALS.take_rcc() };
    rcc.io_a_clk_enable().io_b_clk_enable().io_c_clk_enable();

    unsafe {
        *GPIOC_CRH &= 0xFF0FFFFF;
        *GPIOC_CRH |= 0x00200000;

        *GPIOC_ODR &= !GPIOC13;
    }
    loop {
        unsafe {
            *GPIOC_ODR |= GPIOC13;
        }
        busy_wait(100000);
        unsafe {
            *GPIOC_ODR &= !GPIOC13;
        }
        busy_wait(100000);
    }
}

fn busy_wait(loops: u32) {
    let mut i = 0;
    while i < loops {
        i += 1;
    }
}
