use reqwest::{Client,ClientBuilder,Response};
use serde_json::{Result, Value};

async fn get_grids(lat: f32, long: f32) -> Value {
    let builder = reqwest::Client::builder()
        .user_agent("nws-cli-0.1")
        .build();

    let client:Client = match builder {
        Ok(cl) => cl,
        Err(err) => panic!("Couldn't create client! {err}")
    };

    let echo_json_result = client
        .get(format!("https://api.weather.gov/points/{lat},{long}"))
        .send()
        .await;

    let json_raw = match echo_json_result {
        Ok(val) => val,
        Err(err) => panic!("Failed to send http, {err}")
    };

    match json_raw.json::<serde_json::Value>().await {
        Ok(val) => val,
        Err(err) => panic!("Failed to parse JSON: {err}.")
    }
}

#[tokio::main]
async fn main() {

    let echo_json = get_grids(34.052235, -118.243684).await;
    println!("{:#?}", echo_json);


}
