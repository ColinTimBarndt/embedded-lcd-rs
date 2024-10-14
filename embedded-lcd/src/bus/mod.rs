#[cfg(feature = "blocking")]
pub mod blocking;

mod i2c_8574;
pub use i2c_8574::*;
mod parallel;
pub use parallel::*;

mod timings;
pub use timings::*;

use embedded_hal::digital::PinState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LcdRegisterSelect {
    Control,
    Memory,
}

impl From<LcdRegisterSelect> for PinState {
    fn from(value: LcdRegisterSelect) -> Self {
        Self::from(value == LcdRegisterSelect::Memory)
    }
}
