#![no_std]
#![no_main]

use panic_abort as _;
use stm32f4xx_hal::gpio::GpioExt;
use stm32f4xx_hal::otg_fs::UsbBus;
use stm32f4xx_hal::pac;
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::rcc::RccExt;
use usb_device::prelude::*;

static mut USB_EP_MEMORY: [u32; 1024] = [0u32; 1024];

#[cortex_m_rt::entry]
fn main() -> ! {
    // take core peripherals
    let cp = cortex_m::Peripherals::take().unwrap();
    // take device-specific peripherals
    let dp = pac::Peripherals::take().unwrap();

    // setup clocks
    let rcc = dp.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(25.MHz()) // 25Mhz HSE is present on the board
        .sysclk(48.MHz())
        .require_pll48clk()
        .freeze();

    // setup GPIO
    let gpioa = dp.GPIOA.split();
    let mut pin_usb_dm = gpioa.pa11.into_push_pull_output();
    let mut pin_usb_dp = gpioa.pa12.into_push_pull_output();

    // force D+ for 100ms
    // this will force the host to enumerate devices
    pin_usb_dm.set_low();
    pin_usb_dp.set_low();
    cp.SYST.delay(&clocks).delay_ms(100u32);

    let usb_peripheral = stm32f4xx_hal::otg_fs::USB {
        usb_global: dp.OTG_FS_GLOBAL,
        usb_device: dp.OTG_FS_DEVICE,
        usb_pwrclk: dp.OTG_FS_PWRCLK,
        pin_dm: pin_usb_dm.into_alternate(),
        pin_dp: pin_usb_dp.into_alternate(),
        hclk: clocks.hclk(),
    };

    let usb_bus = UsbBus::new(usb_peripheral, unsafe { &mut USB_EP_MEMORY });

    let mut usb_serial = usbd_serial::SerialPort::new(&usb_bus);

    let mut usb_device = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0xabcd, 0xabcd))
        .manufacturer("Foo Bar")
        .product("STM32 USB Floppy")
        .build();

    loop {
        if !usb_device.poll(&mut [&mut usb_serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match usb_serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                // Echo back in upper case
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                let mut write_offset = 0;
                while write_offset < count {
                    match usb_serial.write(&buf[write_offset..count]) {
                        Ok(len) if len > 0 => {
                            write_offset += len;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
