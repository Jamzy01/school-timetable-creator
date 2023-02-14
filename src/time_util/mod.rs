use chrono::prelude::*;

pub fn local_year_to_system_time(year: i32) -> i64 {
    Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).single().unwrap().timestamp()
}
