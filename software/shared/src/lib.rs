#![no_std]

/*
 * This crate allows for both firmware and driver to agree on what each value is
*/

use core::fmt::Write;
use heapless::String;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum HostTransmission {
    Notification(NotificationData),
    Dashboard(DashboardData),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DashboardData {
    pub time: String<10>,
    pub mem_used: u16,
    pub swap_used: u16,
    pub cpu_load: u16,
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
pub struct NotificationData {
    pub app: String<16>,
    pub summary: String<128>,
    pub body: String<256>,
}

pub const DEVICE_VID: u16 = 0x1209;
pub const DEVICE_PID: u16 = 0x4da5;
pub const CDC_INTERFACE: u8 = 0x0;

// * thanks Arduino, using your math
pub fn map_f32(val: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn map_f64(val: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn map_u32(val: u32, in_min: u32, in_max: u32, out_min: u32, out_max: u32) -> u32 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn map_u64(val: u64, in_min: u64, in_max: u64, out_min: u64, out_max: u64) -> u64 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}
