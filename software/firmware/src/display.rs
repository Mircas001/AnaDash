use embassy_rp::gpio::Output;
use embassy_rp::peripherals::SPI1;
use embassy_rp::spi::{Blocking as BlockingSpi, Spi};
use embassy_time::{Delay, Instant};
use embedded_graphics::prelude::*;
use embedded_graphics::{
    image::Image,
    mono_font::{MonoTextStyle, ascii::FONT_5X7, ascii::FONT_7X13, ascii::FONT_9X15_BOLD},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Line, PrimitiveStyle, Rectangle},
    text::Text,
};
use embedded_hal_bus::spi::ExclusiveDevice;
use embedded_icon::prelude::*;
use embedded_layout::View;
use embedded_layout::{layout::linear::LinearLayout, prelude::*};
use heapless::String;
use shared::{DashboardData, NotificationData, PlayerStatus, duration_to_string};
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

static STANDARD_STYLE: MonoTextStyle<'_, Rgb565> = MonoTextStyle::new(&FONT_7X13, Rgb565::WHITE);
static BOLD_STYLE: MonoTextStyle<'_, Rgb565> = MonoTextStyle::new(&FONT_9X15_BOLD, Rgb565::WHITE);
static MINOR_STYLE: MonoTextStyle<'_, Rgb565> = MonoTextStyle::new(&FONT_5X7, Rgb565::WHITE);

impl Display {
    pub fn new(
        display_spi: ExclusiveDevice<Spi<'static, SPI1, BlockingSpi>, Output<'static>, Delay>,
        rst: Output<'static>,
        dc: Output<'static>,
    ) -> Self {
        let mut display = ST7735::new(display_spi, dc, rst, true, false, 160, 180);
        if display.init(&mut Delay).is_err() {
            if display.init(&mut Delay).is_err() {
                defmt::error!("Screen error!");
            }
        }

        let time: String<10> = String::try_from("??:??:??").unwrap_or_default();

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
                    if self.display.clear(Rgb565::BLACK).is_err() {
                        if self.display.clear(Rgb565::BLACK).is_err() {
                            return;
                        };
                    };
                    self.draw_status_bar();
                    let hello_text = Text::new("Hello", self.display_area.center(), BOLD_STYLE);
                    hello_text.draw(&mut self.display).unwrap_or_default();
                }
                _ => self.draw_music_player(dash),
            }
        } else {
            self.draw_notification();
        }
    }

    fn draw_status_bar(&mut self) {
        let clock = Text::new(
            self.time.as_str(),
            Point::new(self.display_area.center().x, 0),
            STANDARD_STYLE,
        );
        let status_line = Line::new(
            Point::zero(),
            Point::new(self.display_area.size.width as i32, 0),
        )
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1));

        LinearLayout::vertical(Chain::new(clock).append(status_line))
            .with_alignment(horizontal::Center)
            .arrange()
            .align_to(&self.display_area, horizontal::Center, vertical::Top)
            .draw(&mut self.display)
            .unwrap_or_default();
    }

    pub fn show_notification(&mut self, noti: NotificationData) {
        self.noti_buffer = noti;
        self.noti_cooldown = Instant::from_secs(5);
        self.draw_notification();
    }

    fn draw_notification(&mut self) {
        if self.display.clear(Rgb565::BLACK).is_err() {
            if self.display.clear(Rgb565::BLACK).is_err() {
                return;
            };
        };
        self.draw_status_bar();
        let icon = icons::iconoir::size32px::BellNotification::new(Rgb565::WHITE);
        let noti_icon = Image::new(&icon, Point::zero());
        let summary = Text::new(self.noti_buffer.summary.as_str(), Point::zero(), BOLD_STYLE);
        let body = Text::new(
            self.noti_buffer.body.as_str(),
            Point::zero(),
            STANDARD_STYLE,
        );
        LinearLayout::vertical(Chain::new(noti_icon).append(summary).append(body))
            .with_alignment(horizontal::Center)
            .arrange()
            .align_to(&self.display_area, horizontal::Center, vertical::Center)
            .draw(&mut self.display)
            .unwrap_or_default();
    }

    fn draw_music_player(&mut self, dash: DashboardData) {
        self.draw_status_bar();
        let title = Text::new(dash.title.as_str(), Point::zero(), BOLD_STYLE);
        let artist = Text::new(dash.artist.as_str(), Point::zero(), STANDARD_STYLE);
        let duration_string: String<20> = heapless::format!(
            "[{}/{}]",
            duration_to_string(dash.progress),
            duration_to_string(dash.duration)
        )
        .unwrap_or_default();
        let duration = Text::new(duration_string.as_str(), Point::zero(), MINOR_STYLE);
        let player_status = StatusIcon::new(dash.player_status);
        LinearLayout::vertical(
            Chain::new(title)
                .append(artist)
                .append(player_status)
                .append(duration),
        )
        .with_alignment(horizontal::Center)
        .arrange()
        .align_to(&self.display_area, horizontal::Center, vertical::Center)
        .draw(&mut self.display)
        .unwrap_or_default();
    }
}

// * We use an struct to deal with the 3 icon possibilities

struct StatusIcon {
    status: shared::PlayerStatus,
    position: Point,
}

impl StatusIcon {
    fn new(status: PlayerStatus) -> Self {
        Self {
            status,
            position: Point::zero(),
        }
    }
}

impl Drawable for StatusIcon {
    type Color = Rgb565;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        match self.status {
            PlayerStatus::Playing => {
                let icon = icons::iconoir::size24px::Play::new(Rgb565::WHITE);
                Image::new(&icon, self.position).draw(target)
            }
            PlayerStatus::Stopped => {
                let icon = icons::iconoir::size24px::Square::new(Rgb565::WHITE);
                Image::new(&icon, self.position).draw(target)
            }
            PlayerStatus::Paused => {
                let icon = icons::iconoir::size24px::Pause::new(Rgb565::WHITE);
                Image::new(&icon, self.position).draw(target)
            }
        }
    }
}

impl View for StatusIcon {
    fn translate_impl(&mut self, by: Point) {
        self.position += by;
    }

    fn bounds(&self) -> Rectangle {
        // * All icons are 24x24px lmao
        Rectangle::new(self.position, Size::new(24, 24))
    }
}
