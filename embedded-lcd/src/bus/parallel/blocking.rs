use embedded_hal::{
    delay::DelayNs,
    digital::{ErrorType, InputPin, OutputPin, PinState},
};

use crate::{
    bus::{
        blocking::{LcdInit, LcdRead, LcdWrite},
        LcdRegisterSelect, LcdTimingsParallel,
    },
    LcdDisplayMode, LcdEntryMode, LcdFunctionMode, LcdStatus,
};

use super::{
    sealed::{LcdParallelReadModeSet as _, LcdParallelWriteModeSet as _},
    LcdParallelBus, LcdParallelPins, LcdParallelReadModeSet, LcdParallelWriteModeSet,
};

impl<P: LcdParallelPins, T, const WIDTH: u8> LcdParallelBus<P, T, WIDTH>
where
    P::EN: OutputPin,
{
    #[inline(always)]
    fn enable_on<Delay: ?Sized>(
        &mut self,
        rs: LcdRegisterSelect,
        delay: &mut Delay,
    ) -> Result<(), <P::EN as ErrorType>::Error>
    where
        T: LcdTimingsParallel<Delay>,
    {
        self.pins.en().set_high()?;
        self.timings.enable_pulse_on(rs, delay);
        Ok(())
    }

    #[inline(always)]
    fn enable_off<Delay: ?Sized>(
        &mut self,
        rs: LcdRegisterSelect,
        delay: &mut Delay,
    ) -> Result<(), <P::EN as ErrorType>::Error>
    where
        T: LcdTimingsParallel<Delay>,
    {
        self.pins.en().set_low()?;
        self.timings.enable_pulse_off(rs, delay);
        Ok(())
    }

    #[inline(always)]
    fn enable_on_read<Delay: ?Sized>(
        &mut self,
        delay: &mut Delay,
    ) -> Result<(), <P::EN as ErrorType>::Error>
    where
        T: LcdTimingsParallel<Delay>,
    {
        self.pins.en().set_high()?;
        self.timings.read_delay(delay);
        Ok(())
    }

    #[inline(always)]
    fn enable_pulse<Delay: ?Sized>(
        &mut self,
        rs: LcdRegisterSelect,
        delay: &mut Delay,
    ) -> Result<(), <P::EN as ErrorType>::Error>
    where
        T: LcdTimingsParallel<Delay>,
    {
        self.enable_on(rs, delay)?;
        self.enable_off(rs, delay)?;
        Ok(())
    }

    #[inline(always)]
    fn enable_pulse_no_delay_after<Delay: ?Sized>(
        &mut self,
        rs: LcdRegisterSelect,
        delay: &mut Delay,
    ) -> Result<(), <P::EN as ErrorType>::Error>
    where
        T: LcdTimingsParallel<Delay>,
    {
        self.enable_on(rs, delay)?;
        self.pins.en().set_low()?;
        Ok(())
    }
}

impl<P: LcdParallelPins, T, E> LcdParallelBus<P, T, 8>
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

impl<P: LcdParallelPins, T, E> LcdParallelBus<P, T, 8>
where
    P::RS: OutputPin<Error = E>,
    P::RW: LcdParallelWriteModeSet<E> + LcdParallelReadModeSet<E>,
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
    fn read_8bit<Delay: ?Sized>(&mut self, delay: &mut Delay) -> Result<u8, E>
    where
        T: LcdTimingsParallel<Delay>,
    {
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

impl<P: LcdParallelPins, T, E> LcdParallelBus<P, T, 4>
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

impl<P: LcdParallelPins, T, E> LcdParallelBus<P, T, 4>
where
    P::RS: OutputPin<Error = E>,
    P::RW: LcdParallelWriteModeSet<E> + LcdParallelReadModeSet<E>,
    P::EN: OutputPin<Error = E>,
    P::D4: InputPin<Error = E> + OutputPin<Error = E>,
    P::D5: InputPin<Error = E> + OutputPin<Error = E>,
    P::D6: InputPin<Error = E> + OutputPin<Error = E>,
    P::D7: InputPin<Error = E> + OutputPin<Error = E>,
{
    #[inline(always)]
    fn read_4bit<Delay: ?Sized>(&mut self, delay: &mut Delay) -> Result<u8, E>
    where
        T: LcdTimingsParallel<Delay>,
    {
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

impl<P: LcdParallelPins, T, E, Delay> LcdWrite<Delay> for LcdParallelBus<P, T, 8>
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
    Delay: DelayNs + ?Sized,
    T: LcdTimingsParallel<Delay>,
{
    type Error = E;

    #[inline(always)]
    fn write(
        &mut self,
        rs: LcdRegisterSelect,
        data: u8,
        delay: &mut Delay,
    ) -> Result<(), Self::Error> {
        self.pins.rs().set_state(rs.into())?;
        self.set_8bit(data)?;
        self.enable_pulse(rs, delay)
    }
}

impl<P: LcdParallelPins, T, E, Delay> LcdRead<Delay> for LcdParallelBus<P, T, 8>
where
    P::RS: OutputPin<Error = E>,
    P::RW: LcdParallelWriteModeSet<E> + LcdParallelReadModeSet<E>,
    P::EN: OutputPin<Error = E>,
    P::D0: OutputPin<Error = E> + InputPin<Error = E>,
    P::D1: OutputPin<Error = E> + InputPin<Error = E>,
    P::D2: OutputPin<Error = E> + InputPin<Error = E>,
    P::D3: OutputPin<Error = E> + InputPin<Error = E>,
    P::D4: OutputPin<Error = E> + InputPin<Error = E>,
    P::D5: OutputPin<Error = E> + InputPin<Error = E>,
    P::D6: OutputPin<Error = E> + InputPin<Error = E>,
    P::D7: OutputPin<Error = E> + InputPin<Error = E>,
    Delay: DelayNs + ?Sized,
    T: LcdTimingsParallel<Delay>,
{
    fn read_status(&mut self, delay: &mut Delay) -> Result<LcdStatus, Self::Error> {
        self.pins.rs().set_low()?;
        let data = self.read_8bit(delay)?;
        Ok(LcdStatus::from_bits_retain(data))
    }
}

impl<P: LcdParallelPins, T, E, Delay> LcdInit<Delay> for LcdParallelBus<P, T, 8>
where
    P::RS: OutputPin<Error = E>,
    P::RW: LcdParallelWriteModeSet<E>,
    P::EN: OutputPin<Error = E>,
    P::D0: OutputPin<Error = E>,
    P::D1: OutputPin<Error = E>,
    P::D2: OutputPin<Error = E>,
    P::D3: OutputPin<Error = E>,
    P::D4: OutputPin<Error = E>,
    P::D5: OutputPin<Error = E>,
    P::D6: OutputPin<Error = E>,
    P::D7: OutputPin<Error = E>,
    Delay: DelayNs + ?Sized,
    T: LcdTimingsParallel<Delay>,
{
    fn init(
        &mut self,
        function: LcdFunctionMode,
        display: LcdDisplayMode,
        entry: LcdEntryMode,
        delay: &mut Delay,
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

impl<P: LcdParallelPins, T, E, Delay> LcdWrite<Delay> for LcdParallelBus<P, T, 4>
where
    P::RS: OutputPin<Error = E>,
    P::EN: OutputPin<Error = E>,
    P::D4: OutputPin<Error = E>,
    P::D5: OutputPin<Error = E>,
    P::D6: OutputPin<Error = E>,
    P::D7: OutputPin<Error = E>,
    Delay: DelayNs + ?Sized,
    T: LcdTimingsParallel<Delay>,
{
    type Error = E;

    #[inline(always)]
    fn write(
        &mut self,
        rs: LcdRegisterSelect,
        data: u8,
        delay: &mut Delay,
    ) -> Result<(), Self::Error> {
        self.pins.rs().set_state(rs.into())?;
        self.set_4bit(data >> 4)?;
        self.enable_pulse(rs, delay)?;
        self.set_4bit(data)?;
        self.enable_pulse(rs, delay)
    }
}

impl<P: LcdParallelPins, T, E, Delay> LcdRead<Delay> for LcdParallelBus<P, T, 4>
where
    P::RS: OutputPin<Error = E>,
    P::RW: LcdParallelWriteModeSet<E> + LcdParallelReadModeSet<E>,
    P::EN: OutputPin<Error = E>,
    P::D4: OutputPin<Error = E> + InputPin<Error = E>,
    P::D5: OutputPin<Error = E> + InputPin<Error = E>,
    P::D6: OutputPin<Error = E> + InputPin<Error = E>,
    P::D7: OutputPin<Error = E> + InputPin<Error = E>,
    Delay: DelayNs + ?Sized,
    T: LcdTimingsParallel<Delay>,
{
    fn read_status(&mut self, delay: &mut Delay) -> Result<LcdStatus, Self::Error> {
        self.pins.rs().set_low()?;
        let data = self.read_4bit(delay)?;
        Ok(LcdStatus::from_bits_retain(data))
    }
}

impl<P: LcdParallelPins, T, E, Delay> LcdInit<Delay> for LcdParallelBus<P, T, 4>
where
    P::RS: OutputPin<Error = E>,
    P::RW: LcdParallelWriteModeSet<E>,
    P::EN: OutputPin<Error = E>,
    P::D4: OutputPin<Error = E>,
    P::D5: OutputPin<Error = E>,
    P::D6: OutputPin<Error = E>,
    P::D7: OutputPin<Error = E>,
    Delay: DelayNs + ?Sized,
    T: LcdTimingsParallel<Delay>,
{
    fn init(
        &mut self,
        function: crate::driver::LcdFunctionMode,
        display: LcdDisplayMode,
        entry: crate::driver::LcdEntryMode,
        delay: &mut Delay,
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
