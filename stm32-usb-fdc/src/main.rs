#![no_std]
#![no_main]

use panic_abort as _;
use stm32f4xx_hal::gpio::{GpioExt, Output, Pin, PinState, PushPull};
use stm32f4xx_hal::pac;
use stm32f4xx_hal::rcc::RccExt;

#[cortex_m_rt::entry]
fn main() -> ! {
    // take core peripherals
    let _cp = cortex_m::Peripherals::take().unwrap();
    // take device-specific peripherals
    let dp = pac::Peripherals::take().unwrap();

    // setup clocks
    let rcc = dp.RCC.constrain();
    let _clocks = rcc.cfgr.freeze();

    // configure GPIO
    let gpio_a = dp.GPIOA.split();
    let gpio_b = dp.GPIOB.split();
    let gpio_c = dp.GPIOC.split();

    let mut pin_dbg_1 = gpio_a.pa15.into_push_pull_output_in_state(PinState::Low);
    let pin_dbg_btn = gpio_b.pb8.into_pull_down_input();

    cortex_m::asm::delay(10_000_000);

    let mut pin_drive_select_b = gpio_b.pb0.into_push_pull_output_in_state(PinState::High); // not select
    let mut pin_motor_en_b = gpio_a.pa6.into_push_pull_output_in_state(PinState::High); // not spin
    let pin_ready = gpio_c.pc13.into_pull_up_input();
    let _pin_index = gpio_b.pb2.into_pull_up_input();
    // let pin_track_zero = gpio_a.pa1.into_pull_down_input();
    // let mut pin_head_step = gpio_a.pa4.into_push_pull_output_in_state(PinState::Low);
    // let mut pin_dir_select = gpio_a.pa5.into_push_pull_output_in_state(PinState::High); // out
    // let pin_write_protect = gpio_a.pa0.into_pull_down_input();

    cortex_m::asm::delay(50_000_000);

    loop {
        if pin_dbg_btn.is_high() {
            pin_drive_select_b.set_low(); // select
            pin_motor_en_b.set_low(); // start spin
        } else {
            pin_drive_select_b.set_high(); // not select
            pin_motor_en_b.set_high(); // stop spin
        }

        if pin_ready.is_low() {
            pin_dbg_1.set_high();
        } else {
            pin_dbg_1.set_low();
        }
    }
}

fn head_step(pin: &mut Pin<Output<PushPull>, 'A', 4>) {
    pin.toggle();
    cortex_m::asm::delay(1_000_000);
    pin.toggle();
}
