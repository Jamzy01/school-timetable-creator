use std::path::PathBuf;

use super::TimetableEvent;
use csv::Writer;

pub fn serialize_events(events: Vec<TimetableEvent>, save_location: PathBuf) {
    let mut calendar = Writer::from_path(save_location.as_os_str().to_str().unwrap()).expect("Failed to load calendar");

    // Add Events

    for event in events.iter() {
        calendar.write_record(event.as_csv_event());
    }

    // Write To File

    calendar.flush();
}