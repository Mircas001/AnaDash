// https://github.com/CarlKCarlK/clock/blob/main/src/Input_array.rs
// I'm using his excellent array implementation, so shotout to him

use crate::Result;
use crate::error::Error::IndexOutOfBounds;
use core::num::NonZeroU8;
use embassy_rp::gpio::{self, Level};

pub struct InputArray<'a, const N: usize>([gpio::Input<'a>; N]);

impl<'a, const N: usize> InputArray<'a, N> {
    pub const fn new(Inputs: [gpio::Input<'a>; N]) -> Self {
        Self(Inputs)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &gpio::Input<'a>> {
        self.0.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut gpio::Input<'a>> {
        self.0.iter_mut()
    }

    #[inline]
    pub fn get_level_at_index(&mut self, index: u8, level: Level) -> bool {
        self.get_mut(index as usize) // Mutable access
            .ok_or(IndexOutOfBounds)? // Return error if index is out of bounds
            .is_high(level); // Mutate the item
        Ok(())
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut gpio::Input<'a>> {
        self.0.get_mut(index)
    }
}

impl InputArray<'_, { u8::BITS as usize }> {
    #[expect(clippy::shadow_reuse, reason = "Just converting a NonZeroU8 to a u8.")]
}
