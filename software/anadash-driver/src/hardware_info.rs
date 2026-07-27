use log::error;
use map_arduino::{map_f32, map_u64};
use std::fs;
use std::time::{Duration, Instant};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

const CPU_TEMP_PATH: &str = "/sys/class/thermal/thermal_zone1/temp";

pub struct Stats {
    pub cpu_load: u16,
    pub mem_used: u16,
    pub swap_used: u16,
    pub cpu_temp: u16,
}
pub struct HardwareInfo {
    refresh_kind: RefreshKind,
    sys: System,
    total_memory: u64,
    total_swap: u64,
    last_reading: Instant,
}

impl HardwareInfo {
    pub fn new() -> Self {
        let refresh_kind = RefreshKind::nothing()
            .with_memory(MemoryRefreshKind::everything())
            .with_cpu(CpuRefreshKind::nothing().with_cpu_usage());

        let mut sys: System = System::new_with_specifics(refresh_kind);
        sys.refresh_specifics(refresh_kind);

        let total_memory = sys.total_memory();
        let total_swap = sys.total_swap();

        let last_reading = Instant::now();

        Self {
            refresh_kind,
            sys,
            total_memory,
            total_swap,
            last_reading,
        }
    }

    pub fn read_cpu_temp(&self) -> u16 {
        let raw_cpu_temp: String = match fs::read_to_string(CPU_TEMP_PATH) {
            Ok(temp_string) => temp_string,
            Err(e) => {
                error!("Error opening cpu temp file! {}", e);
                return 0;
            }
        };
        let millis_cpu_temp: u16 = match raw_cpu_temp.trim().parse() {
            Ok(temp) => temp,
            Err(e) => {
                error!("Error getting cpu temp from file!: {}", e);
                return 0;
            }
        };
        millis_cpu_temp / 1000
    }

    pub fn get_data(&mut self) -> Stats {
        let one_second: Duration = Duration::new(1, 0);
        self.sys.refresh_specifics(self.refresh_kind);

        let cpu_load: u16;

        if self.last_reading.elapsed() < one_second {
            cpu_load = 0;
        } else {
            self.sys.refresh_cpu_usage();
            cpu_load = map_f32(self.sys.global_cpu_usage(), 0.0, 100.0, 0.0, 4096.0) as u16;
            self.last_reading = Instant::now();
        }

        let mem_used = map_u64(self.sys.used_memory(), 0, self.total_memory, 0, 4096) as u16;
        let swap_used = map_u64(self.sys.used_swap(), 0, self.total_swap, 0, 4096) as u16;
        let cpu_temp: u16 = self.read_cpu_temp();

        Stats {
            cpu_load,
            mem_used,
            swap_used,
            cpu_temp,
        }
    }
}
