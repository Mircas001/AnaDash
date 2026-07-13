use chrono::Local;
use std::fs;
use std::os::raw;
use std::path::Path;


pub fn live_clock() -> String {
    let now = Local::now();
    now.format("%H:%M:%S").to_string()
}
