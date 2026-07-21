use crate::usb_handler::CDC_CHANNEL;
use defmt::*;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::Driver;
use embassy_usb::class::cdc_acm::CdcAcmClass;
use postcard::accumulator::{CobsAccumulator, FeedResult};
use shared::HostTransmission;

const MAX_MSG_SIZE: usize = 256; // * Just to prevent in case someone injects garbage or something;

#[embassy_executor::task]
pub async fn cdc_task(mut cdc: CdcAcmClass<'static, Driver<'static, USB>>) {
    // it gets data from the host and sends them over to main, either being an ST7735 or mcp4728, probably main?
    // I have no idea how to get data out, ill just get data in for now
    // * Also, since this is USB powered, we don't care about disconnection, cuz we'll just shut down
    // * But i'll folow the embassy example of doing the

    let mut data_buf: CobsAccumulator<MAX_MSG_SIZE> = CobsAccumulator::new();
    let mut packet_buf = [0u8; 64]; // * Stores the package

    loop {
        cdc.wait_connection().await;
        info!("CDC Connected!");
        loop {
            let packet = match cdc.read_packet(&mut packet_buf).await {
                Ok(packet) => packet,
                Err(e) => {
                    error!(":( Error with the USB CDC connection!: {}", e);
                    break;
                }
            };

            let mut packet_slice = &packet_buf[..packet];

            'cobs: while !packet_slice.is_empty() {
                // * This puts the slice in the data buffer, and when it's full, sends over to the main part
                packet_slice = match data_buf.feed::<HostTransmission>(packet_slice) {
                    FeedResult::Consumed => break 'cobs,
                    FeedResult::OverFull(remaining) => remaining,
                    FeedResult::DeserError(remaining) => remaining,
                    FeedResult::Success { data, remaining } => {
                        CDC_CHANNEL.send(data).await;
                        remaining
                    }
                }
            }
        }
    }
}
