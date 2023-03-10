use super::{Timetable, TimetableEvent};
use chrono::{DateTime, FixedOffset};
use uuid::Uuid;

const USER_COOKIE: &str = "";

const INCLUDE_CLASSES: bool = false;
const INCLUDE_EVENTS: bool = true;

// Get Cookie By Opening Timetable Request URI In Browser When Logged Into Account on myGrammar, Then Use DevTools To Find The Cookie Header In The Request From The Network Tab

pub fn generate_timetable_request_uri(user_id: i32, start_year: i32, end_year: i32) -> String {
    String::from(format!(
        "https://my.lcgs.tas.edu.au/calendar/ajax/full?start={}&end={}&userId={}",
        crate::time_util::local_year_to_system_time(start_year),
        crate::time_util::local_year_to_system_time(end_year),
        user_id
    ))
}

pub async fn get_raw_timetable_data(user_id: i32, start_year: i32, end_year: i32) -> String {
    match reqwest::Client::new()
        .get(generate_timetable_request_uri(
            user_id, start_year, end_year,
        ))
        .header("cookie", USER_COOKIE)
        .send()
        .await
    {
        Ok(raw_timetable_data) => match raw_timetable_data.text().await {
            Ok(raw_timetable_data) => raw_timetable_data,
            Err(_) => panic!("Failed to load timetable data"),
        },
        Err(_) => panic!("Failed to load timetable data"),
    }
}

pub async fn get_timetable_data(user_id: i32, start_year: i32, end_year: i32) -> Timetable {
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

                                let split_timetable_title_data = timetable_title_data
                                    .split(",")
                                    .map(|timetable_title_data_part| {
                                        timetable_title_data_part.trim()
                                    })
                                    .collect::<Vec<&str>>();

                                let mut event_title =
                                    format!("School Event - {}", timetable_title_data);

                                if split_timetable_title_data[0].starts_with("Day ")
                                    && split_timetable_title_data[1].starts_with("Period ")
                                {
                                    event_title = format!(
                                        "Class - {} {}",
                                        split_timetable_title_data[3],
                                        split_timetable_title_data[2]
                                    )
                                }

                                // Parse Event Times Of Relevance

                                let raw_event_start_time = timetable_data_event.get("start");

                                let mut event_start_time: Option<DateTime<FixedOffset>> = None;

                                match raw_event_start_time {
                                    Some(raw_event_start_time) => {
                                        match raw_event_start_time.as_str() {
                                            Some(raw_event_start_time) => {
                                                match DateTime::parse_from_str(
                                                    raw_event_start_time,
                                                    "%+",
                                                ) {
                                                    Ok(raw_event_start_time) => {
                                                        event_start_time =
                                                            Some(raw_event_start_time);
                                                    }
                                                    Err(_) => (),
                                                }
                                            }
                                            None => (),
                                        }
                                    }
                                    None => (),
                                }

                                let raw_event_end_time = timetable_data_event.get("end");

                                let mut event_end_time: Option<DateTime<FixedOffset>> = None;

                                match raw_event_end_time {
                                    Some(raw_event_end_time) => match raw_event_end_time.as_str() {
                                        Some(raw_event_end_time) => {
                                            match DateTime::parse_from_str(raw_event_end_time, "%+")
                                            {
                                                Ok(raw_event_end_time) => {
                                                    event_end_time = Some(raw_event_end_time);
                                                }
                                                Err(_) => (),
                                            }
                                        }
                                        None => (),
                                    },
                                    None => (),
                                }

                                // Check If The Event Is An All Day Event

                                let event_all_day = timetable_data_event
                                    .get("allDay")
                                    .unwrap_or(&serde_json::Value::Bool(false))
                                    .as_bool()
                                    .unwrap_or(false);

                                // Get The Color Of The Event

                                let default_event_color =
                                    serde_json::Value::String(String::from("#fff"));

                                let event_color = timetable_data_event
                                    .get("color")
                                    .unwrap_or(&default_event_color)
                                    .as_str()
                                    .unwrap_or("#fff");

                                if event_start_time.is_none() || event_end_time.is_none() {
                                    continue; // Skip event
                                }

                                // Get Description

                                let mut description: String = String::from("No Description");

                                if event_title.starts_with("Class - ") {
                                    description = String::from(format!(
                                        "Class Info: {}",
                                        timetable_title_data
                                    ));
                                }

                                // Get Notification

                                let mut notification: Option<i64> = None;

                                if event_title.starts_with("Class - ") {
                                    notification = Some(-10);
                                }

                                if (!event_title.starts_with("School Event - ") || INCLUDE_EVENTS)
                                    && (!event_title.starts_with("Class - ") || INCLUDE_CLASSES)
                                {
                                    timetable_events.push(TimetableEvent::new(
                                        sha256::digest(event_title.to_string()),
                                        Uuid::new_v4().hyphenated().to_string(),
                                        event_title,
                                        event_start_time.unwrap().timestamp(),
                                        event_end_time.unwrap().timestamp(),
                                        event_all_day,
                                        String::from(event_color),
                                        notification,
                                        description,
                                    ))
                                }
                            }
                            None => (),
                        }
                    }
                    None => (),
                }
            }
        }
        Err(_) => {
            panic!("Failed to load timetable data, most likely because the cookie was invalid")
        }
    }

    Timetable::new(timetable_events)
}
