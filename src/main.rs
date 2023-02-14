const START_YEAR: i32 = 2023;
const END_YEAR: i32 = 2024;
const USER_ID: i32 = 620;

mod time_util;
mod timetable_data;

#[tokio::main]
async fn main() {
    let resp = timetable_data::timetable_request::get_timetable_data(USER_ID, START_YEAR, END_YEAR).await;
    println!("{:#?}", resp);
}
