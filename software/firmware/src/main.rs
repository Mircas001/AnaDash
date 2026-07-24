#![no_std]
#![no_main]

use crate::usb_handler::CDC_CHANNEL;
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::i2c::InterruptHandler as i2cIrqs;
use embassy_rp::peripherals::{I2C1, USB};
use embassy_rp::usb::InterruptHandler as UsbIrqs;
use mcp4728::MCP4728Async;
use shared::HostTransmission;
use {defmt as _, panic_probe as _};

mod hardware;
mod usb_handler;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => UsbIrqs<USB>;
    I2C1_IRQ => i2cIrqs<I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut hardware = hardware::Hardware::default();

    info!("Hello!");

    let mut meters = MCP4728Async::new(&mut hardware.i2c, 0x60);

    usb_handler::begin_usb_handler(&_spawner, hardware.usb, hardware.inputs);

    loop {
        let incoming_data = CDC_CHANNEL.receive().await;
        match incoming_data {
            HostTransmission::Notification(noti) => {
                // * blank for now
            }
            HostTransmission::Dashboard(dash) => {
                meters
                    .fast_write(dash.cpu_load, dash.cpu_temp, dash.mem_used, dash.swap_used)
                    .await
                    .unwrap();
                hardware.ldac.set_high();
            }
        }
    }
}
