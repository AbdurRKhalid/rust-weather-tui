use reqwest;

pub async fn get_latitude_longitude(city_name: &String, application_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("http://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid={}", city_name, application_key);
    println!("{}", url);
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}
