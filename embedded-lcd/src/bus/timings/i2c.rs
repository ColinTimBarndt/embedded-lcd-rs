use embedded_hal::delay::DelayNs;

use crate::bus::LcdRegisterSelect;

pub trait LcdTimingsI2c {
    fn enable_pulse_on(&self, rs: LcdRegisterSelect, delay: &mut impl DelayNs);

    fn enable_pulse_off(&self, rs: LcdRegisterSelect, delay: &mut impl DelayNs);

    fn read_delay(&self, delay: &mut impl DelayNs);

    fn power_on_delay(&self, delay: &mut impl DelayNs);

    fn first_init_delay(&self, delay: &mut impl DelayNs);

    fn second_init_delay(&self, delay: &mut impl DelayNs);
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DefaultTimingsI2c;

impl LcdTimingsI2c for DefaultTimingsI2c {
    #[inline(always)]
    fn enable_pulse_on(&self, rs: LcdRegisterSelect, delay: &mut impl DelayNs) {
        match rs {
            LcdRegisterSelect::Control => delay.delay_us(750),
            LcdRegisterSelect::Memory => delay.delay_us(3),
        }
    }

    #[inline(always)]
    fn enable_pulse_off(&self, rs: LcdRegisterSelect, delay: &mut impl DelayNs) {
        match rs {
            LcdRegisterSelect::Control => delay.delay_us(800),
            LcdRegisterSelect::Memory => delay.delay_us(50),
        }
    }

    #[inline(always)]
    fn read_delay(&self, delay: &mut impl DelayNs) {
        delay.delay_us(800);
    }

    #[inline(always)]
    fn power_on_delay(&self, delay: &mut impl DelayNs) {
        delay.delay_ms(50);
    }

    #[inline(always)]
    fn first_init_delay(&self, delay: &mut impl DelayNs) {
        delay.delay_us(4500);
    }

    #[inline(always)]
    fn second_init_delay(&self, delay: &mut impl DelayNs) {
        delay.delay_us(100);
    }
}
