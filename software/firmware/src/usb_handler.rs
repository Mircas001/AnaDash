use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler as UsbIrq};
use embassy_time::Timer;
use embassy_usb::UsbDevice;
use embassy_usb::class::cdc_acm::{CdcAcmClass, State as CdcState};
use embassy_usb::class::hid::{HidReaderWriter, ReportId, RequestHandler, State as HidState};
use embassy_usb::control::OutResponse;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};
