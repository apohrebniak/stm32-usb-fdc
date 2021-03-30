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
    let mut peripheral = peripheral::take();
    peripheral.RCC.enable_io_c_clock();

    let mut gpioc = peripheral.GPIOC;
    let mut pc13 = gpioc.p13.into_push_pull_output(gpioc.crh);

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
