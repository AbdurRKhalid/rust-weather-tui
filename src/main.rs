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
                let weather_data = &json;
                let current_temp = &weather_data["main"]["temp"];
                let feels_like_temp = &weather_data["main"]["feels_like"];
                let min_temp = &weather_data["main"]["temp_min"];
                let max_temp = &weather_data["main"]["temp_max"];
                let humidity = &weather_data["main"]["humidity"];
                let main_weather_type = &weather_data["weather"][0]["main"];
                let wind_speed = &weather_data["wind"]["speed"];
                println!("look outside window it is {}", main_weather_type);
                if main_weather_type == "Mist" {
                    println!("\x1b[90m     ___  \x1b[0m");
                    println!("\x1b[90m  _(   )_\x1b[0m");
                    println!("\x1b[90m (       )\x1b[0m");
                    println!("\x1b[90m  ~~~~~~~\x1b[0m");
                    println!("\x1b[90m   Mist\x1b[0m");
                }
                println!("\n{:<20} | {}", "Property", "Value");
                println!("{:-<20}-+-{:-<15}", "", "");
                println!("{:<20} | {}", "Temperature", format!("{}°C",current_temp));
                println!("{:<20} | {}", "Feels Like", format!("{}°C", feels_like_temp));
                println!("{:<20} | {}", "Min Temperature", format!("{}°C", min_temp));
                println!("{:<20} | {}", "Max Temperature", format!("{}°C", max_temp));
                println!("{:<20} | {}", "Humidity", format!("{}%", humidity));
                println!("{:<20} | {}", "Wind Speed", format!("{} m/s", wind_speed));
            }
            Err(e) => eprintln!("Error :{}", e)
        }
    }
}
