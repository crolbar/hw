use reqwest;
extern crate tokio;
extern crate serde_json;
use serde_json::Value;


async fn fetch_weather_data(location: &str) -> Result<String, reqwest::Error> {
    let api_key = "";
    let url = format!("http://api.weatherapi.com/v1/current.json?key={}&q={}", api_key, location);

    let response_text = reqwest::get(url).await?.text().await?;

    Ok(response_text)
}


#[tokio::main]
async fn print_weather(location: &str) {

    let json_result = fetch_weather_data(location).await;

    match json_result {
        Ok(data) => {
            // println!("{}", data);
            let json_result: Value = serde_json::from_str(&data).unwrap_or_else(|_| {
                println!("Error parsing JSON response.");
                serde_json::Value::Null
            });

            if let (Some(temp), Some(city), Some(condition), Some(wind_speed), Some(wind_dir)) = (
                json_result["current"]["temp_c"].as_f64(),
                json_result["location"]["name"].as_str(),
                json_result["current"]["condition"]["text"].as_str(),
                json_result["current"]["wind_kph"].as_f64(),
                json_result["current"]["wind_dir"].as_str(),
            ) {
                println!("City: {}", city);
                println!("Temperature: {}°C", temp);
                println!("Condition: {}", condition);
                println!("Wind Speed: {} kph", wind_speed);
                println!("Wind Direction: {}", wind_dir);
            } else {
                println!("Some required fields are missing in the JSON response.");
            }
        }
        Err(e) => println!("Error fetching weather data: {}", e),
    }

}


pub fn get_weather() {
    let location = "";
    print_weather(location);
}

pub fn get_weather_city(location: &str) {
    print_weather(location);
}
