use anyhow::Result;
use shared;
use tokio::time::{Duration, interval};
use tokio_serial::SerialPortBuilderExt;

mod hardware_info;
mod mpris_monitor;
mod notification_monitor;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(not(target_family = "unix"))]
    compile_error!("Only unix systems are supported!");

    #[cfg(not(target_os = "linux"))]
    println!("A non-linux Unix system has been detected, this might not work!");

    let mut port = tokio_serial::new("/dev/ttyUSB0", 115200).open_native_async();

    let mut hwinfo = hardware_info::HardwareInfo::new();

    let mut timer = interval(Duration::from_secs(1));

    // naming it notificationsYapper would be unprofessional :(
    let mut notifications_rx = notification_monitor::spawn_notification_monitor();

    let mut mpris_player = mpris_monitor::MprisPlayer::new();

    loop {
        // this will send the current time every second to the resource monitor
        tokio::select! {
            Some(noti) = notifications_rx.recv() => {
                println!("Notification by {}", noti.app);
                println!("Summary: {}", noti.summary);
                println!("Body: {}", noti.body);
            }
            _ = tokio::signal::ctrl_c() => {
                println!("Goodbye ;)");
                break;
            }
            _ = timer.tick() => {
                let hw_stats = hwinfo.get_data();
                mpris_player.update();

                let data  = shared::DashboardData {
                    mem_used: hw_stats.mem_used,
                    swap_used: hw_stats.swap_used,
                    cpu_load: hw_stats.cpu_load,
                    cpu_temp: hw_stats.cpu_temp,
                    player_status: heapless::String::try_from(mpris_player.status.as_str()).unwrap_or_default(),
                    artist: heapless::String::try_from(mpris_player.artist.as_str()).unwrap_or_default(),
                    title: heapless::String::try_from(mpris_player.title.as_str()).unwrap_or_default(),
                    progress: mpris_player.progress,
                    duration: mpris_player.duration,
                };

                let progress_string = shared::duration_to_string(data.progress);
                let duration_string = shared::duration_to_string(data.duration);

                clearscreen::clear().expect("failed to clear screen");
                // * For now, we are just printing the data we get, but I'll have to figure out how to send it over usb, later.
                println!(
                    "{} Memory:{:.2} | Swap:{:.2} | Cpu Load:{:.2} | Cpu Temp:{}C ",
                    utils::live_clock(),
                    data.mem_used,
                    data.swap_used,
                    data.cpu_load,
                    data.cpu_temp
                );
                println!("{} | {} - {} [{}/{}]", data.player_status, data.title, data.artist, progress_string, duration_string);
            }
        }
    }

    Ok(())
}
/*
    FIXME: It crashes on the lack of an player
*/
