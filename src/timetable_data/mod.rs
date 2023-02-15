pub mod timetable_request;
use std::path::PathBuf;

use csv::Writer;
use serde::{Deserialize, Serialize};

use chrono::{Duration, Local, NaiveDateTime, TimeZone};

use chrono::Utc;

use chrono::DateTime;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimetableEvent {
    pub event_id: String, // Used to differentiate events (When a class for example is a double class, it will still have the same id)
    pub internal_id: String, // Used to internally differentiate Timetable Event structs (When a class for example is a double class, it will have different ids)
    pub title: String,
    pub start: i64,
    pub end: i64,
    #[serde(rename = "allDay")]
    pub all_day: bool,
    pub color: String,
    pub reminder: Option<i64>,
    pub description: String,
}

impl TimetableEvent {
    pub fn new(
        event_id: String,
        internal_id: String,
        title: String,
        start: i64,
        end: i64,
        all_day: bool,
        color: String,
        reminder: Option<i64>,
        description: String,
    ) -> Self {
        Self {
            event_id,
            internal_id,
            title,
            start,
            end,
            all_day,
            color,
            reminder,
            description,
        }
    }

    pub fn as_csv_event(&self) -> [String; 10] {
        let mut event: [String; 10] = [
            String::from("event subject"),
            String::from("event start date"),
            String::from("event start time"),
            String::from("event end date"),
            String::from("event end time"),
            String::from("event all day"),
            String::from("reminder"),
            String::from("reminder date"),
            String::from("reminder time"),
            String::from("description"),
        ];

        // Set Event Subject

        event[0] = self.title.to_string();

        // Set Event Start Time

        event[1] = Local
            .from_utc_datetime(&NaiveDateTime::from_timestamp_opt(self.start, 0).unwrap())
            .format("%D")
            .to_string();
        event[2] = Local
            .from_utc_datetime(&NaiveDateTime::from_timestamp_opt(self.start, 0).unwrap())
            .format("%r")
            .to_string();

        // Set Event End Time

        event[3] = Local
            .from_utc_datetime(&NaiveDateTime::from_timestamp_opt(self.end, 0).unwrap())
            .format("%D")
            .to_string();
        event[4] = Local
            .from_utc_datetime(&NaiveDateTime::from_timestamp_opt(self.end, 0).unwrap())
            .format("%r")
            .to_string();

        // Set Event All Day

        event[5] = match self.all_day {
            true => String::from("TRUE"),
            false => String::from("FALSE"),
        };

        // Create Event Reminder

        event[6] = String::from(match self.reminder.is_some() {
            true => String::from("TRUE"),
            false => String::from("FALSE"),
        });

        if self.reminder.is_some() {
            let reminder_time = Local
                .from_utc_datetime(&NaiveDateTime::from_timestamp_opt(self.start, 0).unwrap())
                .checked_sub_signed(Duration::minutes(self.reminder.unwrap()))
                .unwrap();

            event[7] = reminder_time.format("%D").to_string();
            event[8] = reminder_time.format("%r").to_string();
        }

        // Create Event Description

        event[9] = self.description.to_string();

        // Return Serialized Event

        event
    }
}

#[derive(Debug)]
pub struct Timetable {
    pub events: Vec<TimetableEvent>,
}

impl Timetable {
    pub fn new(events: Vec<TimetableEvent>) -> Self {
        Self { events }
    }

    pub fn merge_events_within_range(&mut self, range_in_seconds: i64) {
        loop {
            let cloned_events = self.events.clone();

            let mut merged_event_id: Option<String> = None;

            for event in self.events.iter_mut() {
                let mut found_event_within_range: Option<&TimetableEvent> = None;

                for check_event in cloned_events.iter() {
                    if event.event_id == check_event.event_id
                        && check_event.start - event.end <= range_in_seconds
                        && check_event.start > event.end
                    {
                        found_event_within_range = Some(check_event);
                        break;
                    }
                }

                match found_event_within_range {
                    Some(found_event_within_range) => {
                        event.end = found_event_within_range.end; // Extend event to merge

                        event.end = 3232;
                        event.start = 3232;

                        println!("Found Range: {}", event.title);

                        merged_event_id = Some(found_event_within_range.internal_id.to_string());

                        break;
                    }
                    None => (),
                }
            }

            match merged_event_id {
                Some(merged_event_id) => {
                    self.events = cloned_events
                        .into_iter()
                        .filter(|event| event.internal_id != merged_event_id)
                        .collect::<Vec<TimetableEvent>>();
                }
                None => {
                    // All events merged, therefore break from loop
                    break;
                }
            }
        }
    }

    pub fn serialize_events(&self, save_location: PathBuf) {
        let mut calendar = Writer::from_path(save_location.as_os_str().to_str().unwrap())
            .expect("Failed to load calendar");

        // Add Helper Row

        calendar
            .write_record(&[
                "Subject",
                "Start Date",
                "Start Time",
                "End Date",
                "End Time",
                "All day event",
                "Reminder",
                "Reminder Date",
                "Reminder Time",
                "Description",
            ])
            .expect("Unable to write helper row");

        // Add Events

        for event in self.events.iter() {
            calendar
                .write_record(event.as_csv_event())
                .expect("Unable to write event to calendar");
        }

        // Write To File

        calendar.flush().expect("Unable to write calendar to file");
    }
}
