#![no_std]

/*
 * This crate allows for both firmware and driver to agree on what each value is
*/

use core::fmt::Write;
use heapless::String;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DashboardData {
    pub time: String<10>,
    pub mem_used: f32,
    pub swap_used: f32,
    pub cpu_load: f32,
    pub cpu_temp: u8,
    pub player_status: String<8>,
    pub title: String<64>,
    pub artist: String<64>,
    pub progress: u64,
    pub duration: u64,
}

pub fn duration_to_string(secs: u64) -> String<16> {
    let whole_hours = secs / (60 * 60);
    let secs = secs - whole_hours * 60 * 60;
    let whole_minutes = secs / 60;
    let secs = secs - whole_minutes * 60;

    let mut buf: String<16> = String::new();
    write!(buf, "{:02}:{:02}:{:02}", whole_hours, whole_minutes, secs).unwrap();
    buf
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Notification {
    pub app: String<16>,
    pub summary: String<128>,
    pub body: String<256>,
}
