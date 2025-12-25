use reqwest;

pub async fn get_latitude_longitude(cityName: &String, applicationKey: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("http://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid={}", cityName, applicationKey);
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}
