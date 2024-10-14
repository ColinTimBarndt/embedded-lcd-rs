#[cfg(feature = "blocking")]
mod blocking;
mod pins;
pub use pins::*;

use core::convert::Infallible;

use embedded_hal::digital::OutputPin;

use crate::bus::timings;

mod sealed {
    #[doc(hidden)]
    pub trait LcdParallelWriteModeSet<E> {
        fn set_write_mode(&mut self) -> Result<(), E>;
    }

    #[doc(hidden)]
    pub trait LcdParallelReadModeSet<E> {
        fn set_read_mode(&mut self) -> Result<(), E>;
    }
}

pub trait LcdParallelWriteModeSet<E>: sealed::LcdParallelWriteModeSet<E> {}

pub trait LcdParallelReadModeSet<E>: sealed::LcdParallelReadModeSet<E> {}

pub struct LcdParallelWriteOnly;

impl LcdParallelWriteOnly {
    /// This can be used to get a `&mut WriteOnly` when implementing your own pin struct.
    #[inline]
    pub fn new<T>(data: &mut T) -> &mut Self {
        assert_eq!(core::mem::size_of::<Self>(), 0);
        assert_eq!(core::mem::align_of::<Self>(), 1);

        // # Safety
        // WriteOnly is a Zst with no alignment constraints and `&mut T` is never null.
        unsafe { core::mem::transmute(data) }
    }
}

impl<E> sealed::LcdParallelWriteModeSet<E> for LcdParallelWriteOnly {
    fn set_write_mode(&mut self) -> Result<(), E> {
        Ok(())
    }
}

impl<E> LcdParallelWriteModeSet<E> for LcdParallelWriteOnly {}

impl<T: OutputPin> sealed::LcdParallelWriteModeSet<T::Error> for T {
    fn set_write_mode(&mut self) -> Result<(), T::Error> {
        self.set_low()
    }
}

impl<T: OutputPin> LcdParallelWriteModeSet<T::Error> for T {}

impl<T: OutputPin> sealed::LcdParallelReadModeSet<T::Error> for T {
    fn set_read_mode(&mut self) -> Result<(), T::Error> {
        self.set_high()
    }
}

impl<T: OutputPin> LcdParallelReadModeSet<T::Error> for T {}

#[derive(Debug)]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LcdParallelBus<P: LcdParallelPins, T, const WIDTH: u8> {
    pins: P,
    timings: T,
}

impl<P: LcdParallelPins, E> LcdParallelBus<P, timings::DefaultTimingsParallel8, 8>
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
{
    #[inline]
    pub fn new_8bit(pins: P) -> Self {
        Self {
            pins,
            timings: timings::DefaultTimingsParallel8,
        }
    }
}

impl<P: LcdParallelPins, T, E> LcdParallelBus<P, T, 8>
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
{
    #[inline]
    pub fn new_8bit_with_timings<Delay: ?Sized>(pins: P, timings: T) -> Self
    where
        T: timings::LcdTimingsParallel<Delay>,
    {
        Self { pins, timings }
    }
}

impl<P: LcdParallelPins<D0 = Infallible, D1 = Infallible, D2 = Infallible, D3 = Infallible>, E>
    LcdParallelBus<P, timings::DefaultTimingsParallel4, 4>
where
    P::RS: OutputPin<Error = E>,
    P::RW: LcdParallelWriteModeSet<E>,
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
            timings: timings::DefaultTimingsParallel4,
        }
    }
}

impl<P: LcdParallelPins, T, E> LcdParallelBus<P, T, 4>
where
    P::RS: OutputPin<Error = E>,
    P::RW: LcdParallelWriteModeSet<E>,
    P::EN: OutputPin<Error = E>,
    P::D4: OutputPin<Error = E>,
    P::D5: OutputPin<Error = E>,
    P::D6: OutputPin<Error = E>,
    P::D7: OutputPin<Error = E>,
{
    #[inline]
    pub fn new_4bit_with_timings<Delay: ?Sized>(pins: P, timings: T) -> Self
    where
        T: timings::LcdTimingsParallel<Delay>,
    {
        Self { pins, timings }
    }
}

impl<P: LcdParallelPins, T, const WIDTH: u8> LcdParallelBus<P, T, WIDTH> {
    pub fn destroy(self) -> P {
        self.pins
    }
}
