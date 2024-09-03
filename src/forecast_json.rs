pub mod forecast_json {
    use serde::{Serialize, Deserialize};


    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ForecastJson {
        #[serde(rename = "@context")]
        pub context: Vec<ContextElement>,
        pub geometry: Geometry,
        pub properties: Properties,
        #[serde(rename = "type")]
        pub forecast_type: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum ContextElement {
        ContextClass(ContextClass),
        String(String),
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ContextClass {
        #[serde(rename = "@version")]
        pub version: String,
        #[serde(rename = "@vocab")]
        pub vocab: String,
        pub geo: String,
        pub unit: String,
        pub wx: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Geometry {
        pub coordinates: Vec<Vec<Vec<f64>>>,
        #[serde(rename = "type")]
        pub geometry_type: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Properties {
        pub elevation: Elevation,
        pub forecast_generator: String,
        pub generated_at: String,
        pub periods: Vec<Period>,
        pub units: String,
        pub update_time: String,
        pub valid_times: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Elevation {
        pub unit_code: String,
        pub value: f64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Period {
        pub detailed_forecast: String,
        pub end_time: String,
        pub icon: String,
        pub is_daytime: bool,
        pub name: String,
        pub number: i64,
        pub probability_of_precipitation: ProbabilityOfPrecipitation,
        pub short_forecast: String,
        pub start_time: String,
        pub temperature: i64,
        pub temperature_trend: String,
        pub temperature_unit: String,
        pub wind_direction: String,
        pub wind_speed: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ProbabilityOfPrecipitation {
        pub unit_code: String,
    }

}
