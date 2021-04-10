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

    let mut debug_led_1 = gpio.pb8.into_push_pull_output();
    let mut debug_led_2 = gpio.pb7.into_push_pull_output();
    let mut debug_led_3 = gpio.pb6.into_push_pull_output();

    let mut drive_select_b = gpio.pa0.into_push_pull_output();
    let mut mot_enable_b = gpio.pa2.into_push_pull_output();

    let mut index = gpio.pc14.into_pull_up_input();

    drive_select_b.set_low(); //turn led on
    mot_enable_b.set_low(); // spin motor

    loop {
        if index.is_high() {
            debug_led_1.set_low();
        } else {
            debug_led_1.set_high();
        }
    }
}

fn busy_wait(loops: u32) {
    let mut i = 0;
    while i < loops {
        i += 1;
    }
}
