// extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


pub mod geocode;
pub mod grid_json;


fn get_location(lat:f32, long:f32) -> Result<(String,i32,i32), E> {
    
}
