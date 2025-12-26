use std::env;
mod get_lat_long;
mod get_weather_report;
use get_lat_long::get_latitude_longitude;
use get_weather_report::get_simple_weather;


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let city = &args[1];
    let weather_type = &args[2];
    let application_key = "a25e026039a4ee20f206ce5b9bb1b59b";
    let mut latitude = 0.0;
    let mut longitude = 0.0;
    match get_latitude_longitude(city, application_key).await {
        Ok(json) => {
            let data = &json[0];
             latitude = data["lat"].as_f64().unwrap_or(0.0);
             longitude = data["lon"].as_f64().unwrap_or(0.0);
        }
        Err(e) => eprintln!("Error :{}", e),
    }

    if weather_type == "normal" {
        match get_simple_weather(latitude, longitude, application_key).await {
            Ok(json) => {
                println!("{}", json);
            }
            Err(e) => eprintln!("Error :{}", e)
        }
    }
}
