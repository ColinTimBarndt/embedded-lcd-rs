#![no_std]
#![no_main]

use arduino_hal::Delay;
use embedded_hal::delay::DelayNs as _;
use embedded_lcd::{
    blocking::*,
    bus::{LcdParallelBus, LcdParallelPinsW8},
    LcdDisplayMode, LcdDriver, LcdDriverOptions,
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

    // Create 8 bit parallel bus
    let bus = LcdParallelBus::new_8bit(LcdParallelPinsW8 {
        rs: pins.d12.into_output(),
        en: pins.d11.into_output(),
        d0: pins.d10.into_opendrain(),
        d1: pins.d9.into_opendrain(),
        d2: pins.d8.into_opendrain(),
        d3: pins.d7.into_opendrain(),
        d4: pins.d6.into_opendrain(),
        d5: pins.d5.into_opendrain(),
        d6: pins.d4.into_opendrain(),
        d7: pins.d3.into_opendrain(),
    });

    // Initialize LCD driver
    let mut display = LcdDriver::init(
        LcdDriverOptions::new(bus, embedded_lcd::MemoryMap1602::new())
            .with_charset(embedded_lcd::CharsetA00::QUESTION_FALLBACK),
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
