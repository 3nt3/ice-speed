use chrono::{DateTime, Local};

mod models;

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
        .json::<models::TrainStatus>()
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
        .json::<models::TripResponse>()
        .await
        .unwrap_or_else(|why| panic!("error deserializing trip info: {}", why))
        .trip;

    let now = chrono::Utc::now();
    for stop in trip.stops {
        if stop
            .timetable
            .actual_arrival_time
            .map(|time| time.timestamp_millis() < now.timestamp_millis())
            .unwrap_or(true)
            || stop.timetable.actual_arrival_time.is_none()
            || stop.timetable.scheduled_arrival_time.is_none()
        {
            continue;
        }

        let actual_time = DateTime::<Local>::from(stop.timetable.actual_arrival_time.unwrap());
        let scheduled_time =
            DateTime::<Local>::from(stop.timetable.scheduled_arrival_time.unwrap());

        println!(
            "Next stop: {} ({}) {} -- {} km/h",
            actual_time.format("%H:%M").to_string(),
            scheduled_time.format("%H:%M").to_string(),
            stop.station.name,
            status.speed
        );
        break;
    }
}
