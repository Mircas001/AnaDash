use crate::Irqs;
use defmt::*;
use embassy_rp::gpio::Level::Low;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::i2c::{self, Config as I2cConfig};
use embassy_rp::peripherals::{I2C1, SPI1, USB};
use embassy_rp::spi::{Blocking as BlockingSpi, Config as SpiConfig, Spi};

pub mod input_handler;

pub struct Hardware {
    pub usb: embassy_rp::Peri<'static, USB>,
    pub inputs: input_handler::KeyInputs<'static>,
    pub i2c: i2c::I2c<'static, I2C1, i2c::Async>,
    pub ldac: Output<'static>,
    pub cs: Output<'static>,
    pub rst: Output<'static>,
    pub dc: Output<'static>,
    pub spi: Spi<'static, SPI1, BlockingSpi>,
}

impl Default for Hardware {
    fn default() -> Self {
        let p = embassy_rp::init(Default::default());

        info!("Starting hardware!");

        let usb = p.USB;

        let sda = p.PIN_2;
        let scl = p.PIN_3;
        let i2c = i2c::I2c::new_async(p.I2C1, scl, sda, Irqs, I2cConfig::default());
        let ldac = Output::new(p.PIN_13, Level::Low);

        let spi = Spi::new_blocking(p.SPI1, p.PIN_14, p.PIN_11, p.PIN_12, SpiConfig::default());
        let cs = Output::new(p.PIN_8, Level::Low);
        let rst = Output::new(p.PIN_9, Level::Low);
        let dc = Output::new(p.PIN_10, Level::Low);

        let enc_a = Input::new(p.PIN_20, Pull::Up);
        let enc_b = Input::new(p.PIN_21, Pull::Up);

        let key_inputs = input_handler::KeyInputs::new(
            Input::new(p.PIN_4, Pull::Up),
            Input::new(p.PIN_5, Pull::Up),
            Input::new(p.PIN_6, Pull::Up),
            Input::new(p.PIN_7, Pull::Up),
            Input::new(p.PIN_16, Pull::Up),
            Input::new(p.PIN_17, Pull::Up),
            Input::new(p.PIN_18, Pull::Up),
            Input::new(p.PIN_19, Pull::Up),
            enc_a,
            enc_b,
            Input::new(p.PIN_22, Pull::Up),
        );
        Self {
            inputs: key_inputs,
            usb: usb,
            i2c: i2c,
            ldac: ldac,
            cs: cs,
            rst: rst,
            dc: dc,
            spi: spi,
        }
    }
}
