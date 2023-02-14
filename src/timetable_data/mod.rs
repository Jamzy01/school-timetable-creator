pub mod timetable_request;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TimetableEvent {
    title: String,
    start: String,
    end: String,
    #[serde(rename = "allDay")]
    all_day: bool,
    color: String,
}