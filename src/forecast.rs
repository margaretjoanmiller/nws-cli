pub mod forecast {

    use reqwest::Client;

    use crate::{forecast_json::forecast_json::ForecastJson, geocode::get_grids, grid_json::grid_json::{GridJson, GridJsonProperties}};
    
    pub async fn get_forecast_raw(lat: i64, long: i64) -> anyhow::Result<ForecastJson> {
        let json_raw: GridJson = get_grids(lat, long)?;

        
        let forecast_url = json_raw.properties.forecast;

        let builder = Client::builder()
            .user_agent("nws-cli-0.1")
            .build();

        let client: Client = match builder {
            Ok(cl) => cl,
            Err(err) => panic!("Couldn't create client! {err}")
        };

        let echo_json_result = client
            .get(forecast_url)
            .send()
                    .await?
            .json::<ForecastJson>()
            .await?;

        Ok(echo_json_result)

    }

    
}
