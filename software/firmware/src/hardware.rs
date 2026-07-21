use defmt::*;
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::peripherals::USB;

pub mod input_handler;

pub struct Hardware {
    pub usb: embassy_rp::Peri<'static, USB>,
    pub inputs: input_handler::KeyInputs<'static>,
}

impl Default for Hardware {
    fn default() -> Self {
        let p = embassy_rp::init(Default::default());

        info!("Starting hardware!");

        let usb = p.USB;

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
        }
    }
}
