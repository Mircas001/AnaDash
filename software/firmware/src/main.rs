#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler as UsbIrq};
use embassy_time::Timer;
use embassy_usb::UsbDevice;
use embassy_usb::class::cdc_acm::{CdcAcmClass, State as CdcState};
use embassy_usb::class::hid::{
    HidBootProtocol, HidProtocolMode, HidReaderWriter, HidSubclass, ReportId, RequestHandler,
    State as HidState,
};
use embassy_usb::control::OutResponse;
use static_cell::StaticCell;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};
use {defmt_rtt as _, panic_probe as _};

mod usb_handler;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => UsbIrq<USB>;
});
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    info!("Hello!");

    // * Starting USB here cause I haven't figured out how to put it in an external file
    // TODO: Figure how to put it in another file
    // FIXME: The code's a mess, put this USB stuff elsewhere
    // FIXME: The cells dont live long enough

    let driver = Driver::new(p.USB, Irqs);
    let mut config = embassy_usb::Config::new(0xa1e5, 0xdd95);
    config.manufacturer = Some("MatheusM");
    config.product = Some("AnaDash");
    config.serial_number = Some("001");
    config.max_power = 100; // can I increase this?
    config.max_packet_size_0 = 64; // can i increase this?

    static CDC_STATE: StaticCell<CdcState> = StaticCell::new(); // TODO: Replace these
    static HID_STATE: StaticCell<HidState> = StaticCell::new();

    let mut config_desc = [0; 256];
    let mut bos_desc = [0; 256];
    let mut msos_descriptor = [0; 256]; // * only putting this because the builder REQUIRES it
    let mut control_buf = [0; 64];
    let cdc_state = CDC_STATE.init(CdcState::new());
    let hid_state = HID_STATE.init(HidState::new());

    let mut builder = embassy_usb::Builder::new(
        driver,
        config,
        &mut config_desc,
        &mut bos_desc,
        &mut msos_descriptor,
        &mut control_buf,
    );

    let cdc = CdcAcmClass::new(&mut builder, cdc_state, 64);

    // FIXME: Figure out what an hid_boot_protocol and hid_subclass is

    let hid_config = embassy_usb::class::hid::Config {
        report_descriptor: KeyboardReport::desc(),
        request_handler: None,
        poll_ms: 10,
        max_packet_size: 64,
        hid_subclass: HidSubclass::Boot,
        hid_boot_protocol: HidBootProtocol::Keyboard,
    };
    let hid = HidReaderWriter::<_, 1, 8>::new(&mut builder, hid_state, hid_config);

    let usb = builder.build();
    loop {}
}
