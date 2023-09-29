#![no_std]
#![no_main]

use esp_backtrace as _;
use hal::{
    clock::ClockControl,
    gpio::{Gpio0, Output, PushPull},
    peripherals::Peripherals,
    prelude::*,
    spi::SpiMode,
    Delay, Spi, IO,
};

use embedded_graphics::{pixelcolor::Rgb565, prelude::*, primitives::Rectangle};

// Provides the parallel port and display interface builders
use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics_framebuf::FrameBuf;
use mipidsi::{Builder, ColorInversion};

#[entry]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // Define the Data/Command select pin as a digital output
    let dc = io.pins.gpio2.into_push_pull_output();

    // only spi device
    let _cs = io.pins.gpio10.into_push_pull_output().set_low().unwrap();
    // enable backlight
    let _backlight = io.pins.gpio3.into_push_pull_output().set_high().unwrap();

    // Define the SPI pins and create the SPI interface
    let sck = io.pins.gpio6;
    let mosi = io.pins.gpio7;
    let spi = Spi::new_no_cs_no_miso(
        peripherals.SPI2,
        sck,
        mosi,
        80_u32.MHz(),
        SpiMode::Mode0,
        &clocks,
    );

    // Define the display interface with no chip select
    let di = SPIInterfaceNoCS::new(spi, dc);

    // Define the display from the display interface and initialize it
    let mut display = Builder::gc9a01(di)
        .with_invert_colors(ColorInversion::Inverted)
        .init(&mut delay, Option::<Gpio0<Output<PushPull>>>::None)
        .unwrap();

    let mut data = [embedded_graphics::pixelcolor::Rgb565::BLACK; 240 * 240];
    let mut fbuf = FrameBuf::new(&mut data, 240, 240);

    // Make the display all black
    fbuf.clear(Rgb565::BLACK).unwrap();
    flush(&mut display, &mut fbuf).unwrap();

    loop {
        // Draw a smiley face with white eyes and a red mouth
        // draw_smiley(&mut display).unwrap();
        fbuf.clear(Rgb565::RED).unwrap();
        flush(&mut display, &mut fbuf).unwrap();
        log::info!("RED!");
        delay.delay_ms(500u32);
        fbuf.clear(Rgb565::GREEN).unwrap();
        flush(&mut display, &mut fbuf).unwrap();
        log::info!("GREEN!");
        delay.delay_ms(500u32);
        fbuf.clear(Rgb565::BLUE).unwrap();
        flush(&mut display, &mut fbuf).unwrap();
        log::info!("BLUE!");
        delay.delay_ms(500u32);
    }
}

fn flush<T: DrawTarget<Color = Rgb565>>(
    display: &mut T,
    fbuf: &mut FrameBuf<Rgb565, &mut [Rgb565; 57600]>,
) -> Result<(), T::Error> {
    display.draw_iter(fbuf.into_iter())?;
    Ok(())
}
