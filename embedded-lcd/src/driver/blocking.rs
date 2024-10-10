use embedded_hal::delay::DelayNs;

use crate::{
    bus::{
        blocking::{LcdInit, LcdRead, LcdWrite},
        LcdRegisterSelect,
    },
    charset::CharsetWithFallback,
    driver::{LcdEntryMode, LcdFunctionMode},
    memory_map::DisplayMemoryMap,
};

use super::{LcdDisplayMode, LcdDriver, LcdStatus};

pub trait BlockingLcdDriver<B, M, C>: Sized
where
    B: LcdWrite,
    M: DisplayMemoryMap,
{
    fn init(memory_map: M, charset: C, bus: B, delay: &mut impl DelayNs) -> Result<Self, B::Error>
    where
        B: LcdInit;

    fn clear(&mut self, delay: &mut impl DelayNs) -> Result<(), B::Error>;

    fn return_home(&mut self, delay: &mut impl DelayNs) -> Result<(), <B as LcdWrite>::Error>;

    fn set_display_mode(
        &mut self,
        display_mode: LcdDisplayMode,
        delay: &mut impl DelayNs,
    ) -> Result<(), B::Error>;

    fn set_xy(&mut self, x: u8, y: u8, delay: &mut impl DelayNs) -> Result<(), B::Error>;

    fn set_address(&mut self, address: u8, delay: &mut impl DelayNs) -> Result<(), B::Error>;

    fn write_char(&mut self, ch: char, delay: &mut impl DelayNs) -> Result<(), B::Error>
    where
        C: CharsetWithFallback;

    fn write_str(&mut self, s: &str, delay: &mut impl DelayNs) -> Result<(), <B as LcdWrite>::Error>
    where
        C: CharsetWithFallback,
    {
        for ch in s.chars() {
            self.write_char(ch, delay)?;
        }
        Ok(())
    }

    fn status(&mut self, delay: &mut impl DelayNs) -> Result<LcdStatus, <B as LcdRead>::Error>
    where
        B: LcdRead<Error = <B as LcdWrite>::Error>;
}

impl<B, M, C> BlockingLcdDriver<B, M, C> for LcdDriver<B, M, C>
where
    B: LcdWrite,
    M: DisplayMemoryMap,
{
    fn init(
        memory_map: M,
        charset: C,
        mut bus: B,
        delay: &mut impl DelayNs,
    ) -> Result<Self, B::Error>
    where
        B: LcdInit,
    {
        let mut function = LcdFunctionMode::empty();
        // Enables the second memory line for 2-line displays.
        if memory_map.has_two_memory_lines() {
            function |= LcdFunctionMode::DISPLAY_LINES;
        }

        let display_mode = LcdDisplayMode::SHOW_DISPLAY | LcdDisplayMode::SHOW_CURSOR;

        let entry = LcdEntryMode::INCREMENT;

        // TODO: 5x10 dot font option

        bus.init(function, display_mode, entry, delay)?;

        Ok(Self {
            bus,
            memory_map,
            charset,
            display_mode,
        })
    }

    fn clear(&mut self, delay: &mut impl DelayNs) -> Result<(), <B as LcdWrite>::Error> {
        self.bus
            .write(LcdRegisterSelect::Control, crate::CLEAR_DISPLAY, delay)
    }

    fn return_home(&mut self, delay: &mut impl DelayNs) -> Result<(), <B as LcdWrite>::Error> {
        self.bus
            .write(LcdRegisterSelect::Control, crate::RETURN_HOME, delay)
    }

    fn set_display_mode(
        &mut self,
        display_mode: LcdDisplayMode,
        delay: &mut impl DelayNs,
    ) -> Result<(), <B as LcdWrite>::Error> {
        self.bus.write(
            LcdRegisterSelect::Control,
            crate::DISPLAY_CONTROL | display_mode.intersection(LcdDisplayMode::all()).bits(),
            delay,
        )
    }

    fn set_xy(
        &mut self,
        x: u8,
        y: u8,
        delay: &mut impl DelayNs,
    ) -> Result<(), <B as LcdWrite>::Error> {
        let Some(address) = self.memory_map.address_for_xy(x, y) else {
            return Ok(());
        };
        self.set_address(address, delay)
    }

    fn set_address(
        &mut self,
        address: u8,
        delay: &mut impl DelayNs,
    ) -> Result<(), <B as LcdWrite>::Error> {
        self.bus.write(
            LcdRegisterSelect::Control,
            crate::SET_DDRAM_ADDRESS | address,
            delay,
        )?;
        Ok(())
    }

    fn write_char(
        &mut self,
        ch: char,
        delay: &mut impl DelayNs,
    ) -> Result<(), <B as LcdWrite>::Error>
    where
        C: CharsetWithFallback,
    {
        self.bus.write(
            LcdRegisterSelect::Memory,
            self.charset.code_from_utf8_with_fallback(ch),
            delay,
        )
    }

    fn status(&mut self, delay: &mut impl DelayNs) -> Result<LcdStatus, <B as LcdRead>::Error>
    where
        B: LcdRead<Error = <B as LcdWrite>::Error>,
    {
        self.bus.read_status(delay)
    }
}
