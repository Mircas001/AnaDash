use embassy_rp::gpio::Output;
use embassy_rp::peripherals::SPI1;
use embassy_rp::pio_programs::ws2812::Rgb;
use embassy_rp::spi::{Blocking as BlockingSpi, Spi};
use embassy_time::{Delay, Instant};
use embedded_graphics::prelude::*;
use embedded_graphics::{
    image::Image,
    mono_font::{MonoTextStyle, ascii::FONT_6X13, ascii::FONT_8X13_BOLD},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Line, PrimitiveStyle, Rectangle},
    text::{Alignment, Text},
};
use embedded_hal_bus::spi::ExclusiveDevice;
use embedded_iconoir::prelude::*;
use embedded_layout::View;
use embedded_layout::{layout::linear::LinearLayout, prelude::*};
use heapless::String;
use shared::{DashboardData, NotificationData, PlayerStatus};
use st7735_lcd::ST7735;

pub struct Display {
    pub display: ST7735<
        ExclusiveDevice<Spi<'static, SPI1, BlockingSpi>, Output<'static>, Delay>,
        Output<'static>,
        Output<'static>,
    >,
    time: String<10>,
    noti_buffer: NotificationData,
    noti_cooldown: Instant,
    display_area: Rectangle,
}

static StandardStyle: MonoTextStyle<'_, Rgb565> = MonoTextStyle::new(&FONT_6X13, Rgb565::WHITE);
static BoldStyle: MonoTextStyle<'_, Rgb565> = MonoTextStyle::new(&FONT_8X13_BOLD, Rgb565::WHITE);

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

        let time: String<10> = String::try_from("??:??:??").unwrap();

        let noti_cooldown = Instant::now();

        let display_area = display.bounding_box();

        Self {
            display: display,
            time: time,
            noti_cooldown: noti_cooldown,
            noti_buffer: NotificationData::new(),
            display_area: display_area,
        }
    }

    pub fn update_screen(&mut self, dash: DashboardData) {
        self.time = dash.time.clone();
        if Instant::now() > self.noti_cooldown {
            match dash.player_status {
                PlayerStatus::Stopped => {
                    self.display.clear(Rgb565::BLACK);
                    self.draw_status_bar();
                    let hello_text = Text::new("Hello", self.display_area.center(), BoldStyle);
                    hello_text.draw(&mut self.display);
                }
                _ => self.draw_music_player(dash),
            }
        } else {
            self.draw_notification();
        }
    }

    fn draw_status_bar(&mut self) {
        let clock = Text::with_alignment(
            self.time.as_str(),
            Point::new(self.display_area.center().x, 0),
            StandardStyle,
            Alignment::Center,
        );

        let clock_dimensions = clock.bounding_box();
        Line::new(
            Point::new(0, clock_dimensions.size.height as i32),
            Point::new(
                self.display_area.size.width as i32,
                clock_dimensions.size().height as i32,
            ),
        )
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
        .draw(&mut self.display);

        clock.draw(&mut self.display);
    }

    pub fn show_notification(&mut self, noti: NotificationData) {
        self.noti_buffer = noti;
        self.noti_cooldown = Instant::from_secs(5);
        self.draw_notification();
    }

    fn draw_notification(&mut self) {
        self.display.clear(Rgb565::BLACK);
        self.draw_status_bar();
        let icon = icons::size24px::communication::BellNotification::new(Rgb565::WHITE);
        let noti_icon = Image::new(&icon, Point::zero());
        let summary = Text::new(self.noti_buffer.summary.as_str(), Point::zero(), BoldStyle);
        let body = Text::new(self.noti_buffer.body.as_str(), Point::zero(), StandardStyle);
        LinearLayout::vertical(Chain::new(noti_icon).append(summary).append(body))
            .with_alignment(horizontal::Center)
            .arrange()
            .align_to(&self.display_area, horizontal::Center, vertical::Center)
            .draw(&mut self.display)
            .unwrap();
    }

    fn draw_music_player(&mut self, dash: DashboardData) {
        todo!();
    }
}
