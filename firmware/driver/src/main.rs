use std::thread;
use std::time::{Duration, Instant};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

use crate::sensor_utils::live_clock;

mod hardware_info;
mod sensor_utils;

fn main() {
    #[cfg(not(target_family = "unix"))]
    compile_error!("Only unix systems are supported!");

    #[cfg(not(target_os = "linux"))]
    println("A non-linux Unix system has been detected, this might not work!");

    let mut hwinfo = hardware_info::HardwareInfo::new();
    let one_second = Duration::new(1, 0);

    loop {
        println!("{}", sensor_utils::live_clock());
        // this will send the current time every second to the resource monitor

        let hw_stats = hwinfo.get_data();

        // * For now, we are just printing the data we get, but I'll have to figure out how to send it over usb, later.
        println!(
            "Memory:{:.2} | Swap:{:.2} | Cpu Load:{:.2} | Cpu Temp:{}C",
            hw_stats.mem_used, hw_stats.swap_used, hw_stats.cpu_load, hw_stats.cpu_temp
        );
        thread::sleep(one_second);
    }
}
