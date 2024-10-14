use crate::{CharsetUniversal, DisplayMemoryMap, EmptyFallback};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LcdDriverOptions<B, M: DisplayMemoryMap, C> {
    pub bus: B,
    pub memory_map: M,
    pub charset: C,
    pub font: LcdFontMode,
}

impl<B, M: DisplayMemoryMap> LcdDriverOptions<B, M, EmptyFallback<CharsetUniversal>> {
    #[inline]
    pub fn new(bus: B, memory_map: M) -> Self {
        Self {
            bus,
            memory_map,
            charset: CharsetUniversal::EMPTY_FALLBACK,
            font: LcdFontMode::Font5x8,
        }
    }
}

impl<B, M: DisplayMemoryMap, C> LcdDriverOptions<B, M, C> {
    pub fn with_charset<C2>(self, charset: C2) -> LcdDriverOptions<B, M, C2> {
        LcdDriverOptions {
            bus: self.bus,
            memory_map: self.memory_map,
            charset,
            font: self.font,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LcdFontMode {
    #[default]
    Font5x8,
    Font5x10,
}
