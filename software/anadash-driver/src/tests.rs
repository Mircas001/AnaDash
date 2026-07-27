#[cfg(test)]

use crate::hardware_info;
use crate::mpris_monitor;
use crate::notification_monitor;
use criterion::{Criterion, criterion_group, criterion_main};

async fn bench_hwinfo_performance(c: &mut Criterion) {
    let mut hwinfo: hardware_info::HardwareInfo = hardware_info::HardwareInfo::new();
    c.bench_function("hwinfo", hwinfo.get_data());
}

async fn bench_mpris_performance(c: &mut Criterion) {
    let mut mpris_monitor = mpris_monitor::MprisPlayer::new();
    c.bench_function("mpris_monitor", mpris_monitor.update());
}

#[tokio::test]
fn main_without_serial() {
    
}

criterion_group!(benches, bench_hwinfo_performance, bench_mpris_performance);
criterion_main!(benches);
