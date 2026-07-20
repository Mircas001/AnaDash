use crate::hardware::input_handler::KeyInputs;
use core::{array, future};
use embassy_futures::select::{Either, select, select_array};
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::peripherals::USB;
use embassy_rp::pio_programs::rotary_encoder::Direction;
use embassy_rp::usb::Driver;
use embassy_time::{Duration, Timer};
use embassy_usb::class::hid::HidWriter;
use usbd_hid::descriptor::{KeyboardReport, KeyboardUsage};

const DEBOUNCE_TIME: Duration = Duration::from_millis(10);

const KEYCODES: [u8; 8] = [
    KeyboardUsage::KeyboardF13 as u8,
    KeyboardUsage::KeyboardF14 as u8,
    KeyboardUsage::KeyboardF15 as u8,
    KeyboardUsage::KeyboardF16 as u8,
    KeyboardUsage::KeyboardF18 as u8,
    KeyboardUsage::KeyboardF19 as u8,
    KeyboardUsage::KeyboardF20 as u8,
    KeyboardUsage::KeyboardF21 as u8,
];

#[embassy_executor::task]
pub async fn input_task(
    macro_w: HidWriter<'static, Driver<'static, USB>, 8>,
    media_w: HidWriter<'static, Driver<'static, USB>, 8>,
    mut inputs: KeyInputs<'static>,
) {
    loop {
        let mut key_pressed_fut = select_array([
            inputs.key1.wait_for_falling_edge(),
            inputs.key2.wait_for_falling_edge(),
            inputs.key3.wait_for_falling_edge(),
            inputs.key4.wait_for_falling_edge(),
            inputs.key5.wait_for_falling_edge(),
            inputs.key6.wait_for_falling_edge(),
            inputs.key7.wait_for_falling_edge(),
            inputs.key8.wait_for_falling_edge(),
            inputs.enc_sw.wait_for_falling_edge(),
        ]); // * We wait for any falling edge, or basically, it going LOW
        let encoder_turned = future::pending::<Direction>(); // * Encoder not implemented, this is an placeholder

        match select(key_pressed_fut, encoder_turned).await {
            Either::First(_) => {
                Timer::after(DEBOUNCE_TIME).await; // * We wait a while so we get an stable signal of LOW
                let is_k_pressed: u8 = (inputs.key1.is_low() as u8) << 0 // * We save up 56 bits/7 bytes this way! But i should maybe think of implementing as a custom type
                    | (inputs.key2.is_low() as u8) << 1
                    | (inputs.key3.is_low() as u8) << 2
                    | (inputs.key4.is_low() as u8) << 3
                    | (inputs.key5.is_low() as u8) << 4
                    | (inputs.key6.is_low() as u8) << 5
                    | (inputs.key7.is_low() as u8) << 6
                    | (inputs.key8.is_low() as u8) << 7;

                let is_encoder_pressed: bool = inputs.enc_sw.is_low();
                {
                    // * Putting it on it's own scope so it can be quickly dropped
                    let mut keycodes: [u8; 6] = [0; 6];

                    let mut idx: usize = 0;
                    'outer: for bit in 0..8 {
                        // * Special way of iterating to accomodate the memory saving
                        if is_k_pressed & (1 << bit) != 0 {
                            if idx < keycodes.len() {
                                keycodes[idx] = KEYCODES[bit];
                                idx += 1;
                            } else {
                                break 'outer; // * Yes, it stops after 6
                            }
                        }
                    }
                    let report_keys = KeyboardReport {
                        modifier: 0,
                        reserved: 0,
                        leds: 0,
                        keycodes: keycodes,
                    };
                }
            }
            Either::Second(direction) => {
                unreachable!();
                // * This is an placeholder for now
            }
        }
    }
}
