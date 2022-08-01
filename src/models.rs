use chrono::{serde::ts_milliseconds, serde::ts_milliseconds_option, DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrainStatus {
    pub speed: f32,
    pub train_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TripResponse {
    pub trip: Trip,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trip {
    pub stops: Vec<Stop>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stop {
    pub station: Station,
    pub timetable: Timetable,
    pub info: StopInfo,
    pub track: Track,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    pub eva_nr: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timetable {
    #[serde(with = "ts_milliseconds_option")]
    pub scheduled_arrival_time: Option<DateTime<Utc>>,
    #[serde(with = "ts_milliseconds_option")]
    pub actual_arrival_time: Option<DateTime<Utc>>,
    pub arrival_delay: String,
    #[serde(with = "ts_milliseconds_option")]
    pub scheduled_departure_time: Option<DateTime<Utc>>,
    #[serde(with = "ts_milliseconds_option")]
    pub actual_departure_time: Option<DateTime<Utc>>,
    pub show_actual_departure_time: Option<bool>,
    pub departure_delay: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopInfo {
    pub status: i32,
    pub passed: bool,
    pub position_status: String,
    pub distance: i32,
    pub distance_from_start: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub scheduled: String,
    pub actual: String,
}
