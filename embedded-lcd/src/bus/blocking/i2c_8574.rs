use bitflags::bitflags;
use embedded_hal::{
    delay::DelayNs,
    i2c::{I2c, Operation},
};

use crate::{
    bus::{
        timings::{DefaultTimingsI2c, LcdTimingsI2c},
        LcdRegisterSelect,
    },
    driver::{LcdDisplayMode, LcdEntryMode, LcdFunctionMode, LcdStatus},
};

use super::{LcdInit, LcdRead, LcdWrite};

bitflags! {
    #[derive(Clone, Copy)]
    pub struct Pins: u8 {
        const REGISTER_SELECT = 0x01;
        const READ = 0x02;
        const ENABLE = 0x04;
        const BACKLIGHT = 0x08;
        const DATA = 0xf0;
    }
}

pub struct LcdI2c8574Bus<I, T> {
    i2c: I,
    address: u8,
    state: Pins,
    timings: T,
}

impl<I> LcdI2c8574Bus<I, DefaultTimingsI2c>
where
    I: I2c,
{
    pub fn new(i2c: I, address: u8) -> Self {
        Self {
            i2c,
            address,
            state: Pins::BACKLIGHT,
            timings: DefaultTimingsI2c,
        }
    }
}

impl<I, T> LcdI2c8574Bus<I, T>
where
    I: I2c,
    T: LcdTimingsI2c,
{
    pub fn set_backlight(&mut self, on: bool) -> Result<(), I::Error> {
        self.state.set(Pins::BACKLIGHT, on);
        self.i2c.write(self.address, &[self.state.bits()])
    }

    fn read_data(
        &mut self,
        rs: LcdRegisterSelect,
        delay: &mut impl DelayNs,
    ) -> Result<u8, I::Error> {
        let mut state = self.state;
        state.set(Pins::REGISTER_SELECT, rs == LcdRegisterSelect::Memory);

        let transfer = {
            let b = state | Pins::DATA | Pins::READ;
            pins_to_bytes([b, b | Pins::ENABLE])
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

#[inline(always)]
fn pins_to_bytes<const N: usize>(pins: [Pins; N]) -> [u8; N] {
    core::array::from_fn(|i| pins[i].bits())
}

impl<I, T> LcdWrite for LcdI2c8574Bus<I, T>
where
    I: I2c,
    T: LcdTimingsI2c,
{
    type Error = I::Error;

    fn write(
        &mut self,
        rs: LcdRegisterSelect,
        data: u8,
        delay: &mut impl DelayNs,
    ) -> Result<(), Self::Error> {
        let mut state = self.state;
        state.set(Pins::REGISTER_SELECT, rs == LcdRegisterSelect::Memory);

        let upper = {
            let b = state | Pins::from_bits_retain(data & 0xf0);
            pins_to_bytes([b, b | Pins::ENABLE])
        };
        let lower = {
            let b = state | Pins::from_bits_retain(data << 4);
            pins_to_bytes([b, b | Pins::ENABLE])
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

impl<I, T> LcdRead for LcdI2c8574Bus<I, T>
where
    I: I2c,
    T: LcdTimingsI2c,
{
    type Error = I::Error;

    fn read_status(
        &mut self,
        delay: &mut impl DelayNs,
    ) -> Result<crate::driver::LcdStatus, Self::Error> {
        self.read_data(LcdRegisterSelect::Control, delay)
            .map(LcdStatus::from_bits_retain)
    }
}

impl<I, T> LcdInit for LcdI2c8574Bus<I, T>
where
    I: I2c,
    T: LcdTimingsI2c,
{
    fn init(
        &mut self,
        function: crate::driver::LcdFunctionMode,
        display: crate::driver::LcdDisplayMode,
        entry: crate::driver::LcdEntryMode,
        delay: &mut impl DelayNs,
    ) -> Result<(), Self::Error> {
        self.timings.power_on_delay(delay);

        let transfer = {
            let b = self.state | Pins::from_bits_retain(0x30);
            pins_to_bytes([b, b | Pins::ENABLE])
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
            pins_to_bytes([b, b | Pins::ENABLE])
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
