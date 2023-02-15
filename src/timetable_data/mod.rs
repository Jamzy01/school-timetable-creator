pub mod timetable_request;
pub mod csv_calendar_serializer;
use serde::{Deserialize, Serialize};

use uuid::Uuid;

use ics::properties::{Comment, Status, Summary, Organizer, DtStart, DtEnd, Categories, Description};
use ics::{ICalendar, Event, escape_text};
use chrono::{TimeZone, NaiveDateTime, Duration};

use chrono::Utc;

use chrono::DateTime;

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

    pub fn as_csv_event(&self) -> [String; 21] {
        let mut event: [String; 21] = [String::from("event subject"), String::from("event start date"), String::from("event start time"), String::from("event end date"), String::from("event end time"), String::from("event all day"), String::from("reminder"), String::from("reminder date"), String::from("reminder time"), String::from("meeting organizer"), String::from("required attendees"), String::from("optional attendees"), String::from("meeting resources"), String::from("billing information"), String::from("description"), String::from("mileage"), String::from("priority"), String::from("private"), String::from("sensitivity"), String::from("show time as")];

        // Set Event Subject

        event[0] = self.title.to_string();

        // Set Event Start Time

        event[1] = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(self.start, 0).unwrap(), Utc).format("%D").to_string();
        event[2] = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(self.start, 0).unwrap(), Utc).format("%r").to_string();

        // Set Event End Time

        event[3] = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(self.end, 0).unwrap(), Utc).format("%D").to_string();
        event[4] = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(self.end, 0).unwrap(), Utc).format("%r").to_string();

        // Set Event All Day

        event[5] = match self.all_day {
            true => String::from("TRUE"),
            false => String::from("FALSE")
        };

        // Create Event Reminder

        event[6] = String::from("TRUE");

        let reminder_time = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(self.start, 0).unwrap(), Utc).checked_sub_signed(Duration::minutes(10)).unwrap();

        event[7] = reminder_time.format("%D").to_string();
        event[8] = reminder_time.format("%r").to_string();

        // Create Event Priority

        event[18] = String::from("Normal");

        // Create Event Private

        event[19] = String::from("FALSE");

        // Create Event Sensitivity

        event[20] = String::from("Normal");

        // Create Event Show Time As

        event[21] = String::from("3");

        event
    }
}
