const START_YEAR: i32 = 2023;
const END_YEAR: i32 = 2024;
const USER_ID: i32 = 620;

mod time_util;
mod timetable_data;

use chrono::NaiveDateTime;
use native_dialog::FileDialog;

#[tokio::main]
async fn main() {
    let mut timetable =
        timetable_data::timetable_request::get_timetable_data(USER_ID, START_YEAR, END_YEAR).await;

    timetable.merge_events_within_range(300);

    println!("{:#?}", timetable);

    let mut class_count = 0;

    for event in timetable.events.iter() {
        if event.title.starts_with("Class -") {
            class_count += 1;
        }
    }

    println!("Class Count: {}", class_count);

    println!("Timetable Length: {}", timetable.events.len());

    let calendar_save_file_dialog = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("CSV Calendar File", &["csv"])
        .show_save_single_file()
        .unwrap();

    match calendar_save_file_dialog {
        Some(calendar_save_file_dialog) => timetable.serialize_events(calendar_save_file_dialog),
        None => panic!("Unable to save csv calendar file"),
    }
}
