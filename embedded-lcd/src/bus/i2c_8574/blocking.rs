use embedded_hal::{
    delay::DelayNs,
    i2c::{I2c, Operation},
};

use crate::{
    bus::{
        blocking::{LcdInit, LcdRead, LcdWrite},
        LcdRegisterSelect, LcdTimingsI2c,
    },
    LcdDisplayMode, LcdEntryMode, LcdFunctionMode, LcdStatus,
};

use super::{LcdI2c8574Bus, Pins};

impl<I, T> LcdI2c8574Bus<I, T>
where
    I: I2c,
{
    pub fn set_backlight(&mut self, on: bool) -> Result<(), I::Error> {
        self.state.set(Pins::BACKLIGHT, on);
        self.i2c.write(self.address, &[self.state.bits()])
    }

    fn read_data<Delay: ?Sized>(
        &mut self,
        rs: LcdRegisterSelect,
        delay: &mut Delay,
    ) -> Result<u8, I::Error>
    where
        T: LcdTimingsI2c<Delay>,
    {
        let mut state = self.state;
        state.set(Pins::REGISTER_SELECT, rs == LcdRegisterSelect::Memory);

        let transfer = {
            let b = state | Pins::DATA | Pins::READ;
            Pins::bits_array([b, b | Pins::ENABLE])
        };
        let mut read = 0;

        self.i2c.write(self.address, &transfer)?;
        self.timings.read_delay(delay);
        self.i2c.transaction(
            self.address,
            &mut [
                Operation::Read(core::slice::from_mut(&mut read)),
                Operation::Write(&transfer[..1]),
            ],
        )?;
        self.timings.enable_pulse_off(rs, delay);
        let mut data = read & 0xf0;

        self.i2c.write(self.address, &transfer)?;
        self.timings.read_delay(delay);
        self.i2c.transaction(
            self.address,
            &mut [
                Operation::Read(core::slice::from_mut(&mut read)),
                Operation::Write(&transfer[..1]),
            ],
        )?;
        self.timings.enable_pulse_off(rs, delay);
        data |= read >> 4;

        Ok(data)
    }
}

impl<I, T, Delay> LcdWrite<Delay> for LcdI2c8574Bus<I, T>
where
    I: I2c,
    T: LcdTimingsI2c<Delay>,
    Delay: DelayNs + ?Sized,
{
    type Error = I::Error;

    fn write(
        &mut self,
        rs: LcdRegisterSelect,
        data: u8,
        delay: &mut Delay,
    ) -> Result<(), Self::Error> {
        let mut state = self.state;
        state.set(Pins::REGISTER_SELECT, rs == LcdRegisterSelect::Memory);

        let upper = {
            let b = state | Pins::from_bits_retain(data & 0xf0);
            Pins::bits_array([b, b | Pins::ENABLE])
        };
        let lower = {
            let b = state | Pins::from_bits_retain(data << 4);
            Pins::bits_array([b, b | Pins::ENABLE])
        };

        self.i2c.write(self.address, &upper)?;
        self.timings.enable_pulse_on(rs, delay);
        self.i2c.write(self.address, &upper[..1])?;
        self.timings.enable_pulse_off(rs, delay);

        self.i2c.write(self.address, &lower)?;
        self.timings.enable_pulse_on(rs, delay);
        self.i2c.write(self.address, &lower[..1])?;
        self.timings.enable_pulse_off(rs, delay);

        Ok(())
    }
}

impl<I, T, Delay> LcdRead<Delay> for LcdI2c8574Bus<I, T>
where
    I: I2c,
    T: LcdTimingsI2c<Delay>,
    Delay: DelayNs + ?Sized,
{
    fn read_status(&mut self, delay: &mut Delay) -> Result<LcdStatus, Self::Error> {
        self.read_data(LcdRegisterSelect::Control, delay)
            .map(LcdStatus::from_bits_retain)
    }
}

impl<I, T, Delay> LcdInit<Delay> for LcdI2c8574Bus<I, T>
where
    I: I2c,
    T: LcdTimingsI2c<Delay>,
    Delay: DelayNs + ?Sized,
{
    fn init(
        &mut self,
        function: LcdFunctionMode,
        display: LcdDisplayMode,
        entry: LcdEntryMode,
        delay: &mut Delay,
    ) -> Result<(), Self::Error> {
        self.timings.power_on_delay(delay);

        let transfer = {
            let b = self.state | Pins::from_bits_retain(0x30);
            Pins::bits_array([b, b | Pins::ENABLE])
        };

        self.i2c.write(self.address, &transfer)?;
        self.timings
            .enable_pulse_on(LcdRegisterSelect::Control, delay);
        self.i2c.write(self.address, &transfer[..1])?;
        self.timings.first_init_delay(delay);

        self.i2c.write(self.address, &transfer[1..])?;
        self.timings
            .enable_pulse_on(LcdRegisterSelect::Control, delay);
        self.i2c.write(self.address, &transfer[..1])?;
        self.timings.second_init_delay(delay);

        self.i2c.write(self.address, &transfer[1..])?;
        self.timings
            .enable_pulse_on(LcdRegisterSelect::Control, delay);
        self.i2c.write(self.address, &transfer[..1])?;
        self.timings
            .enable_pulse_off(LcdRegisterSelect::Control, delay);

        // set 4-bit bus
        let transfer = {
            let b = self.state | Pins::from_bits_retain(crate::FUNCTION_SET);
            Pins::bits_array([b, b | Pins::ENABLE])
        };

        self.i2c.write(self.address, &transfer)?;
        self.timings
            .enable_pulse_on(LcdRegisterSelect::Control, delay);
        self.i2c.write(self.address, &transfer[..1])?;
        self.timings
            .enable_pulse_off(LcdRegisterSelect::Control, delay);

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
