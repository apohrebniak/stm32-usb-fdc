#![feature(const_generics)]
#![feature(const_fn)]
#![no_std]
#![no_main]

use crate::peripheral::gpio::{InputPin, OutputPin, OutputSpeed};
use crate::peripheral::Register;
use core::mem::replace;
use core::panic::PanicInfo;

mod peripheral;
mod startup;

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn main() -> ! {
    let mut peripheral = peripheral::take();
    peripheral.RCC.enable_io_a_clock();
    peripheral.RCC.enable_io_b_clock();
    peripheral.RCC.enable_io_c_clock();

    let mut gpio = peripheral.GPIO;
    let mut pa0 = gpio.pa0.into_push_pull_output();
    let mut pa2 = gpio.pa2.into_push_pull_output();
    let mut pa5 = gpio.pa5.into_pull_down_input();
    let mut pc13 = gpio.pc13.into_push_pull_output();

    let mut turn_on = true;

    pa2.set_speed(OutputSpeed::Speed50MHz);
    pa0.set_speed(OutputSpeed::Speed10MHz);
    pc13.set_speed(OutputSpeed::Speed2MHz);

    loop {
        if pa5.is_high() {
            if turn_on {
                pa2.set_high()
            } else {
                pa2.set_low()
            };
            busy_wait(100000);

            if turn_on {
                pa0.set_high()
            } else {
                pa0.set_low()
            };
            busy_wait(100000);

            if turn_on {
                pc13.set_high()
            } else {
                pc13.set_low()
            };
            busy_wait(100000);

            turn_on = !turn_on;
        }
    }
}

fn busy_wait(loops: u32) {
    let mut i = 0;
    while i < loops {
        i += 1;
    }
}
