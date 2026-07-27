#[cfg(test)]
use crate::hardware_info;
use crate::mpris_monitor;
use crate::notification_monitor;
use shared::{DashboardData, HostTransmission, duration_to_string};
use tokio::time::{Duration, interval};

#[tokio::test]
async fn main_without_serial() {
    let mut hwinfo: hardware_info::HardwareInfo = hardware_info::HardwareInfo::new();

    let mut timer: tokio::time::Interval = interval(Duration::from_secs(1));

    // naming it notificationsYapper would be unprofessional :(
    let mut notifications_rx = notification_monitor::spawn_notification_monitor();

    let mut mpris_player = mpris_monitor::MprisPlayer::new();
    let mut test_timer: tokio::time::Interval = interval(Duration::from_secs(120));
    loop {
        // this will send the current time every second to the resource monitor
        tokio::select! {
            Some(noti_transm) = notifications_rx.recv() => {
                match noti_transm {
                    HostTransmission::Notification(noti) => {
                println!("Notification by {}", noti.app);
                println!("Summary: {}", noti.summary);
                println!("Body: {}", noti.body);
                    }
                    _ => continue
                }

            }
            _ = tokio::signal::ctrl_c() => {
                println!("Goodbye ;)");
                break;
            }
            _ = timer.tick() => {
                let hw_stats = hwinfo.get_data();
                mpris_player.update();
                let progress_string = duration_to_string(mpris_player.progress);
                let duration_string = duration_to_string(mpris_player.duration);
                let player_status = mpris_player.status.as_str();
                println!(
                    "Memory:{:.2} | Swap:{:.2} | Cpu Load:{:.2} | Cpu Temp:{}C ",
                    hw_stats.mem_used,
                    hw_stats.swap_used,
                    hw_stats.cpu_load,
                    hw_stats.cpu_temp
                );
                println!("{} | {} - {} [{}/{}]",  player_status, mpris_player.title, mpris_player.artist, progress_string, duration_string);

            }
            _ = test_timer.tick() => {
                break;
            }
        };
    }
}
