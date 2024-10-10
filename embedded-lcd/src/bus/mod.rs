use embedded_hal::digital::PinState;

pub(crate) mod blocking;
pub(crate) mod pins;

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
