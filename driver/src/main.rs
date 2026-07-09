use sysinfo::{
    System, MemoryRefreshKind, CpuRefreshKind, RefreshKind, Components, Component,
};

use std::{thread, time};

// I want to get 

fn main() {
    let memory_kind = MemoryRefreshKind::everything();
    let cpu_kind = CpuRefreshKind::everything().without_frequency();
    let refresh_kind = RefreshKind::nothing().with_memory(memory_kind).with_cpu(cpu_kind);
    let mut sys = System::new_with_specifics(refresh_kind);
    sys.refresh_all();

    let total_memory: f32 = sys.total_memory() as f32;

    let total_swap: f32  = sys.total_swap() as f32;

    let total_memoryby100: f32 = total_memory * 100.0;
    let total_swapby100: f32 = total_swap * 100.0;

    const DAC_MAX: f32 = 4095.0;
    let total_memory_dac: f32 = total_memory * DAC_MAX;
    let total_swap_dac: f32 = total_memory * DAC_MAX;

    let mut components = Components::new_with_refreshed_list();
    for component in components.list_mut() {
        component.refresh();
        println!("{component:?}");
    }    
    
    let millis = time::Duration::from_millis(1000);

    thread::sleep(millis);

    loop {
        sys.refresh_all();
        let mem_usage_percent: f32 = sys.used_memory() as f32 / total_memory * 100.0;
        let swap_usage_percent: f32  = sys.used_swap() as f32 / total_memory * 100.0;

        let mem_dac: u16 = ((sys.used_memory() as f32 / total_memory_dac).clamp(0.0, DAC_MAX) as u16);
        let mem_dac: u16 = ((sys.used_swap() as f32 / total_swap_dac).clamp(0.0, DAC_MAX) as u16);

        println!("Memory Usage: {:.2}% DAC:{}", mem_usage_percent, mem_dac);    
        println!("Swap Usage: {:.2}% DAC:{}", swap_usage_percent, mem_dac);
        println!("--------------");

    }
}
