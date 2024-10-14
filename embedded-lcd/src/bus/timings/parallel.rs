use embedded_hal::delay::DelayNs;

use crate::bus::LcdRegisterSelect;

pub trait LcdTimingsParallel<Delay: ?Sized> {
    fn enable_pulse_on(&self, rs: LcdRegisterSelect, delay: &mut Delay);

    fn enable_pulse_off(&self, rs: LcdRegisterSelect, delay: &mut Delay);

    fn read_delay(&self, delay: &mut Delay);

    fn power_on_delay(&self, delay: &mut Delay);

    fn first_init_delay(&self, delay: &mut Delay);

    fn second_init_delay(&self, delay: &mut Delay);
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DefaultTimingsParallel8;

impl<Delay> LcdTimingsParallel<Delay> for DefaultTimingsParallel8
where
    Delay: DelayNs + ?Sized,
{
    #[inline(always)]
    fn enable_pulse_on(&self, rs: LcdRegisterSelect, delay: &mut Delay) {
        match rs {
            LcdRegisterSelect::Control => delay.delay_us(1500),
            LcdRegisterSelect::Memory => delay.delay_ns(500), // PW_EH >= 230ns on HITACHI specification p. 52
        }
    }

    #[inline(always)]
    fn enable_pulse_off(&self, rs: LcdRegisterSelect, delay: &mut Delay) {
        match rs {
            LcdRegisterSelect::Control => delay.delay_ms(1),
            LcdRegisterSelect::Memory => delay.delay_us(50),
        }
    }

    #[inline(always)]
    fn read_delay(&self, delay: &mut Delay) {
        delay.delay_ms(1);
    }

    #[inline(always)]
    fn power_on_delay(&self, delay: &mut Delay) {
        delay.delay_ms(50);
    }

    #[inline(always)]
    fn first_init_delay(&self, delay: &mut Delay) {
        delay.delay_us(4500);
    }

    #[inline(always)]
    fn second_init_delay(&self, delay: &mut Delay) {
        delay.delay_us(100);
    }
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DefaultTimingsParallel4;

impl<Delay> LcdTimingsParallel<Delay> for DefaultTimingsParallel4
where
    Delay: DelayNs + ?Sized,
{
    #[inline(always)]
    fn enable_pulse_on(&self, rs: LcdRegisterSelect, delay: &mut Delay) {
        match rs {
            LcdRegisterSelect::Control => delay.delay_us(750),
            LcdRegisterSelect::Memory => delay.delay_us(3),
        }
    }

    #[inline(always)]
    fn enable_pulse_off(&self, rs: LcdRegisterSelect, delay: &mut Delay) {
        match rs {
            LcdRegisterSelect::Control => delay.delay_us(800),
            LcdRegisterSelect::Memory => delay.delay_us(50),
        }
    }

    #[inline(always)]
    fn read_delay(&self, delay: &mut Delay) {
        delay.delay_us(800);
    }

    #[inline(always)]
    fn power_on_delay(&self, delay: &mut Delay) {
        delay.delay_ms(50);
    }

    #[inline(always)]
    fn first_init_delay(&self, delay: &mut Delay) {
        delay.delay_us(4500);
    }

    #[inline(always)]
    fn second_init_delay(&self, delay: &mut Delay) {
        delay.delay_us(100);
    }
}
