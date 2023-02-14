use chrono::{DateTime, Utc, NaiveDateTime};

use chrono::TimeZone;

use super::TimetableEvent;

const USER_COOKIE: &str = "_ga=GA1.1.222620736.1675298841; _ga_Y2LZ2LSJHM=GS1.1.1676343654.8.0.1676343654.0.0.0; PHPSESSID=lmlbtjr7kn81b7dc3anuv7nlmt; _ga_J34MZBT82M=GS1.1.1676350245.21.0.1676350245.0.0.0";

pub fn generate_timetable_request_uri(user_id: i32, start_year: i32, end_year: i32) -> String {
    String::from(format!("https://my.lcgs.tas.edu.au/calendar/ajax/full?start={}&end={}&userId={}", crate::time_util::local_year_to_system_time(start_year), crate::time_util::local_year_to_system_time(end_year), user_id))
}

pub async fn get_raw_timetable_data(user_id: i32, start_year: i32, end_year: i32) -> String {
    match reqwest::Client::new()
        .get(generate_timetable_request_uri(user_id, start_year, end_year)).header("cookie", USER_COOKIE)
        .send()
        .await {
            Ok(raw_timetable_data) => {
                match raw_timetable_data.text().await {
                    Ok(raw_timetable_data) => raw_timetable_data,
                    Err(_) => panic!("Failed to load timetable data")
                }
            },
            Err(_) => panic!("Failed to load timetable data")
        }
}

pub async fn get_timetable_data(user_id: i32, start_year: i32, end_year: i32) -> Vec<TimetableEvent> {
    let mut timetable_events: Vec<TimetableEvent> = Vec::new();

    let raw_timetable_data = get_raw_timetable_data(user_id, start_year, end_year).await;

    match serde_json::from_str::<serde_json::Value>(&raw_timetable_data) {
        Ok(timetable_data) => {
            for timetable_data_event in timetable_data.as_array().unwrap_or(&Vec::new()) {
                match timetable_data_event.get("title") {
                    Some(raw_timetable_title_data) => {
                        match raw_timetable_title_data.as_str() {
                            Some(timetable_title_data) => {
                                // Parse Event Title

                                let split_timetable_title_data = timetable_title_data.split(",").map(|timetable_title_data_part| timetable_title_data_part.trim()).collect::<Vec<&str>>();
                
                                let mut event_title = format!("School Event - {}", timetable_title_data);

                                if split_timetable_title_data[0].starts_with("Day ") && split_timetable_title_data[1].starts_with("Period ") {
                                    event_title = format!("Class - {} {}", split_timetable_title_data[3], split_timetable_title_data[2])
                                }

                                // Parse Event Times Of Relevance

                                let raw_event_start_time = timetable_data_event.get("start");

                                let event_start_time: NaiveDateTime;

                                match raw_event_start_time {
                                    Some(raw_event_start_time) => {
                                        match raw_event_start_time.as_str() {
                                            Some(raw_event_start_time) => {
                                                match NaiveDateTime::parse_from_str(raw_event_start_time, "%+") {
                                                    Ok(raw_event_start_time) => {
                                                        event_start_time = raw_event_start_time;
                                                    },
                                                    Err(_) => ()
                                                }
                                            },
                                            None => ()
                                        }
                                    },
                                    None => ()
                                }

                                println!("Timetable Event: {}", event_title);
                            },
                            None => ()
                        }
                        
                    },
                    None => ()
                }

                
            }
        },
        Err(_) => panic!("Failed to load timetable data")
    }

    timetable_events
}