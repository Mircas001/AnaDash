#![no_std]
#![no_main]

use core::error;

use crate::usb_handler::CDC_CHANNEL;
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::dma;
use embassy_rp::i2c::InterruptHandler as i2cIrqs;
use embassy_rp::peripherals::{DMA_CH0, DMA_CH1, DMA_CH2, I2C1, PIO0, UART0, USB};
use embassy_rp::pio::{InterruptHandler as PioIrqs, Pio};
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};
use embassy_rp::uart::InterruptHandler as UARTInterruptHandler;
use embassy_rp::usb::InterruptHandler as UsbIrqs;
use embassy_time::{Delay, Duration, Instant, Ticker};
use embassy_usb::class::dfu::consts::Status::Ok;
use embedded_hal_bus::spi::ExclusiveDevice;
use mcp4728::MCP4728Async;
use shared::HostTransmission;
use smart_leds::RGB8;
use static_cell::StaticCell;
use {defmt as _, panic_probe as _};

mod display;
mod hardware;
mod usb_handler;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => UsbIrqs<USB>;
    I2C1_IRQ => i2cIrqs<I2C1>;
    UART0_IRQ => UARTInterruptHandler<UART0>;
    PIO0_IRQ_0 => PioIrqs<PIO0>;
    DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>;
});

static SERIAL: StaticCell<embassy_rp::uart::Uart<'static, embassy_rp::uart::Blocking>> =
    StaticCell::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut hardware = hardware::Hardware::default();

    let uart = SERIAL.init(hardware.uart);

    defmt_serial::defmt_serial(uart);

    info!("Hello!");

    let mut meters = MCP4728Async::new(&mut hardware.i2c, 0x60);
    if let Err(e) = meters.fast_write(4096, 4096, 4096, 4096).await {
        warn!("Failed to write to meters! {}", e);
    }
    hardware.ldac.set_high();

    usb_handler::begin_usb_handler(&_spawner, hardware.usb, hardware.inputs);

    hardware.ldac.set_low();

    let display_spi = ExclusiveDevice::new(hardware.spi, hardware.cs, Delay).unwrap();

    let mut display = display::Display::new(display_spi, hardware.rst, hardware.dc);

    loop {
        let incoming_data = CDC_CHANNEL.receive().await;
        match incoming_data {
            HostTransmission::Notification(noti) => {
                display.show_notification(noti);
            }
            HostTransmission::Dashboard(dash) => {
                if let Err(e) = meters
                    .fast_write(dash.cpu_load, dash.cpu_temp, dash.mem_used, dash.swap_used)
                    .await
                {
                    warn!("Failed to write to meters! {}", e);
                }
                hardware.ldac.set_high();
                display.update_screen(dash);
            }
        }
    }
}
