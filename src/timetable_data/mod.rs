pub mod timetable_request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TimetableEvent {
    pub title: String,
    pub start: i64,
    pub end: i64,
    #[serde(rename = "allDay")]
    pub all_day: bool,
    pub color: String,
}

impl TimetableEvent {
    pub fn new(title: String, start: i64, end: i64, all_day: bool, color: String) -> Self {
        Self {
            title,
            start,
            end,
            all_day,
            color,
        }
    }
}
