use embassy_rp::gpio::Output;
use embassy_rp::peripherals::SPI1;
use embassy_rp::spi::{Blocking as BlockingSpi, Config as SpiConfig, Spi};
use embassy_time::Delay;
use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_8X13},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
    text::{Alignment, Text},
};
use embedded_hal_bus::spi::ExclusiveDevice;
use embedded_layout::View;
use heapless::String;
use st7735_lcd::ST7735;
use tinybmp::CompressionMethod::Rgb;

pub struct Display {
    pub display: ST7735<
        ExclusiveDevice<Spi<'static, SPI1, BlockingSpi>, Output<'static>, Delay>,
        Output<'static>,
        Output<'static>,
    >,
}

impl Display {
    pub fn new(
        spi: Spi<'static, SPI1, BlockingSpi>,
        cs: Output<'static>,
        rst: Output<'static>,
        dc: Output<'static>,
    ) -> Self {
        let display_spi = ExclusiveDevice::new(spi, cs, Delay).unwrap();
        let mut display = ST7735::new(display_spi, dc, rst, true, false, 160, 180);
        display.init(&mut Delay);
        Self { display: display }
    }
    pub fn draw_status_bar(&mut self, time: String<10>) {
        let mut clock = Text::with_alignment(
            time.as_str(),
            Point::new(self.display.bounding_box().center().x, 0),
            MonoTextStyle::new(&FONT_8X13, Rgb565::WHITE),
            Alignment::Center,
        );

        let center = self.display.bounding_box().center().x_axis();

        let clock_dimensions = clock.bounding_box();
        Line::new(
            Point::new(0, clock_dimensions.size.height as i32),
            Point::new(
                self.display.bounding_box().size.width as i32,
                clock_dimensions.size().height as i32,
            ),
        )
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
        .draw(&mut self.display);

        clock.draw(&mut self.display);
    }
}
