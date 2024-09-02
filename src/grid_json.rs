pub mod grid_json {

    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct GridJson {
        #[serde(rename = "@context")]
        pub context: Vec<ContextElement>,

        pub id: String,

        #[serde(rename = "type")]
        pub grid_json_type: String,

        pub geometry: Geometry,

        pub properties: GridJsonProperties,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum ContextElement {
        ContextClass(ContextClass),

        String(String),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ContextClass {
        #[serde(rename = "@version")]
        version: String,

        wx: String,

        s: String,

        geo: String,

        unit: String,

        #[serde(rename = "@vocab")]
        vocab: String,

        geometry: Distance,

        city: String,

        state: String,

        distance: Distance,

        bearing: CountyClass,

        value: Value,

        unit_code: Distance,

        forecast_office: CountyClass,

        forecast_grid_data: CountyClass,

        public_zone: CountyClass,

        county: CountyClass,
    }

    #[derive(Serialize, Deserialize)]
    pub struct CountyClass {
        #[serde(rename = "@type")]
        bearing_type: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Distance {
        #[serde(rename = "@id")]
        id: String,

        #[serde(rename = "@type")]
        distance_type: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Value {
        #[serde(rename = "@id")]
        id: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Geometry {
        #[serde(rename = "type")]
        geometry_type: String,

        coordinates: Vec<f64>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GridJsonProperties {
        #[serde(rename = "@id")]
        id: String,

        #[serde(rename = "@type")]
        properties_type: String,

        cwa: String,

        pub forecast_office: String,

        pub grid_id: String,

        pub grid_x: i64,

        pub grid_y: i64,

        pub forecast: String,

        forecast_hourly: String,

        forecast_grid_data: String,

        observation_stations: String,

        relative_location: RelativeLocation,

        forecast_zone: String,

        time_zone: String,

        radar_station: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RelativeLocation {
        #[serde(rename = "type")]
        relative_location_type: String,

        geometry: Geometry,

        properties: RelativeLocationProperties,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RelativeLocationProperties {
        city: String,

        state: String,

        distance: DistanceClass,

        bearing: DistanceClass,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DistanceClass {
        unit_code: String,

        value: f64,
    }   
}
