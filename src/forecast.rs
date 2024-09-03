pub mod forecast {

    use anyhow::Error;
    use reqwest::Client;

    use crate::{
        forecast_json::forecast_json::{ForecastJson, Periods},
        geocode::geocode::get_grids,
        grid_json::grid_json::{GridJson, GridJsonProperties},
    };

    pub async fn get_forecast_raw(lat: f32, long: f32) -> anyhow::Result<ForecastJson> {
        let json_raw: GridJson = get_grids(lat, long).await?;

        let forecast_url: String = json_raw.properties.forecast;

        let builder = Client::builder().user_agent("nws-cli-0.1").build();

        let client: Client = match builder {
            Ok(cl) => cl,
            Err(err) => panic!("Couldn't create client! {err}"),
        };

        let echo_json_result = client
            .get(forecast_url)
            .send()
            .await?
            .json::<ForecastJson>()
            .await?;

        Ok(echo_json_result)
    }

    pub async fn get_current_condtions(lat: f32, long: f32) -> anyhow::Result<String> {
        let raw_json: ForecastJson = get_forecast_raw(lat, long).await?;

        let today_i_hope: &Periods = &raw_json
            .properties
            .periods
            .get(0)
            .expect("Invalid json!");

        let current_temp: String = format!("{}°", &today_i_hope.temperature);
        let current_conditions: String = format!("{}°", &today_i_hope.short_forecast);

        Ok(format!("{current_temp} -- {current_conditions}"))
    }
}
