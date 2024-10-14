#![no_std]
#![no_main]

use arduino_hal::{i2c, Delay};
use embedded_hal::delay::DelayNs as _;
use embedded_lcd::{blocking::*, bus::LcdI2c8574Bus, LcdDisplayMode, LcdDriver};

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

    // Create 8 bit parallel bus
    let bus = LcdI2c8574Bus::new(i2c, 0x27);

    // Initialize LCD driver
    let mut display = LcdDriver::init(
        embedded_lcd::MemoryMap2004::new(),
        embedded_lcd::CharsetA00::QUESTION_FALLBACK,
        bus,
        &mut delay,
    )
    .unwrap();

    display
        .set_display_mode(LcdDisplayMode::SHOW_DISPLAY, &mut delay)
        .unwrap();

    let display = &mut display as &mut dyn WritableLcd;

    const HELLO_WORLDS: &[&str] = &[
        "Hello, world!",
        "Hallo, Welt!",
        "Bonjour a tous!",
        "Hallo, wereld!",
        "ハロー、ワールト゛！",
    ];

    for hello in HELLO_WORLDS.iter().cycle() {
        show_message(display, hello, &mut delay);
    }
    unreachable!()
}

trait WritableLcd:
    BlockingLcdDriver<Delay, Error = i2c::Error> + BlockingLcdWrite<Delay, Error = i2c::Error>
{
}

impl<T> WritableLcd for T where
    T: BlockingLcdDriver<Delay, Error = i2c::Error> + BlockingLcdWrite<Delay, Error = i2c::Error>
{
}

#[inline(never)]
fn show_message(display: &mut dyn WritableLcd, message: &str, delay: &mut Delay) {
    display.clear(delay).unwrap();
    display.return_home(delay).unwrap();
    display.write_str(&message, delay).unwrap();
    delay.delay_ms(2000);
}
