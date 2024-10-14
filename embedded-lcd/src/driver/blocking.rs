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

pub trait BlockingLcdDriverInit<Delay>: Sized
where
    Delay: DelayNs + ?Sized,
{
    type MemoryMap: DisplayMemoryMap;
    type Charset;
    type Bus: LcdInit<Delay> + LcdWrite<Delay>;

    fn init(
        memory_map: Self::MemoryMap,
        charset: Self::Charset,
        bus: Self::Bus,
        delay: &mut Delay,
    ) -> Result<Self, <Self::Bus as LcdWrite<Delay>>::Error>;
}

pub trait BlockingLcdDriverDestroy {
    type Bus;

    fn destroy(self) -> Self::Bus;
}

pub trait BlockingLcdWrite<Delay: ?Sized> {
    type Error;

    fn write_char(&mut self, ch: char, delay: &mut Delay) -> Result<(), Self::Error>;

    fn write_str(&mut self, s: &str, delay: &mut Delay) -> Result<(), Self::Error> {
        for ch in s.chars() {
            self.write_char(ch, delay)?;
        }
        Ok(())
    }
}

pub trait BlockingLcdRead<Delay: ?Sized> {
    type Error;

    fn status(&mut self, delay: &mut Delay) -> Result<LcdStatus, Self::Error>;
}

pub trait BlockingLcdDriver<Delay: ?Sized> {
    type Error;

    fn clear(&mut self, delay: &mut Delay) -> Result<(), Self::Error>;

    fn return_home(&mut self, delay: &mut Delay) -> Result<(), Self::Error>;

    fn set_display_mode(
        &mut self,
        display_mode: LcdDisplayMode,
        delay: &mut Delay,
    ) -> Result<(), Self::Error>;

    fn set_xy(&mut self, x: u8, y: u8, delay: &mut Delay) -> Result<(), Self::Error>;

    fn set_address(&mut self, address: u8, delay: &mut Delay) -> Result<(), Self::Error>;
}

impl<B, M, C, Delay> BlockingLcdDriverInit<Delay> for LcdDriver<B, M, C>
where
    B: LcdInit<Delay> + LcdWrite<Delay>,
    M: DisplayMemoryMap,
    Delay: DelayNs + ?Sized,
{
    type MemoryMap = M;
    type Charset = C;
    type Bus = B;

    // TODO: hand back arguments on error
    fn init(
        memory_map: Self::MemoryMap,
        charset: Self::Charset,
        mut bus: Self::Bus,
        delay: &mut Delay,
    ) -> Result<Self, <Self::Bus as LcdWrite<Delay>>::Error> {
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
}

impl<B, M, C, Delay> BlockingLcdDriver<Delay> for LcdDriver<B, M, C>
where
    B: LcdWrite<Delay>,
    M: DisplayMemoryMap,
    Delay: DelayNs + ?Sized,
{
    type Error = B::Error;

    fn clear(&mut self, delay: &mut Delay) -> Result<(), Self::Error> {
        self.bus
            .write(LcdRegisterSelect::Control, crate::CLEAR_DISPLAY, delay)
    }

    fn return_home(&mut self, delay: &mut Delay) -> Result<(), Self::Error> {
        self.bus
            .write(LcdRegisterSelect::Control, crate::RETURN_HOME, delay)
    }

    fn set_display_mode(
        &mut self,
        display_mode: LcdDisplayMode,
        delay: &mut Delay,
    ) -> Result<(), Self::Error> {
        self.bus.write(
            LcdRegisterSelect::Control,
            crate::DISPLAY_CONTROL | display_mode.intersection(LcdDisplayMode::all()).bits(),
            delay,
        )
    }

    fn set_xy(&mut self, x: u8, y: u8, delay: &mut Delay) -> Result<(), Self::Error> {
        let Some(address) = self.memory_map.address_for_xy(x, y) else {
            return Ok(()); // TODO: Better error handling
        };
        self.set_address(address, delay)
    }

    fn set_address(&mut self, address: u8, delay: &mut Delay) -> Result<(), Self::Error> {
        self.bus.write(
            LcdRegisterSelect::Control,
            crate::SET_DDRAM_ADDRESS | address,
            delay,
        )?;
        Ok(())
    }
}

impl<B, M, C, Delay> BlockingLcdWrite<Delay> for LcdDriver<B, M, C>
where
    B: LcdWrite<Delay>,
    M: DisplayMemoryMap,
    C: CharsetWithFallback,
    Delay: DelayNs + ?Sized,
{
    type Error = B::Error;

    fn write_char(&mut self, ch: char, delay: &mut Delay) -> Result<(), Self::Error> {
        self.bus.write(
            LcdRegisterSelect::Memory,
            self.charset.code_from_utf8_with_fallback(ch),
            delay,
        )
    }
}

impl<B, M, C, Delay> BlockingLcdRead<Delay> for LcdDriver<B, M, C>
where
    B: LcdRead<Delay>,
    M: DisplayMemoryMap,
    C: CharsetWithFallback,
    Delay: DelayNs + ?Sized,
{
    type Error = B::Error;

    fn status(&mut self, delay: &mut Delay) -> Result<LcdStatus, Self::Error>
    where
        B: LcdRead<Delay>,
    {
        self.bus.read_status(delay)
    }
}
