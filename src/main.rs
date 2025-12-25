use std::env;
mod get_lat_long;
use get_lat_long::get_latitude_longitude;


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let city = &args[1];
    let weather_type = &args[3];
    let application_key = "a25e026039a4ee20f206ce5b9bb1b59b";

    match get_latitude_longitude(city, application_key).await{
        Ok(body) => println!("{}", body),
        Err(e) => eprintln!("Error: {}", e),
    }

}
