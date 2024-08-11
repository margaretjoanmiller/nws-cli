#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let echo_json: String = reqwest::Client::new()
        .get("https://api.weather.gov/points/34.052235,-118.243683")
        .header("User-Agent", "nws-cli-0.1")
        .send()
        .await?
        .text()
        .await?;

    println!("{:#?}", echo_json);
    Ok(())
}
