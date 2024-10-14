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

impl core::fmt::Debug for Pins {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.debug_struct("Pins")
            .field("register_select", &self.contains(Self::REGISTER_SELECT))
            .field("read", &self.contains(Self::READ))
            .field("enable", &self.contains(Self::ENABLE))
            .field("backlight", &self.contains(Self::BACKLIGHT))
            .field("data", &(self.bits() >> 4))
            .finish()
    }
}

#[cfg(feature = "ufmt")]
impl ufmt::uDebug for Pins {
    fn fmt<W>(&self, fmt: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        fmt.debug_struct("Pins")?
            .field("register_select", &self.contains(Self::REGISTER_SELECT))?
            .field("read", &self.contains(Self::READ))?
            .field("enable", &self.contains(Self::ENABLE))?
            .field("backlight", &self.contains(Self::BACKLIGHT))?
            .field("data", &(self.bits() >> 4))?
            .finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Pins {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "Pins {{ register_select: {}, read: {}, enable: {}, backlight: {}, data: {} }}",
            self.contains(Self::REGISTER_SELECT),
            self.contains(Self::READ),
            self.contains(Self::ENABLE),
            self.contains(Self::BACKLIGHT),
            self.bits() >> 4
        )
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
