#![feature(const_generics)]
#![feature(const_fn)]
#![no_std]
#![no_main]

use core::mem::replace;
use core::panic::PanicInfo;
use crate::peripheral::gpio::OutputPin;

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
    let mut peripheral = peripheral::take();
    peripheral.RCC.enable_io_c_clock();

    let mut gpio = peripheral.GPIO;
    let mut pc13 = gpio.pc13.into_push_pull_output(gpio.crh);
    let mut pc13 = pc13.into_intput();
    let mut pc13 = pc13.into_push_pull_output(gpio.crh);

    loop {
        pc13.set_low();

        busy_wait(100000);
        pc13.set_high();

        busy_wait(100000);
    }
}

fn busy_wait(loops: u32) {
    let mut i = 0;
    while i < loops {
        i += 1;
    }
}
