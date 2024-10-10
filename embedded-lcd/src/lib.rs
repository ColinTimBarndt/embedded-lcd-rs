#![no_std]

pub mod bus;

mod driver;
pub use driver::*;

mod charset;
pub use charset::*;

mod memory_map;
pub use memory_map::*;

const CLEAR_DISPLAY: u8 = 0x01;
const RETURN_HOME: u8 = 0x02;
const ENTRY_MODE_SET: u8 = 0x04;
const DISPLAY_CONTROL: u8 = 0x08;
const CURSOR_SHIFT: u8 = 0x10;
const FUNCTION_SET: u8 = 0x20;
const SET_CGRAM_ADDRESS: u8 = 0x40;
const SET_DDRAM_ADDRESS: u8 = 0x80;
