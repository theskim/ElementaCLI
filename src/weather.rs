use std::env;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use reqwest::Url;

#[derive(Debug, Deserialize, Serialize)]
struct WeatherResponse {
    coord: Coord,
    weather: Vec<Weather>,
    base: String,
    main: Main,
    visibility: u32,
    wind: Wind,
    clouds: Clouds,
    dt: u64,
    sys: Sys,
    timezone: i32,
    id: u32,
    name: String,
    cod: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: u32,
    humidity: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Wind {
    speed: f64,
    deg: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Clouds {
    all: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Sys {
    country: String,
    sunrise: u64,
    sunset: u64,
}

pub async fn get_weather() -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve API key and city from command-line arguments
    let args: Vec<String> = env::args().collect();
    let api_key = &args[1];
    let city: &String = &args[2];

    // Create a new reqwest client
    let client = Client::new();

    // Fetch weather forecast
    let my_weather: WeatherResponse = fetch_weather(&client, api_key, city).await?;

    // Display weather forecast information
    println!("Weather Forecast for {}, {} ({}, {}):", 
        my_weather.name, my_weather.sys.country, my_weather.coord.lat, my_weather.coord.lon);
    println!("Current Status: {}", my_weather.weather[0].main);
    println!("Current Temperature: {:.2}°C", my_weather.main.temp - 273.15);
    println!("Feels like: {:.2}°C", my_weather.main.feels_like - 273.15);
    println!("Maximum Temperature: {:.2}°C", my_weather.main.temp_max - 273.15);
    println!("Minimum Temperature: {:.2}°C", my_weather.main.temp_min - 273.15);
    println!("Humidity: {:.2} %", my_weather.main.humidity);
    println!("Wind: {}m/s, {:}", my_weather.wind.speed, deg_to_compass(my_weather.wind.deg).unwrap());
    println!("Cloud: {}%", my_weather.clouds.all);
    println!("Visibility: {:.2} m", my_weather.visibility);
    println!("Pressure: {:.2} hPa", my_weather.main.pressure);

    Ok(())
}

fn deg_to_compass(degree: u32) -> Result<String, reqwest::Error> {
    let compass: String;

    match degree {
        0 => compass = "N ↑".to_string(),
        1..=44 => compass = format!("N {}° E ↗", degree),
        45 => compass = "NE ↗".to_string(),
        46..=89 => compass = format!("N {}° E ↗", 90 - degree),
        90 => compass = "E ➡".to_string(),
        91..=134 => compass = format!("S {}° E ↘", degree - 90),
        135 => compass = "SE ↘".to_string(),
        136..=179 => compass = format!("S {}° E ↘", 180 - degree),
        180 => compass = "S ↓".to_string(),
        181..=224 => compass = format!("S {}° W ↙", degree - 180),
        225 => compass = "SW ↙".to_string(),
        226..=269 => compass = format!("S {}° W ↙", 270 - degree),
        270 => compass = "W ←".to_string(),
        271..=314 => compass = format!("N {}° W ↖", degree - 270),
        315 => compass = "NW ↖".to_string(),
        316..=359 => compass = format!("N {}° E ↖", 360 - degree),
        _ => panic!("Invalid degree value"),
    }

    Ok(compass)
}

async fn fetch_weather(
    client: &Client, 
    api_key: &String, 
    city: &String
) -> Result<WeatherResponse, reqwest::Error> {
    let url = Url::parse(&format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city, api_key
    )).expect("API failure");

    let response = client.get(url).send().await?.json::<WeatherResponse>().await?;
    Ok(response)
}