use chrono::{serde::ts_milliseconds, serde::ts_milliseconds_option, DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TrainStatus {
    speed: f32,
    train_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TripResponse {
    trip: Trip,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Trip {
    stops: Vec<Stop>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Stop {
    station: Station,
    timetable: Timetable,
    info: StopInfo,
    track: Track,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Station {
    eva_nr: String,
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Timetable {
    #[serde(with = "ts_milliseconds_option")]
    scheduled_arrival_time: Option<DateTime<Utc>>,
    #[serde(with = "ts_milliseconds_option")]
    actual_arrival_time: Option<DateTime<Utc>>,
    arrival_delay: String,
    #[serde(with = "ts_milliseconds_option")]
    scheduled_departure_time: Option<DateTime<Utc>>,
    #[serde(with = "ts_milliseconds_option")]
    actual_departure_time: Option<DateTime<Utc>>,
    show_actual_departure_time: Option<bool>,
    departure_delay: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StopInfo {
    status: i32,
    passed: bool,
    position_status: String,
    distance: i32,
    distance_from_start: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Track {
    scheduled: String,
    actual: String,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let status = client
        .get("https://iceportal.de/api1/rs/status")
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:10.0) Gecko/20100101 Firefox/10.0",
        )
        .send()
        .await
        .unwrap()
        .json::<TrainStatus>()
        .await
        .unwrap_or_else(|why| panic!("error deserializing: {}", why));

    let trip = client
        .get("https://iceportal.de/api1/rs/tripInfo/trip")
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:10.0) Gecko/20100101 Firefox/10.0",
        )
        .send()
        .await
        .unwrap()
        .json::<TripResponse>()
        .await
        .unwrap_or_else(|why| panic!("error deserializing trip info: {}", why));

    dbg!(trip);

    println!("{}: {} km/h", status.train_type, status.speed);
}
