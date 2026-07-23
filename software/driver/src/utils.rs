use chrono::Local;
use tokio_serial::ErrorKind::NoDevice;
use tokio_serial::{Error, SerialPortInfo, SerialPortType, UsbPortInfo, available_ports};

pub fn live_clock() -> heapless::String<10> {
    let now = Local::now();
    let time_str = now.format("%H:%M:%S").to_string();
    let time: heapless::String<10> =
        heapless::String::try_from(time_str.as_str()).unwrap_or_default();
    return time;
}

pub fn get_serial_with_vid_pid(vid: u16, pid: u16) -> Result<SerialPortInfo, Error> {
    let available_ports = match available_ports() {
        Ok(ports) => ports,
        Err(e) => {
            return Err(e);
        }
    };
    for port in available_ports.iter() {
        match &port.port_type {
            SerialPortType::UsbPort(usb_info) => {
                if usb_info.vid == vid && usb_info.pid == pid {
                    return Ok(port.to_owned());
                }
            }
            _ => continue, // * we don't care
        }
    }
    return Err(Error {
        kind: NoDevice,
        description: "Unable to find an matching serial device!".to_string(),
    });
}
