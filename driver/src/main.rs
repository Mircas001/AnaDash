use sysinfo::{Component, Components, CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use std::{thread, time, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    
    
    /*
           this part of the code is crucial for performance
       basically, i have told the sysinfo library to only look for RAM, swap, and cpu usage, this helps drop the time it takes for it to refresh
       from 100ms to 500µs, which is half bad but way better
    */
    let memory_kind = MemoryRefreshKind::everything();
    let cpu_kind = CpuRefreshKind::everything().without_frequency();
    let refresh_kind = RefreshKind::nothing()
        .with_memory(memory_kind)
        .with_cpu(cpu_kind);
    let mut sys = System::new_with_specifics(refresh_kind);
    sys.refresh_all();

    const DAC_MAX: f32 = 4095.0;

    let components = Components::new_with_refreshed_list();
    for component in components.list() {
        println!("{component:?}");
    }
    let millis = time::Duration::from_millis(1000);

    thread::sleep(millis);

    loop {
        let function_stopwatch = time::Instant::now();
        sys.refresh_specifics(refresh_kind);
        sys.refresh_cpu_usage();

        let mem_usage_percent: f32 = (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0;
        let swap_usage_percent: f32 = (sys.used_swap() as f32 / sys.total_swap() as f32) * 100.0;

        let mem_dac: f32 = (sys.used_memory() as f32 / sys.total_memory() as f32) * DAC_MAX;
        let swap_dac: f32 = (sys.used_swap() as f32 / sys.total_swap() as f32) * DAC_MAX;
        let cpu_dac: f32 = (sys.global_cpu_usage() * 4095.0) / 100.0;

        println!(
            "Memory Usage: {}/{} {:.2}% DAC:{}",
            sys.used_memory(),
            sys.total_memory(),
            mem_usage_percent,
            mem_dac
        );
        println!(
            "Swap Usage: {}/{} {:.2}% DAC:{}",
            sys.used_swap(),
            sys.total_swap(),
            swap_usage_percent,
            swap_dac
        );
        println!("CPU Usage {}% DAC:{}", sys.global_cpu_usage(), cpu_dac);
        println!("--------------");

        thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    }
}
