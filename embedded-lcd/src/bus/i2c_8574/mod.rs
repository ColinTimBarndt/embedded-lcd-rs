#[cfg(feature = "blocking")]
mod blocking;

use bitflags::bitflags;

use crate::bus::timings::DefaultTimingsI2c;

use super::LcdTimingsI2c;

bitflags! {
    #[derive(Clone, Copy)]
    struct Pins: u8 {
        const REGISTER_SELECT = 0x01;
        const READ = 0x02;
        const ENABLE = 0x04;
        const BACKLIGHT = 0x08;
        const DATA = 0xf0;
    }
}

impl Pins {
    #[inline(always)]
    fn bits_array<const N: usize>(pins: [Pins; N]) -> [u8; N] {
        core::array::from_fn(|i| pins[i].bits())
    }
}

impl Default for Pins {
    fn default() -> Self {
        Self::BACKLIGHT
    }
}

pub struct LcdI2c8574Bus<I, T> {
    i2c: I,
    address: u8,
    state: Pins,
    timings: T,
}

impl<I> LcdI2c8574Bus<I, DefaultTimingsI2c> {
    #[inline]
    pub fn new(i2c: I, address: u8) -> Self {
        Self {
            i2c,
            address,
            state: Pins::default(),
            timings: DefaultTimingsI2c,
        }
    }
}

impl<I, T> LcdI2c8574Bus<I, T> {
    #[inline]
    pub fn new_with_timings<Delay: ?Sized>(i2c: I, address: u8, timings: T) -> Self
    where
        T: LcdTimingsI2c<Delay>,
    {
        Self {
            i2c,
            address,
            state: Pins::default(),
            timings,
        }
    }

    #[inline]
    pub fn destroy(self) -> I {
        self.i2c
    }
}
