use core::convert::Infallible;

use embedded_hal::{
    delay::DelayNs,
    digital::{ErrorType, InputPin, OutputPin, PinState},
};

use crate::{
    bus::{
        pins::ParallelPins,
        timings::{DefaultTimingsParallel4, DefaultTimingsParallel8, LcdTimingsParallel},
        LcdRegisterSelect,
    },
    driver::{LcdDisplayMode, LcdEntryMode, LcdFunctionMode, LcdStatus},
};

use super::{LcdInit, LcdRead, LcdWrite};

mod sealed {
    #[doc(hidden)]
    pub trait WriteModeSet<E> {
        fn set_write_mode(&mut self) -> Result<(), E>;
    }

    #[doc(hidden)]
    pub trait ReadModeSet<E> {
        fn set_read_mode(&mut self) -> Result<(), E>;
    }
}

use sealed::{ReadModeSet as _, WriteModeSet as _};

pub trait WriteModeSet<E>: sealed::WriteModeSet<E> {}

pub trait ReadModeSet<E>: sealed::ReadModeSet<E> {}

pub struct WriteOnly;

impl WriteOnly {
    /// This can be used to get a `&mut WriteOnly` when implementing your own pin struct.
    pub fn new<T>(data: &mut T) -> &mut Self {
        assert_eq!(core::mem::size_of::<Self>(), 0);
        assert_eq!(core::mem::align_of::<Self>(), 1);

        // # Safety
        // WriteOnly is a Zst with no alignment constraints and `&mut T` is never null.
        unsafe { core::mem::transmute(data) }
    }
}

impl<E> sealed::WriteModeSet<E> for WriteOnly {
    fn set_write_mode(&mut self) -> Result<(), E> {
        Ok(())
    }
}

impl<E> WriteModeSet<E> for WriteOnly {}

impl<T: OutputPin> sealed::WriteModeSet<T::Error> for T {
    fn set_write_mode(&mut self) -> Result<(), T::Error> {
        self.set_low()
    }
}

impl<T: OutputPin> WriteModeSet<T::Error> for T {}

impl<T: OutputPin> sealed::ReadModeSet<T::Error> for T {
    fn set_read_mode(&mut self) -> Result<(), T::Error> {
        self.set_high()
    }
}

impl<T: OutputPin> ReadModeSet<T::Error> for T {}

#[derive(Debug)]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LcdParallelBus<P: ParallelPins, T: LcdTimingsParallel, const WIDTH: u8> {
    pins: P,
    timings: T,
}

impl<P: ParallelPins, E> LcdParallelBus<P, DefaultTimingsParallel8, 8>
where
    P::RS: OutputPin<Error = E>,
    P::RW: WriteModeSet<E>,
    P::EN: OutputPin<Error = E>,
    P::D0: OutputPin<Error = E>,
    P::D1: OutputPin<Error = E>,
    P::D2: OutputPin<Error = E>,
    P::D3: OutputPin<Error = E>,
    P::D4: OutputPin<Error = E>,
    P::D5: OutputPin<Error = E>,
    P::D6: OutputPin<Error = E>,
    P::D7: OutputPin<Error = E>,
{
    #[inline]
    pub fn new_8bit(pins: P) -> Self {
        Self {
            pins,
            timings: DefaultTimingsParallel8,
        }
    }
}

impl<P: ParallelPins, T: LcdTimingsParallel, E> LcdParallelBus<P, T, 8>
where
    P::RS: OutputPin<Error = E>,
    P::RW: WriteModeSet<E>,
    P::EN: OutputPin<Error = E>,
    P::D0: OutputPin<Error = E>,
    P::D1: OutputPin<Error = E>,
    P::D2: OutputPin<Error = E>,
    P::D3: OutputPin<Error = E>,
    P::D4: OutputPin<Error = E>,
    P::D5: OutputPin<Error = E>,
    P::D6: OutputPin<Error = E>,
    P::D7: OutputPin<Error = E>,
{
    #[inline]
    pub fn new_8bit_with_timings(pins: P, timings: T) -> Self {
        Self { pins, timings }
    }
}

impl<P: ParallelPins<D0 = Infallible, D1 = Infallible, D2 = Infallible, D3 = Infallible>, E>
    LcdParallelBus<P, DefaultTimingsParallel4, 4>
where
    P::RS: OutputPin<Error = E>,
    P::RW: WriteModeSet<E>,
    P::EN: OutputPin<Error = E>,
    P::D4: OutputPin<Error = E>,
    P::D5: OutputPin<Error = E>,
    P::D6: OutputPin<Error = E>,
    P::D7: OutputPin<Error = E>,
{
    #[inline]
    pub fn new_4bit(pins: P) -> Self {
        Self {
            pins,
            timings: DefaultTimingsParallel4,
        }
    }
}

impl<P: ParallelPins, T: LcdTimingsParallel, E> LcdParallelBus<P, T, 4>
where
    P::RS: OutputPin<Error = E>,
    P::RW: WriteModeSet<E>,
    P::EN: OutputPin<Error = E>,
    P::D4: OutputPin<Error = E>,
    P::D5: OutputPin<Error = E>,
    P::D6: OutputPin<Error = E>,
    P::D7: OutputPin<Error = E>,
{
    #[inline]
    pub fn new_4bit_with_timings(pins: P, timings: T) -> Self {
        Self { pins, timings }
    }
}

impl<P: ParallelPins, T: LcdTimingsParallel, const WIDTH: u8> LcdParallelBus<P, T, WIDTH>
where
    P::EN: OutputPin,
{
    #[inline(always)]
    fn enable_on(
        &mut self,
        rs: LcdRegisterSelect,
        delay: &mut impl DelayNs,
    ) -> Result<(), <P::EN as ErrorType>::Error> {
        self.pins.en().set_high()?;
        self.timings.enable_pulse_on(rs, delay);
        Ok(())
    }

    #[inline(always)]
    fn enable_off(
        &mut self,
        rs: LcdRegisterSelect,
        delay: &mut impl DelayNs,
    ) -> Result<(), <P::EN as ErrorType>::Error> {
        self.pins.en().set_low()?;
        self.timings.enable_pulse_off(rs, delay);
        Ok(())
    }

    #[inline(always)]
    fn enable_on_read(
        &mut self,
        delay: &mut impl DelayNs,
    ) -> Result<(), <P::EN as ErrorType>::Error> {
        self.pins.en().set_high()?;
        self.timings.read_delay(delay);
        Ok(())
    }

    #[inline(always)]
    fn enable_pulse(
        &mut self,
        rs: LcdRegisterSelect,
        delay: &mut impl DelayNs,
    ) -> Result<(), <P::EN as ErrorType>::Error> {
        self.enable_on(rs, delay)?;
        self.enable_off(rs, delay)?;
        Ok(())
    }

    #[inline(always)]
    fn enable_pulse_no_delay_after(
        &mut self,
        rs: LcdRegisterSelect,
        delay: &mut impl DelayNs,
    ) -> Result<(), <P::EN as ErrorType>::Error> {
        self.enable_on(rs, delay)?;
        self.pins.en().set_low()?;
        Ok(())
    }
}

impl<P: ParallelPins, T: LcdTimingsParallel, E> LcdParallelBus<P, T, 8>
where
    P::RS: OutputPin<Error = E>,
    P::EN: OutputPin<Error = E>,
    P::D0: OutputPin<Error = E>,
    P::D1: OutputPin<Error = E>,
    P::D2: OutputPin<Error = E>,
    P::D3: OutputPin<Error = E>,
    P::D4: OutputPin<Error = E>,
    P::D5: OutputPin<Error = E>,
    P::D6: OutputPin<Error = E>,
    P::D7: OutputPin<Error = E>,
{
    #[inline(always)]
    fn set_8bit(&mut self, data: u8) -> Result<(), E> {
        self.pins.d0().set_state(PinState::from(data & 0x01 != 0))?;
        self.pins.d1().set_state(PinState::from(data & 0x02 != 0))?;
        self.pins.d2().set_state(PinState::from(data & 0x04 != 0))?;
        self.pins.d3().set_state(PinState::from(data & 0x08 != 0))?;
        self.pins.d4().set_state(PinState::from(data & 0x10 != 0))?;
        self.pins.d5().set_state(PinState::from(data & 0x20 != 0))?;
        self.pins.d6().set_state(PinState::from(data & 0x40 != 0))?;
        self.pins.d7().set_state(PinState::from(data & 0x80 != 0))?;

        Ok(())
    }
}

impl<P: ParallelPins, T: LcdTimingsParallel, E> LcdParallelBus<P, T, 8>
where
    P::RS: OutputPin<Error = E>,
    P::RW: WriteModeSet<E> + ReadModeSet<E>,
    P::EN: OutputPin<Error = E>,
    P::D0: OutputPin<Error = E> + InputPin<Error = E>,
    P::D1: OutputPin<Error = E> + InputPin<Error = E>,
    P::D2: OutputPin<Error = E> + InputPin<Error = E>,
    P::D3: OutputPin<Error = E> + InputPin<Error = E>,
    P::D4: OutputPin<Error = E> + InputPin<Error = E>,
    P::D5: OutputPin<Error = E> + InputPin<Error = E>,
    P::D6: OutputPin<Error = E> + InputPin<Error = E>,
    P::D7: OutputPin<Error = E> + InputPin<Error = E>,
{
    #[inline(always)]
    fn read_8bit(&mut self, delay: &mut impl DelayNs) -> Result<u8, E> {
        self.pins.rw().set_read_mode()?;
        self.pins.rs().set_low()?;
        self.pins.d0().set_high()?;
        self.pins.d1().set_high()?;
        self.pins.d2().set_high()?;
        self.pins.d3().set_high()?;
        self.pins.d4().set_high()?;
        self.pins.d5().set_high()?;
        self.pins.d6().set_high()?;
        self.pins.d7().set_high()?;
        self.enable_on_read(delay)?;

        let mut data = 0u8;
        data |= self.pins.d0().is_high()? as u8;
        data |= (self.pins.d1().is_high()? as u8) << 1;
        data |= (self.pins.d2().is_high()? as u8) << 2;
        data |= (self.pins.d3().is_high()? as u8) << 3;
        data |= (self.pins.d4().is_high()? as u8) << 4;
        data |= (self.pins.d5().is_high()? as u8) << 5;
        data |= (self.pins.d6().is_high()? as u8) << 6;
        data |= (self.pins.d7().is_high()? as u8) << 7;

        self.enable_off(LcdRegisterSelect::Control, delay)?;
        self.pins.rw().set_write_mode()?;

        Ok(data)
    }
}

impl<P: ParallelPins, T: LcdTimingsParallel, E> LcdParallelBus<P, T, 4>
where
    P::RS: OutputPin<Error = E>,
    P::EN: OutputPin<Error = E>,
    P::D4: OutputPin<Error = E>,
    P::D5: OutputPin<Error = E>,
    P::D6: OutputPin<Error = E>,
    P::D7: OutputPin<Error = E>,
{
    #[inline(always)]
    fn set_4bit(&mut self, data: u8) -> Result<(), E> {
        self.pins.d4().set_state(PinState::from(data & 0x1 != 0))?;
        self.pins.d5().set_state(PinState::from(data & 0x2 != 0))?;
        self.pins.d6().set_state(PinState::from(data & 0x4 != 0))?;
        self.pins.d7().set_state(PinState::from(data & 0x8 != 0))?;

        Ok(())
    }
}

impl<P: ParallelPins, T: LcdTimingsParallel, E> LcdParallelBus<P, T, 4>
where
    P::RS: OutputPin<Error = E>,
    P::RW: WriteModeSet<E> + ReadModeSet<E>,
    P::EN: OutputPin<Error = E>,
    P::D4: InputPin<Error = E> + OutputPin<Error = E>,
    P::D5: InputPin<Error = E> + OutputPin<Error = E>,
    P::D6: InputPin<Error = E> + OutputPin<Error = E>,
    P::D7: InputPin<Error = E> + OutputPin<Error = E>,
{
    #[inline(always)]
    fn read_4bit(&mut self, delay: &mut impl DelayNs) -> Result<u8, E> {
        self.pins.rw().set_read_mode()?;
        self.pins.d4().set_high()?;
        self.pins.d5().set_high()?;
        self.pins.d6().set_high()?;
        self.pins.d7().set_high()?;

        self.enable_on_read(delay)?;

        let mut data = 0u8;
        data |= (self.pins.d4().is_high()? as u8) << 4;
        data |= (self.pins.d5().is_high()? as u8) << 5;
        data |= (self.pins.d6().is_high()? as u8) << 6;
        data |= (self.pins.d7().is_high()? as u8) << 7;

        self.enable_off(LcdRegisterSelect::Control, delay)?;
        self.enable_on_read(delay)?;

        data |= self.pins.d4().is_high()? as u8;
        data |= (self.pins.d5().is_high()? as u8) << 1;
        data |= (self.pins.d6().is_high()? as u8) << 2;
        data |= (self.pins.d7().is_high()? as u8) << 3;

        self.enable_off(LcdRegisterSelect::Control, delay)?;
        self.pins.rw().set_write_mode()?;

        Ok(data)
    }
}

impl<P: ParallelPins, T: LcdTimingsParallel, E> LcdWrite for LcdParallelBus<P, T, 8>
where
    P::RS: OutputPin<Error = E>,
    P::EN: OutputPin<Error = E>,
    P::D0: OutputPin<Error = E>,
    P::D1: OutputPin<Error = E>,
    P::D2: OutputPin<Error = E>,
    P::D3: OutputPin<Error = E>,
    P::D4: OutputPin<Error = E>,
    P::D5: OutputPin<Error = E>,
    P::D6: OutputPin<Error = E>,
    P::D7: OutputPin<Error = E>,
{
    type Error = E;

    #[inline(always)]
    fn write(
        &mut self,
        rs: LcdRegisterSelect,
        data: u8,
        delay: &mut impl DelayNs,
    ) -> Result<(), Self::Error> {
        self.pins.rs().set_state(rs.into())?;
        self.set_8bit(data)?;
        self.enable_pulse(rs, delay)
    }
}

impl<P: ParallelPins, T: LcdTimingsParallel, E> LcdRead for LcdParallelBus<P, T, 8>
where
    P::RS: OutputPin<Error = E>,
    P::RW: WriteModeSet<E> + ReadModeSet<E>,
    P::EN: OutputPin<Error = E>,
    P::D0: OutputPin<Error = E> + InputPin<Error = E>,
    P::D1: OutputPin<Error = E> + InputPin<Error = E>,
    P::D2: OutputPin<Error = E> + InputPin<Error = E>,
    P::D3: OutputPin<Error = E> + InputPin<Error = E>,
    P::D4: OutputPin<Error = E> + InputPin<Error = E>,
    P::D5: OutputPin<Error = E> + InputPin<Error = E>,
    P::D6: OutputPin<Error = E> + InputPin<Error = E>,
    P::D7: OutputPin<Error = E> + InputPin<Error = E>,
{
    type Error = E;

    fn read_status(&mut self, delay: &mut impl DelayNs) -> Result<LcdStatus, Self::Error> {
        self.pins.rs().set_low()?;
        let data = self.read_8bit(delay)?;
        Ok(LcdStatus::from_bits_retain(data))
    }
}

impl<P: ParallelPins, T: LcdTimingsParallel, E> LcdInit for LcdParallelBus<P, T, 8>
where
    P::RS: OutputPin<Error = E>,
    P::RW: WriteModeSet<E>,
    P::EN: OutputPin<Error = E>,
    P::D0: OutputPin<Error = E>,
    P::D1: OutputPin<Error = E>,
    P::D2: OutputPin<Error = E>,
    P::D3: OutputPin<Error = E>,
    P::D4: OutputPin<Error = E>,
    P::D5: OutputPin<Error = E>,
    P::D6: OutputPin<Error = E>,
    P::D7: OutputPin<Error = E>,
{
    fn init(
        &mut self,
        function: crate::driver::LcdFunctionMode,
        display: LcdDisplayMode,
        entry: crate::driver::LcdEntryMode,
        delay: &mut impl DelayNs,
    ) -> Result<(), Self::Error> {
        self.pins.rs().set_low()?;
        self.pins.rw().set_write_mode()?;
        self.pins.en().set_low()?;
        self.timings.power_on_delay(delay);

        self.set_8bit(0x30)?;
        self.enable_pulse_no_delay_after(LcdRegisterSelect::Control, delay)?;
        self.timings.first_init_delay(delay);
        self.enable_pulse_no_delay_after(LcdRegisterSelect::Control, delay)?;
        self.timings.second_init_delay(delay);
        self.enable_pulse(LcdRegisterSelect::Control, delay)?;

        self.write_command(
            crate::FUNCTION_SET
                | function
                    .intersection(LcdFunctionMode::all())
                    .union(LcdFunctionMode::DATA_LENGTH)
                    .bits(),
            delay,
        )?;
        self.write_command(
            crate::DISPLAY_CONTROL | display.intersection(LcdDisplayMode::all()).bits(),
            delay,
        )?;
        self.write_command(crate::CLEAR_DISPLAY, delay)?;
        self.write_command(
            crate::ENTRY_MODE_SET | entry.intersection(LcdEntryMode::all()).bits(),
            delay,
        )?;

        Ok(())
    }
}

impl<P: ParallelPins, T: LcdTimingsParallel, E> LcdWrite for LcdParallelBus<P, T, 4>
where
    P::RS: OutputPin<Error = E>,
    P::EN: OutputPin<Error = E>,
    P::D4: OutputPin<Error = E>,
    P::D5: OutputPin<Error = E>,
    P::D6: OutputPin<Error = E>,
    P::D7: OutputPin<Error = E>,
{
    type Error = E;

    #[inline(always)]
    fn write(
        &mut self,
        rs: LcdRegisterSelect,
        data: u8,
        delay: &mut impl DelayNs,
    ) -> Result<(), Self::Error> {
        self.pins.rs().set_state(rs.into())?;
        self.set_4bit(data >> 4)?;
        self.enable_pulse(rs, delay)?;
        self.set_4bit(data)?;
        self.enable_pulse(rs, delay)
    }
}

impl<P: ParallelPins, T: LcdTimingsParallel, E> LcdRead for LcdParallelBus<P, T, 4>
where
    P::RS: OutputPin<Error = E>,
    P::RW: WriteModeSet<E> + ReadModeSet<E>,
    P::EN: OutputPin<Error = E>,
    P::D4: OutputPin<Error = E> + InputPin<Error = E>,
    P::D5: OutputPin<Error = E> + InputPin<Error = E>,
    P::D6: OutputPin<Error = E> + InputPin<Error = E>,
    P::D7: OutputPin<Error = E> + InputPin<Error = E>,
{
    type Error = E;

    fn read_status(&mut self, delay: &mut impl DelayNs) -> Result<LcdStatus, Self::Error> {
        self.pins.rs().set_low()?;
        let data = self.read_4bit(delay)?;
        Ok(LcdStatus::from_bits_retain(data))
    }
}

impl<P: ParallelPins, T: LcdTimingsParallel, E> LcdInit for LcdParallelBus<P, T, 4>
where
    P::RS: OutputPin<Error = E>,
    P::RW: WriteModeSet<E>,
    P::EN: OutputPin<Error = E>,
    P::D4: OutputPin<Error = E>,
    P::D5: OutputPin<Error = E>,
    P::D6: OutputPin<Error = E>,
    P::D7: OutputPin<Error = E>,
{
    fn init(
        &mut self,
        function: crate::driver::LcdFunctionMode,
        display: LcdDisplayMode,
        entry: crate::driver::LcdEntryMode,
        delay: &mut impl DelayNs,
    ) -> Result<(), Self::Error> {
        self.pins.rs().set_low()?;
        self.pins.rw().set_write_mode()?;
        self.pins.en().set_low()?;
        self.timings.power_on_delay(delay);

        self.set_4bit(0x3)?;
        self.enable_pulse_no_delay_after(LcdRegisterSelect::Control, delay)?;
        self.timings.first_init_delay(delay);
        self.enable_pulse_no_delay_after(LcdRegisterSelect::Control, delay)?;
        self.timings.second_init_delay(delay);
        self.enable_pulse(LcdRegisterSelect::Control, delay)?;

        self.set_4bit(crate::FUNCTION_SET >> 4)?; // set 4-bit bus
        self.enable_pulse(LcdRegisterSelect::Control, delay)?;

        self.write_command(
            crate::FUNCTION_SET
                | function
                    .intersection(LcdFunctionMode::all().difference(LcdFunctionMode::DATA_LENGTH))
                    .bits(),
            delay,
        )?;
        self.write_command(
            crate::DISPLAY_CONTROL | display.intersection(LcdDisplayMode::all()).bits(),
            delay,
        )?;
        self.write_command(crate::CLEAR_DISPLAY, delay)?;
        self.write_command(
            crate::ENTRY_MODE_SET | entry.intersection(LcdEntryMode::all()).bits(),
            delay,
        )?;

        Ok(())
    }
}
