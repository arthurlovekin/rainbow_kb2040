//! # Rainbow Example for the Adafruit KB2040
//!
//! Runs a rainbow-effect colour wheel on the on-board LED.
//!
//! Uses the `ws2812_pio` driver to control the LED, which in turns uses the
//! RP2040's PIO block.

#![no_std]
#![no_main]

use adafruit_kb2040::entry;
use fugit::RateExtU32; // Frequency trait for u32.kHz()
use core::iter::once;
use embedded_hal::delay::DelayNs;
use panic_halt as _;

use adafruit_kb2040::{
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        pac,
        pio::PIOExt,
        timer::Timer,
        watchdog::Watchdog,
        Sio,
        i2c::I2C,
    },
    XOSC_CRYSTAL_FREQ,
};
use smart_leds::{brightness, SmartLedsWrite, RGB8};
use ws2812_pio::Ws2812;

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this
/// function as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then the LED, then runs
/// the colour wheel in an infinite loop.
#[entry]
fn main() -> ! {
    // Configure the RP2040 peripherals

    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = Sio::new(pac.SIO);

    let pins = adafruit_kb2040::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // Configure I2C communication to the BNO055
    let mut i2c = I2C::i2c0(
        pac.I2C0,
        pins.a2.reconfigure(), //sda
        pins.a3.reconfigure(), //scl
        400.kHz(),
        &mut pac.RESETS,
        125_000_000.Hz(),
    );

    // // Scan for devices on the bus by attempting to read from them
    // use embedded_hal_0_2::prelude::_embedded_hal_blocking_i2c_Read;
    // for i in 0..=127u8 {
    //     let mut readbuf: [u8; 1] = [0; 1];
    //     let result = i2c.read(i, &mut readbuf);
    //     if let Ok(d) = result {
    //         // Do whatever work you want to do with found devices
    //         // writeln!(uart, "Device found at address{:?}", i).unwrap();
    //     }
    // }

// // Write some data to a device at 0x2c
// use embedded_hal_0_2::prelude::_embedded_hal_blocking_i2c_Write;
// i2c.write(0x2Cu8, &[1, 2, 3]).unwrap();

// // Write and then read from a device at 0x3a
// use embedded_hal_0_2::prelude::_embedded_hal_blocking_i2c_WriteRead;
// let mut readbuf: [u8; 1] = [0; 1];
// i2c.write_read(0x2Cu8, &[1, 2, 3], &mut readbuf).unwrap();

    // Configure the addressable LED
    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

    let mut ws = Ws2812::new(
        pins.neopixel.into_function(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    // Infinite colour wheel loop

    let mut n: u8 = 128;
    let mut timer = timer; // rebind to force a copy of the timer
    loop {
        ws.write(brightness(once(wheel(n)), 32)).unwrap();
        n = n.wrapping_add(1);

        timer.delay_ms(25);

    }
}

/// Convert a number from `0..=255` to an RGB color triplet.
///
/// The colours are a transition from red, to green, to blue and back to red.
fn wheel(mut wheel_pos: u8) -> RGB8 {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        // No green in this sector - red and blue only
        (255 - (wheel_pos * 3), 0, wheel_pos * 3).into()
    } else if wheel_pos < 170 {
        // No red in this sector - green and blue only
        wheel_pos -= 85;
        (0, wheel_pos * 3, 255 - (wheel_pos * 3)).into()
    } else {
        // No blue in this sector - red and green only
        wheel_pos -= 170;
        (wheel_pos * 3, 255 - (wheel_pos * 3), 0).into()
    }
}
