use embassy_rp::gpio::Output;
use embassy_rp::peripherals::SPI1;
use embassy_rp::spi::{Blocking as BlockingSpi, Config as SpiConfig, Spi};
use embassy_time::Delay;
use embedded_hal_bus::spi::ExclusiveDevice;
use st7735_lcd::ST7735;

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
        Self { display: display }
    }
    
}
