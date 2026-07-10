use core::time;
use std::thread;
use std::time::{Duration, Instant};
use systemstat::{Platform, System, saturating_sub_bytes};

fn main() {
    let setup_start = Instant::now();
    let sys = System::new();

    let total_mem: f32 = match sys.memory() {
        Ok(mem) => mem.total.as_u64() as f32,
        Err(x) => {
            eprintln!("Error getting total memory: {}", x);
            1.0
        }
    };

    let total_swap: f32 = match sys.swap() {
        Ok(swap) => swap.total.as_u64() as f32,
        Err(x) => {
            eprintln!("Error getting total swap: {}", x);
            1.0
        }
    };

    let setup_time = setup_start.elapsed().as_micros();
    println!("Setting up took {}μs", setup_time);

    loop {
        let start_time = Instant::now();

        /*
         * TODO: This code is copy and pasted, so i can understand what does it mean, so i have to fix that
         */
        let cpu_load: f32 = match sys.cpu_load_aggregate() {
            Ok(cpu) => {
                thread::sleep(Duration::from_secs(1));
                let cpu = cpu.done().unwrap();
                1.0 - cpu.idle
            }
            Err(x) => {
                eprintln!("Error getting cpu load: {}", x);
                0.0
            }
        };

        let mem_used: f32 = match sys.memory() {
            Ok(mem) => {
                let used_memory: f32 = saturating_sub_bytes(mem.total, mem.free).as_u64() as f32;
                used_memory / total_mem
            }
            Err(x) => {
                eprintln!("Error getting used memory: {}", x);
                0.0
            }
        };

        let swap_used: f32 = match sys.swap() {
            Ok(swap) => {
                let used_swap: f32 = saturating_sub_bytes(swap.total, swap.free).as_u64() as f32;
                used_swap / total_swap
            }
            Err(x) => {
                eprintln!("Error getting used swap: {}", x);
                0.0
            }
        };

        let cpu_temp: f32 = match sys.cpu_temp() {
            Ok(cpu_temp) => cpu_temp,
            Err(x) => {
                eprintln!("Error getting cpu temperature: {}", x);
                0.0
            }
        };
        let mut time_taken: u128 = start_time.elapsed().as_micros();
        time_taken -= Duration::new(1, 0).as_micros();
        println!("Running this took {}μs", time_taken);
        println!(
            "Memory:{} | Swap:{} | Cpu Load:{} | Cpu Temp:{}C",
            mem_used * 100.0,
            swap_used * 100.0,
            cpu_load * 100.0,
            cpu_temp
        );
    }
}
