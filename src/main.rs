#![feature(const_generics)]
#![feature(const_fn)]
#![no_std]
#![no_main]

use crate::peripheral::gpio::{InputPin, OutputPin, OutputSpeed};
use crate::peripheral::Register;
use core::mem::replace;
use core::panic::PanicInfo;
use core::ptr;

mod peripheral;
mod startup;


const USART1_ORIGIN: usize = 0x40013800;

const USART1_SR: *mut usize = (USART1_ORIGIN) as *mut usize;
const USART1_DR: *mut usize = (USART1_ORIGIN + 0x04) as *mut usize;
const USART1_BRR: *mut usize = (USART1_ORIGIN + 0x08) as *mut usize;
const USART1_CR1: *mut usize = (USART1_ORIGIN + 0x0C) as *mut usize;
const USART1_CR2: *mut usize = (USART1_ORIGIN + 0x10) as *mut usize;
const USART1_CR3: *mut usize = (USART1_ORIGIN + 0x14) as *mut usize;

//TXE bit 7
//UE bit 13
//M bit 12
//TE bit 3

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn main() -> ! {
    let mut peripheral = peripheral::take();
    peripheral.RCC
        .enable_io_a_clock()
        .enable_io_b_clock()
        .enable_io_c_clock()
        .enable_usart_1_clock() //enable usart1 clock
        .enable_io_alt_clock(); //enable alternate function clock

    let mut msg: usize = 0;

    let mut gpio = peripheral.GPIO;
    let mut tx_pin = gpio.pa9.into_alt_push_pull_output();
    tx_pin.set_speed(OutputSpeed::Speed50MHz);
    let mut rx_pin = gpio.pa10.into_floating_input();

    let mut debug_led = gpio.pb8.into_push_pull_output();

    //skip rcc reset
    //clock 8MHz; baud 300; parity disabled; 1 stop bit
    unsafe {
        *USART1_CR1 |= (1 << 13); //enable uart
        *USART1_CR1 |= (1 << 12); //9 bits
        *USART1_CR1 |= (1 << 10); //even partity
        *USART1_BRR |= ((8000 / 300) as u16) as usize;//set speed in brr
        *USART1_CR1 |= (1 << 3); //TE in CR1; enable transmit
        *USART1_CR1 |= (1 << 2); //RE in CR1; enable receive
    }

    loop {
        //wait while TXE not 1
        //write byte
        unsafe {
            while *USART1_SR & (0b1 << 5) == 0 {}
            msg = ptr::read(USART1_DR as *const usize);
            if msg > 0 {
                debug_led.set_high();
                busy_wait(10000);
            }
            *USART1_DR = 0x78 ;
            while *USART1_SR & (0b1 << 6) == 0 {}
            debug_led.set_low();
        }
    }

}

fn busy_wait(loops: u32) {
    let mut i = 0;
    while i < loops {
        i += 1;
    }
}
