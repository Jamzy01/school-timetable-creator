const START_YEAR: i32 = 2023;
const END_YEAR: i32 = 2024;
const USER_ID: i32 = 620;

mod time_util;
mod timetable_data;

#[tokio::main]
async fn main() {
    let timetable =
        timetable_data::timetable_request::get_timetable_data(USER_ID, START_YEAR, END_YEAR).await;
    println!("{:#?}", timetable);

    let mut class_count = 0;

    for event in timetable.iter() {
        if event.title.starts_with("Class -") {
            class_count += 1;
        }
    }

    println!("Class Count: {}", class_count);

    println!("Timetable Length: {}", timetable.len());
}
