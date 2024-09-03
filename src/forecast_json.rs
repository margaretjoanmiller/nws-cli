use serde::{Serialize, Deserialize};
pub mod forecast_json {
	#[derive(Serialize, Deserialize, Debug)]
pub struct Geometry {
	pub geometry_type: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Elevation {
	pub unit_code: String,

	pub value: i64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ProbabilityOfPrecipitation {
	pub unit_code: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Periods {
	pub detailed_forecast: String,

	pub end_time: String,

	pub icon: String,

	pub is_daytime: bool,

	pub name: String,

	pub number: i32,

	pub probability_of_precipitation: ProbabilityOfPrecipitation,

	pub short_forecast: String,

	pub start_time: String,

	pub temperature: i32,

	pub temperature_trend: String,

	pub temperature_unit: String,

	pub wind_direction: String,

	pub wind_speed: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Properties {
	pub elevation: Elevation,

	pub forecast_generator: String,

	pub generated_at: String,

	pub periods: Vec<Periods>,

	pub units: String,

	pub update_time: String,

	pub valid_times: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ForecastJson {
	#[serde (rename="@context")]
	pub context: Vec<String>,

	pub geometry: Geometry,

	pub properties: Properties,

	pub root_type: String,
}

}
