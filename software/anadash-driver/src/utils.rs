use chrono::Local;
use tokio_serial::ErrorKind::NoDevice;
use tokio_serial::{Error, SerialPortInfo, SerialPortType, available_ports};

pub fn live_clock() -> heapless::String<10> {
    let now = Local::now();
    let time_str = now.format("%H:%M:%S").to_string();
    let time: heapless::String<10> =
        heapless::String::try_from(time_str.as_str()).unwrap_or_default();
    return time;
}
