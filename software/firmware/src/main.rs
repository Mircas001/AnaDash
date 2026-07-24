#![no_std]
#![no_main]

use crate::usb_handler::CDC_CHANNEL;
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::{PIO0, USB};
use embassy_rp::pio::InterruptHandler;
use embassy_rp::usb::InterruptHandler as UsbIrqs;
use shared::HostTransmission;

use {defmt as _, panic_probe as _};

mod hardware;
mod usb_handler;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => UsbIrqs<USB>;
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let hardware = hardware::Hardware::default();

    info!("Hello!");

    usb_handler::begin_usb_handler(&_spawner, hardware.usb, hardware.inputs);

    loop {
        let incoming_data = CDC_CHANNEL.receive().await;
        match incoming_data {
            HostTransmission::Notification(noti) => {
                // * blank for now
            }
            HostTransmission::Dashboard(dash) => {}
        }
    }
}
