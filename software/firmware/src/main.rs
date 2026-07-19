#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::InterruptHandler as UsbIrqs;
use {defmt_rtt as _, panic_probe as _};

mod usb_handler;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => UsbIrqs<USB>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    info!("Hello!");

    usb_handler::begin_usb_handler(&_spawner, p.USB);

    loop {}
}
