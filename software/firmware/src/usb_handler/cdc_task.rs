use embassy_rp::peripherals::USB;
use embassy_rp::usb::Driver;
use embassy_usb::class::cdc_acm::CdcAcmClass;

#[embassy_executor::task]
pub async fn cdc_task(mut cdc: CdcAcmClass<'static, Driver<'static, USB>>) {
    // it gets data from the host and sends them over to main, either being an ST7735 or mcp4728, probably main?
    // I have no idea how to get data out, ill just get data in for now
    // * Also, since this is USB powered, we don't care about disconnection, cuz we'll just shut down
    // * But i'll folow the embassy example of doing the 
}


