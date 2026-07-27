use map_arduino::{map_f32, map_u64};
use std::time::{Duration, Instant};
use sysinfo::{Components, CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

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
    components: Components,
    cpu_idx: Option<usize>,
}

impl HardwareInfo {
    pub fn new() -> Self {
        let refresh_kind = RefreshKind::nothing()
            .with_memory(MemoryRefreshKind::everything())
            .with_cpu(CpuRefreshKind::nothing().with_cpu_usage());

        let mut sys: System = System::new_with_specifics(refresh_kind);
        sys.refresh_specifics(refresh_kind);

        let total_memory = sys.total_memory();
        let total_swap: u64 = sys.total_swap();

        let last_reading = Instant::now();
        let components = Components::new_with_refreshed_list();

        // acha o índice do componente de CPU uma única vez
        let cpu_idx = components.iter().position(|c| {
            let l = c.label().to_lowercase();
            l.contains("tctl") || l.contains("package")
        });

        Self {
            refresh_kind,
            sys,
            total_memory,
            total_swap,
            last_reading,
            components,
            cpu_idx,
        }
    }

    pub fn read_cpu_temp(&mut self) -> u16 {
        self.components.refresh(true);

        if let Some(idx) = self.cpu_idx {
            if let Some(temp) = self.components[idx].temperature() {
                return temp as u16;
            }
        }
        return 30;
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
