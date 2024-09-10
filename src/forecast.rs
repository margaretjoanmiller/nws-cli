pub mod forecast {
    
    use std::ops::Deref;

    use reqwest::Client;

    use crate::{
        forecast_json::forecast_json::{ForecastJson, Period},
        geocode::geocode::get_grids,
        grid_json::grid_json::GridJson,
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

        let today_i_hope: &Period = raw_json
            .properties
            .periods.first()
            .expect("Invalid json!");

        let current_temp: String = format!("{}°", &today_i_hope.temperature);
        let current_conditions: String = format!("{}", &today_i_hope.short_forecast);

        Ok(format!("{current_temp} -- {current_conditions}"))
    }
    
    pub async fn get_five_day_forecast(lat: f32, long: f32) -> anyhow::Result<String> {
        let raw_json: ForecastJson = get_forecast_raw(lat, long).await?;

        let forecast_vec = raw_json.properties.periods;
        let mut forecast_string = String::new();

        for n in 1..10 {
            let raw_forecast = &forecast_vec[n-1];
            forecast_string = format!("{forecast_string}\n {}: {}° -- {}",
                                      &raw_forecast.name,
                                      // &raw_forecast.icon,
                                      &raw_forecast.temperature,
                                      &raw_forecast.short_forecast);
        }

        Ok(forecast_string)
    }
}
