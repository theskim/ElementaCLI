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
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve API key and city from command-line arguments
    let args: Vec<String> = env::args().collect();
    let api_key = &args[1];
    let city = &args[2];
    let lat = "43.6532"; // Toronto, ON, Canada
    let lon = "79.3832";

    // Create a new reqwest client
    let client = Client::new();

    // Fetch weather forecast
    let my_weather: WeatherResponse = fetch_weather(&client, api_key, lat, lon).await?;

    // Display weather forecast information
    println!("Weather Forecast for {}:", city);
    println!("Your Location is ({}, {}).", my_weather.coord.lat, my_weather.coord.lon);
    println!("Current Temperature: {}°C", (my_weather.main.feels_like - 273.15).round());
    Ok(())
}

async fn fetch_weather(
    client: &Client,
    api_key: &str,
    lat: &str,
    lon: &str
) -> Result<WeatherResponse, reqwest::Error> {
    let url = Url::parse(&format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
        lat, lon, api_key
    )).expect("API failure");

    let response = client.get(url).send().await?.json::<WeatherResponse>().await?;
    Ok(response)
}