use serde::{Deserialize, Serialize};
pub mod forecast_json {
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Geometry {
        pub geometry_type: Option<String>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Elevation {
        pub unit_code: Option<String>,

        pub value: Option<i64>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ProbabilityOfPrecipitation {
        pub unit_code: Option<String>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Periods {
        pub number: Option<i32>,

        pub name: Option<String>,

        pub start_time: Option<String>,

        pub end_time: Option<String>,

        pub is_daytime: Option<bool>,

        pub temperature: Option<i32>,

        pub temperature_unit: Option<String>,

        pub temperature_trend: Option<String>,

        pub probability_of_precipitation: Option<ProbabilityOfPrecipitation>,

        pub wind_speed: Option<String>,

        pub wind_direction: Option<String>,

        pub icon: Option<String>,

        pub short_forecast: Option<String>,

        pub detailed_forecast: Option<String>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Properties {
        pub units: Option<String>,

        pub forecast_generator: Option<String>,

        pub generated_at: Option<String>,

        pub update_time: Option<String>,

        pub valid_times: Option<String>,

        pub elevation: Option<Elevation>,

        pub periods: Option<Vec<Periods>>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ForecastJson {
        #[serde(rename = "@context")]
        pub context: Option<Vec<String>>,

        pub root_type: Option<String>,

        pub geometry: Option<Geometry>,

        pub properties: Option<Properties>,
    }
}
