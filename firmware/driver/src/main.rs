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

    let mut hwinfo = HardwareInfo::new();

    loop {
        println!("{}", sensor_utils::live_clock());
        // this will send the current time every second to the resource monitor
        sys.refresh_specifics(refresh_kind);
        sys.refresh_cpu_usage();

        let mem_used: f32 = sys.used_memory() as f32 / total_memory_float;
        let swap_used: f32 = sys.used_swap() as f32 / total_swap_float;
        let cpu_load: f32 = sys.global_cpu_usage();

        let cpu_temp: u32 = sensor_utils::read_cpu_temp();

        // * For now, we are just printing the data we get, but I'll have to figure out how to send it over usb, later.
        println!(
            "Memory:{:.2} | Swap:{:.2} | Cpu Load:{:.2} | Cpu Temp:{}C",
            mem_used * 100.0,
            swap_used * 100.0,
            cpu_load,
            cpu_temp
        );
        thread::sleep(one_second);
    }
}
