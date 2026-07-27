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

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum PlayerStatus {
    Playing,
    Paused,
    Stopped,
}

impl PlayerStatus {
    pub fn as_str(&self) -> &str {
        match self {
            PlayerStatus::Playing => "▶",
            PlayerStatus::Paused => "▮▮",
            PlayerStatus::Stopped => "■",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DashboardData {
    pub time: String<10>,
    pub mem_used: u16,
    pub swap_used: u16,
    pub cpu_load: u16,
    pub cpu_temp: u16,
    pub player_status: PlayerStatus,
    pub title: String<64>,
    pub artist: String<64>,
    pub progress: u64,
    pub duration: u64,
}

pub fn duration_to_string(secs: u64) -> String<10> {
    let whole_hours = secs / (60 * 60);
    let secs = secs - whole_hours * 60 * 60;
    let whole_minutes = secs / 60;
    let secs = secs - whole_minutes * 60;

    let mut buf: String<10> = String::new();
    write!(buf, "{:02}:{:02}:{:02}", whole_hours, whole_minutes, secs).unwrap();
    buf
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotificationData {
    pub app: String<16>,
    pub summary: String<128>,
    pub body: String<256>,
}

impl NotificationData {
    pub fn new() -> Self {
        let app: String<16> = String::new();
        let summary: String<128> = String::new();
        let body: String<256> = String::new();
        Self {
            app: app,
            summary: summary,
            body: body,
        }
    }
}

pub const DEVICE_VID: u16 = 0x1209;
pub const DEVICE_PID: u16 = 0x4da5;
pub const CDC_INTERFACE: u8 = 0x0;
