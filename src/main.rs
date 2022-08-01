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
        .unwrap_or_else(|why| panic!("error deserializing trip info: {}", why));

    dbg!(trip);

    println!("{}: {} km/h", status.train_type, status.speed);
}
