use anyhow::Result;
use std::io::{Write, stdout};
use tokio::time::{Duration, interval};

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
                println!("Notification icon: {}", noti.notification_icon);
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

                clearscreen::clear().expect("failed to clear screen");
                // * For now, we are just printing the data we get, but I'll have to figure out how to send it over usb, later.
                println!(
                    "{} Memory:{:.2} | Swap:{:.2} | Cpu Load:{:.2} | Cpu Temp:{}C ",
                    utils::live_clock(),
                    hw_stats.mem_used,
                    hw_stats.swap_used,
                    hw_stats.cpu_load,
                    hw_stats.cpu_temp
                );
                println!("{} | {} - {} [{}/{}]", mpris_player.status, mpris_player.title, mpris_player.artist, mpris_player.progress, mpris_player.duration);
            }
        }
    }

    Ok(())
}
