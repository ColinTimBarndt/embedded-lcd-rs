use core::convert::Infallible;

use super::LcdParallelWriteOnly;

pub trait LcdParallelPins {
    type RS;
    type RW;
    type EN;
    type D0;
    type D1;
    type D2;
    type D3;
    type D4;
    type D5;
    type D6;
    type D7;

    fn rs(&mut self) -> &mut Self::RS;
    fn rw(&mut self) -> &mut Self::RW;
    fn en(&mut self) -> &mut Self::EN;
    fn d0(&mut self) -> &mut Self::D0;
    fn d1(&mut self) -> &mut Self::D1;
    fn d2(&mut self) -> &mut Self::D2;
    fn d3(&mut self) -> &mut Self::D3;
    fn d4(&mut self) -> &mut Self::D4;
    fn d5(&mut self) -> &mut Self::D5;
    fn d6(&mut self) -> &mut Self::D6;
    fn d7(&mut self) -> &mut Self::D7;
}

#[derive(Debug)]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LcdParallelPinsRW8<RS, RW, EN, D0, D1, D2, D3, D4, D5, D6, D7> {
    pub rs: RS,
    pub rw: RW,
    pub en: EN,
    pub d0: D0,
    pub d1: D1,
    pub d2: D2,
    pub d3: D3,
    pub d4: D4,
    pub d5: D5,
    pub d6: D6,
    pub d7: D7,
}

impl<RS, RW, EN, D0, D1, D2, D3, D4, D5, D6, D7> LcdParallelPins
    for LcdParallelPinsRW8<RS, RW, EN, D0, D1, D2, D3, D4, D5, D6, D7>
{
    type RS = RS;
    type RW = RW;
    type EN = EN;
    type D0 = D0;
    type D1 = D1;
    type D2 = D2;
    type D3 = D3;
    type D4 = D4;
    type D5 = D5;
    type D6 = D6;
    type D7 = D7;

    fn rs(&mut self) -> &mut Self::RS {
        &mut self.rs
    }
    fn rw(&mut self) -> &mut Self::RW {
        &mut self.rw
    }
    fn en(&mut self) -> &mut Self::EN {
        &mut self.en
    }
    fn d0(&mut self) -> &mut Self::D0 {
        &mut self.d0
    }
    fn d1(&mut self) -> &mut Self::D1 {
        &mut self.d1
    }
    fn d2(&mut self) -> &mut Self::D2 {
        &mut self.d2
    }
    fn d3(&mut self) -> &mut Self::D3 {
        &mut self.d3
    }
    fn d4(&mut self) -> &mut Self::D4 {
        &mut self.d4
    }
    fn d5(&mut self) -> &mut Self::D5 {
        &mut self.d5
    }
    fn d6(&mut self) -> &mut Self::D6 {
        &mut self.d6
    }
    fn d7(&mut self) -> &mut Self::D7 {
        &mut self.d7
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LcdParallelPinsW8<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7> {
    pub rs: RS,
    pub en: EN,
    pub d0: D0,
    pub d1: D1,
    pub d2: D2,
    pub d3: D3,
    pub d4: D4,
    pub d5: D5,
    pub d6: D6,
    pub d7: D7,
}

impl<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7> LcdParallelPins
    for LcdParallelPinsW8<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>
{
    type RS = RS;
    type RW = LcdParallelWriteOnly;
    type EN = EN;
    type D0 = D0;
    type D1 = D1;
    type D2 = D2;
    type D3 = D3;
    type D4 = D4;
    type D5 = D5;
    type D6 = D6;
    type D7 = D7;

    fn rs(&mut self) -> &mut Self::RS {
        &mut self.rs
    }
    fn rw(&mut self) -> &mut Self::RW {
        LcdParallelWriteOnly::new(self)
    }
    fn en(&mut self) -> &mut Self::EN {
        &mut self.en
    }
    fn d0(&mut self) -> &mut Self::D0 {
        &mut self.d0
    }
    fn d1(&mut self) -> &mut Self::D1 {
        &mut self.d1
    }
    fn d2(&mut self) -> &mut Self::D2 {
        &mut self.d2
    }
    fn d3(&mut self) -> &mut Self::D3 {
        &mut self.d3
    }
    fn d4(&mut self) -> &mut Self::D4 {
        &mut self.d4
    }
    fn d5(&mut self) -> &mut Self::D5 {
        &mut self.d5
    }
    fn d6(&mut self) -> &mut Self::D6 {
        &mut self.d6
    }
    fn d7(&mut self) -> &mut Self::D7 {
        &mut self.d7
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LcdParallelPinsRW4<RS, RW, EN, D4, D5, D6, D7> {
    pub rs: RS,
    pub rw: RW,
    pub en: EN,
    pub d4: D4,
    pub d5: D5,
    pub d6: D6,
    pub d7: D7,
}

impl<RS, RW, EN, D4, D5, D6, D7> LcdParallelPins
    for LcdParallelPinsRW4<RS, RW, EN, D4, D5, D6, D7>
{
    type RS = RS;
    type RW = RW;
    type EN = EN;
    type D0 = Infallible;
    type D1 = Infallible;
    type D2 = Infallible;
    type D3 = Infallible;
    type D4 = D4;
    type D5 = D5;
    type D6 = D6;
    type D7 = D7;

    fn rs(&mut self) -> &mut Self::RS {
        &mut self.rs
    }
    fn rw(&mut self) -> &mut Self::RW {
        &mut self.rw
    }
    fn en(&mut self) -> &mut Self::EN {
        &mut self.en
    }
    fn d0(&mut self) -> &mut Self::D0 {
        panic!()
    }
    fn d1(&mut self) -> &mut Self::D1 {
        panic!()
    }
    fn d2(&mut self) -> &mut Self::D2 {
        panic!()
    }
    fn d3(&mut self) -> &mut Self::D3 {
        panic!()
    }
    fn d4(&mut self) -> &mut Self::D4 {
        &mut self.d4
    }
    fn d5(&mut self) -> &mut Self::D5 {
        &mut self.d5
    }
    fn d6(&mut self) -> &mut Self::D6 {
        &mut self.d6
    }
    fn d7(&mut self) -> &mut Self::D7 {
        &mut self.d7
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LcdParallelPinsW4<RS, EN, D4, D5, D6, D7> {
    pub rs: RS,
    pub en: EN,
    pub d4: D4,
    pub d5: D5,
    pub d6: D6,
    pub d7: D7,
}

impl<RS, EN, D4, D5, D6, D7> LcdParallelPins for LcdParallelPinsW4<RS, EN, D4, D5, D6, D7> {
    type RS = RS;
    type RW = LcdParallelWriteOnly;
    type EN = EN;
    type D0 = Infallible;
    type D1 = Infallible;
    type D2 = Infallible;
    type D3 = Infallible;
    type D4 = D4;
    type D5 = D5;
    type D6 = D6;
    type D7 = D7;

    fn rs(&mut self) -> &mut Self::RS {
        &mut self.rs
    }
    fn rw(&mut self) -> &mut Self::RW {
        LcdParallelWriteOnly::new(self)
    }
    fn en(&mut self) -> &mut Self::EN {
        &mut self.en
    }
    fn d0(&mut self) -> &mut Self::D0 {
        panic!()
    }
    fn d1(&mut self) -> &mut Self::D1 {
        panic!()
    }
    fn d2(&mut self) -> &mut Self::D2 {
        panic!()
    }
    fn d3(&mut self) -> &mut Self::D3 {
        panic!()
    }
    fn d4(&mut self) -> &mut Self::D4 {
        &mut self.d4
    }
    fn d5(&mut self) -> &mut Self::D5 {
        &mut self.d5
    }
    fn d6(&mut self) -> &mut Self::D6 {
        &mut self.d6
    }
    fn d7(&mut self) -> &mut Self::D7 {
        &mut self.d7
    }
}
