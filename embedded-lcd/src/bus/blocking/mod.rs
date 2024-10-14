use crate::driver::{LcdDisplayMode, LcdEntryMode, LcdFunctionMode, LcdStatus};

use super::LcdRegisterSelect;

pub trait LcdWrite<Delay: ?Sized> {
    type Error;

    fn write(
        &mut self,
        rs: LcdRegisterSelect,
        data: u8,
        delay: &mut Delay,
    ) -> Result<(), Self::Error>;

    fn write_command(&mut self, data: u8, delay: &mut Delay) -> Result<(), Self::Error> {
        self.write(LcdRegisterSelect::Control, data, delay)
    }

    fn write_memory(&mut self, data: u8, delay: &mut Delay) -> Result<(), Self::Error> {
        self.write(LcdRegisterSelect::Memory, data, delay)
    }
}

pub trait LcdRead<Delay: ?Sized>: LcdWrite<Delay> {
    fn read_status(&mut self, delay: &mut Delay) -> Result<LcdStatus, Self::Error>;
}

pub trait LcdInit<Delay: ?Sized>: LcdWrite<Delay> {
    fn init(
        &mut self,
        function: LcdFunctionMode,
        display_mode: LcdDisplayMode,
        entry: LcdEntryMode,
        delay: &mut Delay,
    ) -> Result<(), Self::Error>;
}
