use chrono::Local;

pub fn live_clock() -> String {
    let now = Local::now();
    now.format("%H:%M:%S").to_string()
}
