use crate::Irqs;
use defmt::*;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::i2c::{self, Config};
use embassy_rp::peripherals::{I2C1, USB};

pub mod input_handler;

pub struct Hardware {
    pub usb: embassy_rp::Peri<'static, USB>,
    pub inputs: input_handler::KeyInputs<'static>,
    pub i2c: i2c::I2c<'static, I2C1, i2c::Async>,
    pub ldac: Output<'static>,
}

impl Default for Hardware {
    fn default() -> Self {
        let p = embassy_rp::init(Default::default());

        info!("Starting hardware!");

        let usb = p.USB;

        let sda = p.PIN_2;
        let scl = p.PIN_3;
        let mut i2c = i2c::I2c::new_async(p.I2C1, scl, sda, Irqs, Config::default());
        let ldac = Output::new(p.PIN_13, Level::Low);

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
        }
    }
}
