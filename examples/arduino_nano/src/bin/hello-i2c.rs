#![no_std]
#![no_main]

use arduino_hal::Delay;
use embedded_hal::delay::DelayNs as _;
use embedded_lcd::{blocking::*, bus::LcdI2c8574Bus, LcdDisplayMode, LcdDriver, LcdDriverOptions};

extern crate panic_halt;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);

    // Setup USB Serial
    let mut serial = arduino_hal::default_serial!(peripherals, pins, 115200);

    let mut delay = Delay::new();

    ufmt::uwriteln!(serial, "Start").unwrap();

    // Configure I2C interface
    let i2c = arduino_hal::I2c::new(
        peripherals.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        100_000,
    );

    // Create i2c bus
    let bus = LcdI2c8574Bus::new(i2c, 0x27);

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
