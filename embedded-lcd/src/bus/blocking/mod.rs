use embedded_hal::delay::DelayNs;

use crate::driver::{LcdDisplayMode, LcdEntryMode, LcdFunctionMode, LcdStatus};

use super::LcdRegisterSelect;

pub mod parallel;

pub trait LcdRead {
    type Error;

    fn read_status(&mut self, delay: &mut impl DelayNs) -> Result<LcdStatus, Self::Error>;
}

pub trait LcdWrite {
    type Error;

    fn write(
        &mut self,
        rs: LcdRegisterSelect,
        data: u8,
        delay: &mut impl DelayNs,
    ) -> Result<(), Self::Error>;

    fn write_command(&mut self, data: u8, delay: &mut impl DelayNs) -> Result<(), Self::Error> {
        self.write(LcdRegisterSelect::Control, data, delay)
    }

    fn write_memory(&mut self, data: u8, delay: &mut impl DelayNs) -> Result<(), Self::Error> {
        self.write(LcdRegisterSelect::Memory, data, delay)
    }
}

pub trait LcdInit: LcdWrite {
    fn init(
        &mut self,
        function: LcdFunctionMode,
        display_mode: LcdDisplayMode,
        entry: LcdEntryMode,
        delay: &mut impl DelayNs,
    ) -> Result<(), Self::Error>;
}
