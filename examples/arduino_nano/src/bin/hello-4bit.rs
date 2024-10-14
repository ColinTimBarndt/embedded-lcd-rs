#![no_std]
#![no_main]

use arduino_hal::Delay;
use embedded_hal::delay::DelayNs as _;
use embedded_lcd::{
    blocking::*,
    bus::{LcdParallelBus, LcdParallelPinsW4},
    LcdDisplayMode, LcdDriver,
};

extern crate panic_halt;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);

    // Setup USB Serial
    let mut serial = arduino_hal::default_serial!(peripherals, pins, 115200);

    let mut delay = Delay::new();

    ufmt::uwriteln!(serial, "Start").unwrap();

    // Create 4 bit parallel bus
    let bus = LcdParallelBus::new_4bit(LcdParallelPinsW4 {
        rs: pins.d12.into_output(),
        en: pins.d11.into_output(),
        d4: pins.d6.into_opendrain(),
        d5: pins.d5.into_opendrain(),
        d6: pins.d4.into_opendrain(),
        d7: pins.d3.into_opendrain(),
    });

    // Initialize LCD driver
    let mut display = LcdDriver::init(
        embedded_lcd::MemoryMap1602::new(),          // 16x2 LCD
        embedded_lcd::CharsetA00::QUESTION_FALLBACK, // ASCII + Japanese
        bus,
        &mut delay,
    )
    .unwrap();

    display
        .set_display_mode(LcdDisplayMode::SHOW_DISPLAY, &mut delay)
        .unwrap();

    const HELLO_WORLDS: &[&str] = &[
        "Hello, world!",
        "Hallo, Welt!",
        "Bonjour a tous!",
        "Hallo, wereld!",
        "ハロー、ワールト゛！",
    ];

    for hello in HELLO_WORLDS.iter().cycle() {
        display.clear(&mut delay).unwrap();
        display.return_home(&mut delay).unwrap();
        display.write_str(&hello, &mut delay).unwrap();
        delay.delay_ms(2000);
    }
    unreachable!()
}
