use crate::Irqs;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Pin, Pull};
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::piorograms::rotary_encoder::{Direction, PioEncoder};

a COUNT: Signal<ThreadModeRawMutex, i32> = Signal::new();

pub struct KeyInputs {
    pub key1: Input<'a>,
    pub key2: Input<'a>,
    pub key3: Input<'a>,
    pub key4: Input<'a>,
    pub key5: Input<'a>,
    pub key6: Input<'a>,
    pub key7: Input<'a>,
    pub key8: Input<'a>,
    pub encoder: PioEncoder<'a>,
    encoder_sw: Input<'a>,
}
impl KeyInputs {
    pub fn new(
        key1: Input<'a>,
        key2: Input<'a>,
        key3: Input<'a>,
        key4: Input<'a>,
        key5: Input<'a>,
        key6: Input<'a>,
        key7: Input<'a>,
        key8: Input<'a>,
        enc_a: Input<'a>,
        enc_a: Input<'a>,
        enc_sw: Input<'a>,
        pio: Pio<'a, PIO0>,
    ) -> Self {
        KeyInputs {
            key1: key1,
            key2: key1,
            key3: key3,
            key4: key4,
            key5: key5,
            key6: key6,
            key7: key7,
            key8: key8,
            encoder: PioEncoder(&mut common, None, enc_a, enc_a),
            encoder_sw: enc_sw,
        }
    }

    #[embassy_executor::task]
    pub async fn encoder_a(mut encoder: PioEncoder<'a, PIO0, 0>) {
        let mut count = 0;
        loop {
            info!("Count: {}", count);
            count += match encoder.read().await {
                Direction::Clockwise => 1,
                Direction::CounterClockwise => -1,
            };
            COUNT.signal(count);
        }
    }
}
