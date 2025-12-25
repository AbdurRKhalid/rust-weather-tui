use reqwest;
use serde_json::Value;

pub async fn get_latitude_longitude(city_name: &String, application_key: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let url = format!("http://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid={}", city_name, application_key);
    let response = reqwest::get(url).await?;
    let json: Value = response.json().await?;
    Ok(json)
}
