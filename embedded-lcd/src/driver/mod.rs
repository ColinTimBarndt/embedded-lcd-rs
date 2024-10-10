use core::fmt::Debug;

use bitflags::bitflags;

pub mod blocking;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LcdDriver<B, M, C> {
    bus: B,
    memory_map: M,
    charset: C,
    display_mode: LcdDisplayMode,
}

impl<B, M, C> LcdDriver<B, M, C> {
    pub fn memory_map(&self) -> &M {
        &self.memory_map
    }

    pub fn charset(&self) -> &C {
        &self.charset
    }
}

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct LcdStatus: u8 {
        const ADDRESS = 0x7f;
        const BUSY = 0x80;
    }
}

impl LcdStatus {
    #[inline]
    pub const fn address(&self) -> u8 {
        self.intersection(Self::ADDRESS).bits()
    }

    #[inline]
    pub const fn busy(&self) -> bool {
        self.intersects(Self::BUSY)
    }
}

impl core::fmt::Debug for LcdStatus {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.debug_struct("LcdStatus")
            .field("address", &self.address())
            .field("busy", &self.busy())
            .finish()
    }
}

#[cfg(feature = "ufmt")]
impl ufmt::uDebug for LcdStatus {
    fn fmt<W>(&self, fmt: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        fmt.debug_struct("LcdStatus")?
            .field("address", &self.address())?
            .field("busy", &self.busy())?
            .finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for LcdStatus {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "LcdStatus {{ address: {}, busy: {} }}",
            self.address(),
            self.busy()
        )
    }
}

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct LcdDisplayMode: u8 {
        /// The character indicated by the cursor is blinking.
        const SHOW_CURSOR_POSITION = 1;
        /// Cursor is visible.
        const SHOW_CURSOR = 2;
        /// Display on.
        const SHOW_DISPLAY = 4;
    }
}

impl LcdDisplayMode {
    pub const fn show_cursor_position(&self) -> bool {
        self.intersects(Self::SHOW_CURSOR_POSITION)
    }

    pub const fn show_cursor(&self) -> bool {
        self.intersects(Self::SHOW_CURSOR)
    }

    pub const fn show_display(&self) -> bool {
        self.intersects(Self::SHOW_DISPLAY)
    }
}

impl Debug for LcdDisplayMode {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.debug_struct("LcdDisplayMode")
            .field("show_cursor_position", &self.show_cursor_position())
            .field("show_cursor", &self.show_cursor())
            .field("show_display", &self.show_display())
            .finish()
    }
}

#[cfg(feature = "ufmt")]
impl ufmt::uDebug for LcdDisplayMode {
    fn fmt<W>(&self, fmt: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        fmt.debug_struct("LcdDisplayMode")?
            .field("show_cursor_position", &self.show_cursor_position())?
            .field("show_cursor", &self.show_cursor())?
            .field("show_display", &self.show_display())?
            .finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for LcdDisplayMode {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "LcdDisplayMode {{ show_cursor_position: {}, show_cursor: {}, show_display: {} }}",
            self.show_cursor_position(),
            self.show_cursor(),
            self.show_display()
        )
    }
}

bitflags! {
    pub struct LcdEntryMode: u8 {
        /// Shifts the entire display either to the right (INCREMENT = 0) or to
        /// the left (INCREMENT = 1) when SHIFT is 1. The display does not shift
        /// if SHIFT is 0.
        const SHIFT = 1;
        /// Increments DDRAM address by 1 when set, decrements otherwise.
        const INCREMENT = 2;
    }
}

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct LcdFunctionMode: u8 {
        /// 0: 5x8 dots
        /// 1: 5x10 dots
        const FONT = 0x04;
        /// 0: 1 line
        /// 1: 2 lines
        const DISPLAY_LINES = 0x08;
        /// 0: 4 bits
        /// 1: 8 bits
        const DATA_LENGTH = 0x10;
    }
}
