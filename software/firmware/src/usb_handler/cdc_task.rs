use embassy_rp::peripherals::USB;
use embassy_rp::usb::Driver;
use embassy_usb::class::cdc_acm::CdcAcmClass;

#[embassy_executor::task]
pub async fn cdc_task(mut cdc: CdcAcmClass<'static, Driver<'static, USB>>) {
    // it gets data from the host and sends them over to main, either being an ST7735 or mcp4728, probably main?
}
