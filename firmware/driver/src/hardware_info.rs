use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use::std::time::{Duration, Instant};

pub struct HardwareInfo {
    let refresh_kind: RefreshKind,
    let sys: System,
    let total_memory: f32,
    let total_swap: f32,
    let mut last_reading: Instant
}

impl HardwareInfo {
    pub fn new() -> Self {
        let refresh_kind = RefreshKind::nothing()
            .with_memory(MemoryRefreshKind::everything())
            .with_cpu(CpuRefreshKind::nothing().with_cpu_usage());

        let mut sys: System = System::new_with_specifics(refresh_kind);
        sys.refresh_specifics(refresh_kind);

        let total_memory_float: f32 = sys.total_memory() as f32;
        let total_swap_float: f32 = sys.total_swap() as f32;

        let mut last_reading = Instant::now();

        Self{refresh_kind, sys, total_memory, total_swap, last_reading}
    }
    fn read_cpu_temp() -> f32 {
        let raw_cpu_temp: String = fs::read_to_string("/sys/class/thermal/thermal_zone1/temp")
            .expect("Failed to read the cpu_thermal_zone");
        let millis_cpu_temp: u32 = raw_cpu_temp.trim().parse().unwrap();
        millis_cpu_temp / 1000.0
    }
    pub fn get_data -> Stats {
        sys.refresh_specifics(refresh_kind);
        
        let mut cpu_load: f32;
       
        let one_second = Duration::new(1 , 0);
        if Self.lastReading < one_second {
            cpu_load = 0;
        } else {
            Self.sys.refresh_cpu_usage();
            _cpu_load: f32 = sys.global_cpu_usage();
            lastReading = Instant::now();
        }

        let mem_used: f32 = sys.used_memory() as f32 / Self.total_memory;
        let swap_used: f32 = sys.used_swap() as f32 / Self.total_swap;
        let cpu_temp: f32 = Self.read_cpu_temp();

        Stats {
            cpu_load,
            mem_used,
            swap_used,
            cpu_temp
        }
    }
}