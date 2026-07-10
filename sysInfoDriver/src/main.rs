use std::thread;
use std::time::{Duration, Instant};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

mod sensor_utils;

fn main() {
    let setup_start = Instant::now();

    let refresh_kind = RefreshKind::nothing()
        .with_memory(MemoryRefreshKind::everything())
        .with_cpu(CpuRefreshKind::nothing().with_cpu_usage());

    let mut sys = System::new_with_specifics(refresh_kind);

    sys.refresh_specifics(refresh_kind);

    let total_memory_float: f32 = sys.total_memory() as f32;
    let total_swap_float: f32 = sys.total_swap() as f32;

    let setup_time = setup_start.elapsed().as_micros();

    let one_second = Duration::new(1, 0);

    println!("Setting up took {}μs", setup_time);
    loop {
        let start_time = Instant::now();

        sys.refresh_specifics(refresh_kind);
        sys.refresh_cpu_usage();

        let mem_used: f32 = sys.used_memory() as f32 / total_memory_float;
        let swap_used: f32 = sys.used_swap() as f32 / total_swap_float;
        let cpu_load: f32 = sys.global_cpu_usage();

        let time_taken: u128 = start_time.elapsed().as_micros();

        let cpu_temp: u32 = sensor_utils::read_cpu_temp();

        println!("Running this took {}μs", time_taken);
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
