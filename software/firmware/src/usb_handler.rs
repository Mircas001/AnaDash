use crate::Irqs;
use crate::hardware::input_handler::KeyInputs;
use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::Peri;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::Driver;
use embassy_usb::class::cdc_acm::{CdcAcmClass, State as CdcState};
use embassy_usb::class::hid::{HidBootProtocol, HidReaderWriter, HidSubclass, State as HidState};
use static_cell::StaticCell;
use embassy_sync::channel::Channel;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, SerializedDescriptor};
use {defmt_rtt as _, panic_probe as _};
use shared::HostTransmission;

mod input_task;
mod cdc_task;

/*
 * This is quite new territory for me, so i'm commenting everything so I remember and understand it later
 * By the way, for clarification:
 * The macropad uses an generic boot keyboard to send between f13-f21
 * and an media key descriptor to send volume commands...
 * Meanwhile the dashboard data will be sent over CDC, as an shared struct with postcard sliced with COBS
*/

pub static CDC_CHANNEL: Channel<ThreadModeRawMutex, HostTransmission, 4> = Channel::new();

pub fn begin_usb_handler(
    spawner: &Spawner,
    usb: Peri<'static, USB>,
    input_keys: KeyInputs<'static>,
) {
    info!("Creating USB driver...");
    // * This creates the driver, and configurates the information, such as who made it, the product name, the power etc
    let driver = Driver::new(usb, Irqs);
    let mut config = embassy_usb::Config::new(0x1209, 0x4da5); // TODO: Figure out new VID and PID
    config.manufacturer = Some("MatheusM");
    config.product = Some("AnaDash");
    config.serial_number = Some("001");
    config.max_power = 100; // can I increase this?
    config.max_packet_size_0 = 64; // can i increase this?

    /*
        * On static cells:
        * This may be the first time of seeing static, but basically, static variables live forever.
        * The USB needs those to live forever in order to work, and it needs to be acessed outside of this function.
        * To do this we use an static variable
        * And we chose the static cells to prevent unsavoury conditions related to having a variable anyone can mess around it
        * Static cell creates an fixed space that lives forever, stored on the binary. It's loaded into memory when you start the program
        * It also makes sure you can only init it once, so you dont zero out the variable someone is using
        * So, in short, static cells are an special type of variable that gives you more safety over an static mut
        --------------------------------------------------------------------------------------------------------
        * On [u8, 256]:
        * This is basically an array of 256 8-bit integers
    */

    static CONFIG_DESC: StaticCell<[u8; 256]> = StaticCell::new(); // * The builder sets up an USB descriptor, basically the resumé of the USB device
    static BOS_DESC: StaticCell<[u8; 256]> = StaticCell::new(); // * Binary Object Descriptor, sets up extra resources used by USB 2.1+, won't be used but builder require sit 
    static MSOS_DESC: StaticCell<[u8; 256]> = StaticCell::new(); // * This one is an special thing for the Windows to recognize the usb device, i'm only putting because it's required
    static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new(); // * This is an buffer for the USB to talk
    static CDC_STATE: StaticCell<CdcState> = StaticCell::new(); // * This is the shared data between the builder stack and the CDC stack, think of it as... bulletin board? i guess
    static MACRO_KBD_STATE: StaticCell<HidState> = StaticCell::new(); // * Same as above ^^
    static MEDIA_KBD_STATE: StaticCell<HidState> = StaticCell::new();

    // * I'm not explaining everything again, you get the idea
    let config_desc = CONFIG_DESC.init([0; 256]);
    let bos_desc = BOS_DESC.init([0; 256]);
    let msos_desc = MSOS_DESC.init([0; 256]);
    let control_buf = CONTROL_BUF.init([0; 64]);
    let cdc_state = CDC_STATE.init(CdcState::new());
    let macro_kbd_state = MACRO_KBD_STATE.init(HidState::new());
    let media_kbd_state = MEDIA_KBD_STATE.init(HidState::new());

    /*
     * This creates the builder object, I imagine it as this sheet of paper that's getting all the USB configurations and classes
     * Written down on it, and then passed to the USB object
     */
    let mut builder = embassy_usb::Builder::new(
        driver,
        config,
        config_desc,
        bos_desc,
        msos_desc,
        control_buf,
    );

    // * This is the CDC class, the serial interface, sending the dashboard data over,
    let host_cdc = CdcAcmClass::new(&mut builder, cdc_state, 64);

    let macro_kbd_config = embassy_usb::class::hid::Config {
        report_descriptor: KeyboardReport::desc(), // * This describes how the HID talks to the PC
        request_handler: None, // * This handles the requests made by the host, we are ignoring it
        poll_ms: 10,           // * Specifies the frequency the host should ask for new input
        max_packet_size: 64, // * The max packet size, think of it as the maximum amount the USB protocol can chew
        hid_subclass: HidSubclass::Boot, // * Basically, to allow BIOSes to use this keyboard, this just says "hey, I'm generic somethign"
        hid_boot_protocol: HidBootProtocol::Keyboard, // * This specifies that the something is an keyboard, so basically, an keyboard like any other
                                                      // * yeah sure, we aren't true keyboards, just to the eyes of the BIOS
    };
    let macro_hid =
        HidReaderWriter::<_, 1, 8>::new(&mut builder, macro_kbd_state, macro_kbd_config);

    let (macro_reader, macro_writer) = macro_hid.split();
    drop(macro_reader); // * trying to save up RAM
    /*
     * This line creates the HID object, with the USB driver (inferred by rust), with a 1 byte large read buffer,
     * and an 8 byte large writer buffer. Passing the builder object, the state struct and the configuration
     * I wish there was a better way to break this down but I'm limited by formatting
     */

    // * For this one you get the idea
    let media_kbd_config = embassy_usb::class::hid::Config {
        report_descriptor: MediaKeyboardReport::desc(), // * Only note that this time, we are using an media descriptor, the one your keyboard uses to raise the volume!
        request_handler: None,
        poll_ms: 10,
        max_packet_size: 8,
        hid_subclass: HidSubclass::No, // * And no boot protocols!
        hid_boot_protocol: HidBootProtocol::None,
    };
    let media_hid =
        HidReaderWriter::<_, 1, 8>::new(&mut builder, media_kbd_state, media_kbd_config);
    let (media_reader, media_writer) = media_hid.split();
    drop(media_reader); // * trying to save up RAM

    info!("Building the USB device");
    let usb = builder.build(); // * Finally, we make the usb device

    spawner.spawn(input_task::input_task(macro_writer, media_writer, input_keys).unwrap());
    spawner.spawn(cdc_task::cdc_task(host_cdc).unwrap());
}
