use crate::geocode;

#[tokio::main]
async fn main() {

    let echo_json = geocode::get_grids(34.052235, -118.243684).await;
    println!("{:#?}", echo_json);


}
