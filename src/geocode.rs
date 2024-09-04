pub mod geocode {
    

    use reqwest::Client;
    
    use crate::grid_json::grid_json::GridJson;

    pub async fn get_grids(lat: f32, long: f32) -> anyhow::Result<GridJson> {
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
            .await?
            .json::<GridJson>()
            .await?;
        Ok(echo_json_result)
    }
}
