use super::TimetableEvent;
use chrono::{DateTime, FixedOffset};

const USER_COOKIE: &str = "_ga=GA1.1.222620736.1675298841; _ga_Y2LZ2LSJHM=GS1.1.1676408136.10.1.1676408139.0.0.0; _ga_J34MZBT82M=GS1.1.1676425627.24.1.1676425639.0.0.0; PHPSESSID=p4726ktsu3d86m50mho1qj29e5";

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

pub async fn get_timetable_data(
    user_id: i32,
    start_year: i32,
    end_year: i32,
) -> Vec<TimetableEvent> {
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

                                timetable_events.push(TimetableEvent::new(
                                    event_title,
                                    event_start_time.unwrap().timestamp(),
                                    event_end_time.unwrap().timestamp(),
                                    event_all_day,
                                    String::from(event_color),
                                ))
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

    timetable_events
}
