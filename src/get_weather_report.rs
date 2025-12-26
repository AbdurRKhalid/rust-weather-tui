use reqwest;
use serde_json::Value;

pub async fn get_simple_weather(latitude: f64, longitude: f64, application_key: &str) -> Result <Value, Box<dyn std::error::Error>> {
    let url = format!("http://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=metric", latitude, longitude, application_key);
    let response = reqwest::get(url).await?;
    let json: Value = response.json().await?;
    Ok(json)
}
