use nwslib::{forecast::{self, forecast::{get_forecast_raw,get_current_condtions}}, forecast_json, geocode::geocode::get_grids};

// extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


#[tokio::main]
async fn main() {

    let forecast = get_current_condtions(34.05, -118.25).await.expect("Could not get forecast!");
    // let grid = get_forecast_raw(34.05, -118.25).await.expect("Could not get grids!");
    println!("{}", forecast);


}
