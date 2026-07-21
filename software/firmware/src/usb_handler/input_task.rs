use crate::hardware::input_handler::KeyInputs;
use cortex_m::peripheral::SCB;
use defmt::warn;
use embassy_futures::select::{Either, select, select_array};
use embassy_rp::peripherals::USB;
use embassy_rp::rom_data::reset_to_usb_boot;
use embassy_rp::usb::Driver;
use embassy_time::{Duration, Timer};
use embassy_usb::class::hid::HidWriter;
use rotary_encoder_hal::Rotary;
use usbd_hid::descriptor::{KeyboardReport, KeyboardUsage, MediaKey, MediaKeyboardReport};

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
const MUTE_REPORT: MediaKeyboardReport = MediaKeyboardReport {
    usage_id: MediaKey::Mute as u16,
};
const VOLUME_UP_REPORT: MediaKeyboardReport = MediaKeyboardReport {
    usage_id: MediaKey::VolumeIncrement as u16,
};
const VOLUME_DOWN_REPORT: MediaKeyboardReport = MediaKeyboardReport {
    usage_id: MediaKey::VolumeDecrement as u16,
};
const RESET_COMBO: u8 = 0b1000_0001;
const BOOT_COMBO: u8 = 0b0001_1000;

#[embassy_executor::task]
pub async fn input_task(
    mut macro_w: HidWriter<'static, Driver<'static, USB>, 8>,
    mut media_w: HidWriter<'static, Driver<'static, USB>, 8>,
    mut inputs: KeyInputs<'static>,
) {
    let mut enc = Rotary::new(inputs.enc_a, inputs.enc_b);

    loop {
        let key_pressed_fut = select_array([
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

        let (enc_a, enc_b) = enc.pins();
        let encoder_turned =
            select_array([enc_a.wait_for_falling_edge(), enc_b.wait_for_falling_edge()]);

        match select(key_pressed_fut, encoder_turned).await {
            Either::First(_) => {
                Timer::after(DEBOUNCE_TIME).await; // * We wait a while so we get an stable signal of LOW
                {
                    // * Putting it on it's own scope so it can be quickly dropped
                    let is_encoder_pressed: bool = inputs.enc_sw.is_low();
                    if is_encoder_pressed {
                        match media_w.write_serialize(&MUTE_REPORT).await {
                            Ok(()) => continue, // * There's no reason for you to press the encoder switch and another key
                            Err(e) => warn!("Error sending media report!: {}", e),
                        }
                        continue; // * just reinforcing the previous point
                    }
                }
                {
                    // * there's no reason for this to be in scope, the values are getting dropped as soon as it's over anyway,
                    // * i just did that because it would be ugly
                    let is_k_pressed: u8 = (inputs.key1.is_low() as u8) << 0 // * We save up 56 bits/7 bytes this way! But i should maybe think of implementing as a custom type
                        | (inputs.key2.is_low() as u8) << 1
                        | (inputs.key3.is_low() as u8) << 2
                        | (inputs.key4.is_low() as u8) << 3
                        | (inputs.key5.is_low() as u8) << 4
                        | (inputs.key6.is_low() as u8) << 5
                        | (inputs.key7.is_low() as u8) << 6
                        | (inputs.key8.is_low() as u8) << 7;

                    if is_k_pressed & RESET_COMBO == RESET_COMBO {
                        SCB::sys_reset();
                    } // * lowk wish there was a way that didnt involve it being inside the input_task
                    if is_k_pressed & BOOT_COMBO == BOOT_COMBO {
                        reset_to_usb_boot(0, 0);
                    }

                    let mut keycodes: [u8; 6] = [0; 6];

                    let mut idx: usize = 0;
                    'outer: for bit in 0..8 {
                        // * Special way of iterating to accomodate the memory saving
                        if is_k_pressed & (1 << bit) != 0 {
                            // * fancy way of saying: is the key pressed?
                            if idx < keycodes.len() {
                                keycodes[idx] = KEYCODES[bit];
                                idx += 1;
                            } else {
                                break 'outer; // * Yes, it stops after 6, for safety.
                            }
                        }
                    }

                    let keys_report = KeyboardReport {
                        modifier: 0,
                        reserved: 0,
                        leds: 0,
                        keycodes: keycodes,
                    };
                    match macro_w.write_serialize(&keys_report).await {
                        Ok(()) => {}
                        Err(e) => warn!("Error sending macro report!: {} :(", e),
                    }
                }
            }
            Either::Second(_) => {
                let direction = enc.update().unwrap();
                let encoder_report: MediaKeyboardReport;
                match direction {
                    rotary_encoder_hal::Direction::Clockwise => encoder_report = VOLUME_UP_REPORT,
                    rotary_encoder_hal::Direction::CounterClockwise => {
                        encoder_report = VOLUME_DOWN_REPORT // * VS CODE WONT LET ME TAKE OUT OF THE CURLY BRACES
                    }
                    rotary_encoder_hal::Direction::None => continue,
                };
                match media_w.write_serialize(&encoder_report).await {
                    Ok(()) => {}
                    Err(e) => warn!("Error sending encoder report! {}", e),
                }
                // * This is an placeholder for now
            }
        }
    }
}
