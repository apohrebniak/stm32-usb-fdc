#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod startup;

const RCC_APB2ENR: *mut usize = 0x40021018 as *mut usize;
const GPIOC_CRH: *mut usize = 0x40011004 as *mut usize;
const GPIOC_ODR: *mut usize = 0x4001100C as *mut usize;

const RCC_IOPCEN: usize = 1 << 4;
const GPIOC13: usize = 1 << 13;

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn main() -> ! {
    unsafe {
        *RCC_APB2ENR |= RCC_IOPCEN;
        *GPIOC_CRH &= 0xFF0FFFFF;
        *GPIOC_CRH |= 0x00200000;

        // *GPIOC_ODR &= !GPIOC13;
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
