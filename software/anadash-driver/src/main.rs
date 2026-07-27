use anyhow::Result;
use log::{error, info, warn};
use shared::{DEVICE_PID, DEVICE_VID, DashboardData, HostTransmission};
use std::io::Write;
use tokio::time::{Duration, interval};
use tokio_serial::SerialPortBuilderExt;

mod hardware_info;
mod mpris_monitor;
mod notification_monitor;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Starting driver...");
    #[cfg(not(target_family = "unix"))]
    compile_error!("Only unix systems are supported!");

    #[cfg(not(target_os = "linux"))]
    warn!("A non-linux Unix system has been detected, this might not work!");

    info!("Opening serial port");
    let keyboard_port = match utils::get_serial_with_vid_pid(DEVICE_VID, DEVICE_PID) {
        Ok(port_info) => port_info,
        Err(e) => {
            panic!("Error getting serial port! {}", e.description);
        }
    };
    let mut keyboard_cdc =
        match tokio_serial::new(keyboard_port.port_name, 115200).open_native_async() {
            Ok(port) => port,
            Err(e) => panic!("Error opening serial port! {}", e),
        };

    info!("Getting hardware info object");
    let mut hwinfo = hardware_info::HardwareInfo::new();

    let mut timer: tokio::time::Interval = interval(Duration::from_secs(1));

    info!("Starting DBUS monitor");
    // naming it notificationsYapper would be unprofessional :(
    let mut notifications_rx = notification_monitor::spawn_notification_monitor();

    info!("Starting MPRIS monitor");
    let mut mpris_player = mpris_monitor::MprisPlayer::new();

    loop {
        // this will send the current time every second to the resource monitor
        tokio::select! {
            Some(noti) = notifications_rx.recv() => {
                info!("Notification received!");
                let mut buf = [0u8; 256];
                let bytes = postcard::to_slice_cobs(&noti, &mut buf)?;
                keyboard_cdc.write_all(bytes)?;
            }
            _ = tokio::signal::ctrl_c() => {
                println!("Goodbye ;)");
                break;
            }
            _ = timer.tick() => {
                info!("Getting hardware info");
                let hw_stats = hwinfo.get_data();
                info!("Getting MPRIS...");
                mpris_player.update();

                let data  = HostTransmission::Dashboard(DashboardData {
                    time: utils::live_clock(),
                    mem_used: hw_stats.mem_used,
                    swap_used: hw_stats.swap_used,
                    cpu_load: hw_stats.cpu_load,
                    cpu_temp: hw_stats.cpu_temp,
                    player_status: mpris_player.status,
                    artist: heapless::String::try_from(mpris_player.artist.as_str()).unwrap_or_default(),
                    title: heapless::String::try_from(mpris_player.title.as_str()).unwrap_or_default(),
                    progress: mpris_player.progress,
                    duration: mpris_player.duration,
                });
                info!("Sending data over..");
                let mut buf = [0u8; 256];
                let bytes = postcard::to_slice_cobs(&data, &mut buf)?;
                keyboard_cdc.write_all(bytes)?;
            }
        }
    }

    Ok(())
}
