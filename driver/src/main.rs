use systemstat::{System, Platform, saturating_sub_bytes};
use std::{thread, time, env};

fn main() {
    /*

        * TODO: Write an file for these calculations
        * TODO: Check performance 

     */
    let sys = System::new();

    let total_mem: u64 = match sys.memory() {
        Ok(mem) => mem.total.as_u64(),
        Err(x) =>{ eprintln!("Error getting total memory! {}", x); 0}
    };

    println!("Total memory : {}", total_mem);

    let total_swap: u64 = match sys.swap() {
        Ok(swap) => swap.total.as_u64(),
        Err(x) => {eprintln!("Error getting total swap {}", x); 0}
    };

    println!("Total swap : {}", total_swap);

    const DAC_MAX: f32 = 4095.0;    
/* 
    loop {
        let function_stopwatch = time::Instant::now();

        let mut used_bytes 
        

        let mem_usage_percent: f32 = (used_mem as f32 / total_mem as f32) * 100.0;
        let swap_usage_percent: f32 = (used_swap as f32 / total_swap as f32) * 100.0;

        let mem_dac: f32 = (used_memory as f32 / total_memory as f32) * DAC_MAX;
        let swap_dac: f32 = (used_swap as f32 / total_swap as f32) * DAC_MAX;
        let cpu_dac: f32 = (global_cpu_usage * 4095.0) / 100.0;

        println!(
            "Memory Usage: {}/{} {:.2}% DAC:{}",
            used_memory(),
            total_memory(),
            mem_usage_percent,
            mem_dac
        );
        println!(
            "Swap Usage: {}/{} {:.2}% DAC:{}",
            used_swap(),
            total_swap(),
            swap_usage_percent,
            swap_dac
        );
        println!("CPU Usage {}% DAC:{}", global_cpu_usage(), cpu_dac);
        println!("--------------");

        thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    }
*/
}
