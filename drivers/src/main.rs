extern crate systemstat;

use std::thread;
use std::time::Duration;
use systemstat::{System, Platform, saturating_sub_bytes};

fn main() {
    let sys = System::new();
    
    let total_mem: u64 = match sys.memory() {
            Ok(mem) => mem.total.as_u64(),
            Err(x) => 0
        };
    
    let total_swap: u64 = match sys.swap()
    

}
