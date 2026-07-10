use std::fs;
use std::os::raw;
use std::path::Path;

pub fn read_cpu_temp() -> u32 {
    let raw_cpu_temp: String = fs::read_to_string("/sys/class/thermal/thermal_zone1/temp")
        .expect("Failed to read the cpu_thermal_zone");
    let millis_cpu_temp: u32 = raw_cpu_temp.trim().parse().unwrap();
    millis_cpu_temp / 1000
}
