use chrono::{DateTime, Local};

pub fn get_now_time() -> DateTime<Local> {
    Local::now()
}
