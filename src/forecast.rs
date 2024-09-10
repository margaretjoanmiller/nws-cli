// nws-cli, a CLI application that gets the forecast or current conditions from the National Weather Service
//     Copyright (C) 2024 Margaret Joan Miller

//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.

//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU General Public License for more details.

//     You should have received a copy of the GNU General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>


pub mod forecast {
    
    

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
        let current_conditions: String = today_i_hope.short_forecast.to_string();

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
