use embassy_rp::gpio::Input;

// * <'a> is an argument about what lifetime does the struct have
// TODO: Figure out a way to get if the encoder is turning
pub struct KeyInputs<'a> {
    pub key1: Input<'a>,
    pub key2: Input<'a>,
    pub key3: Input<'a>,
    pub key4: Input<'a>,
    pub key5: Input<'a>,
    pub key6: Input<'a>,
    pub key7: Input<'a>,
    pub key8: Input<'a>,
    pub enc_a: Input<'a>,
    pub enc_b: Input<'a>,
    pub enc_sw: Input<'a>,
}
impl<'a> KeyInputs<'a> {
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
        enc_b: Input<'a>,
        enc_sw: Input<'a>,
    ) -> Self {
        KeyInputs {
            key1: key1,
            key2: key2,
            key3: key3,
            key4: key4,
            key5: key5,
            key6: key6,
            key7: key7,
            key8: key8,
            enc_a: enc_a,
            enc_b: enc_b,
            enc_sw: enc_sw,
        }
    }
}
