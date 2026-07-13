use std::fs;
use std::time::{Duration, Instant};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

pub struct Stats {
    pub cpu_load: f32,
    pub mem_used: f32,
    pub swap_used: f32,
    pub cpu_temp: f32,
}
pub struct HardwareInfo {
    refresh_kind: RefreshKind,
    sys: System,
    total_memory: f32,
    total_swap: f32,
    last_reading: Instant,
}

impl HardwareInfo {
    pub fn new() -> Self {
        let refresh_kind = RefreshKind::nothing()
            .with_memory(MemoryRefreshKind::everything())
            .with_cpu(CpuRefreshKind::nothing().with_cpu_usage());

        let mut sys: System = System::new_with_specifics(refresh_kind);
        sys.refresh_specifics(refresh_kind);

        let total_memory: f32 = sys.total_memory() as f32;
        let total_swap: f32 = sys.total_swap() as f32;

        let mut last_reading = Instant::now();

        Self {
            refresh_kind,
            sys,
            total_memory,
            total_swap,
            last_reading,
        }
    }

    pub fn read_cpu_temp(&self) -> f32 {
        let raw_cpu_temp: String = fs::read_to_string("/sys/class/thermal/thermal_zone1/temp")
            .expect("Failed to read the cpu_thermal_zone");
        let millis_cpu_temp: u32 = raw_cpu_temp.trim().parse().unwrap();
        millis_cpu_temp as f32 / 1000.0
    }

    pub fn get_data(&mut self) -> Stats {
        self.sys.refresh_specifics(self.refresh_kind);

        let mut cpu_load: f32;

        let one_second: Duration = Duration::new(1, 0);
        if self.last_reading.elapsed() < one_second {
            cpu_load = 0.0;
        } else {
            self.sys.refresh_cpu_usage();
            cpu_load = self.sys.global_cpu_usage() / 100.0;
            self.last_reading = Instant::now();
        }

        let mem_used: f32 = self.sys.used_memory() as f32 / self.total_memory;
        let swap_used: f32 = self.sys.used_swap() as f32 / self.total_swap;
        let cpu_temp: f32 = self.read_cpu_temp();

        Stats {
            cpu_load,
            mem_used,
            swap_used,
            cpu_temp,
        }
    }
}
