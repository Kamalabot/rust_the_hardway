use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
struct Coord {
    lon: f32,
    lat: f32,
}

#[derive(Debug, Deserialize, Serialize)]
struct WeatherData {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Main {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    pressure: i32,
    humidity: i32,
    sea_level: i32,
    grnd_level: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Wind {
    speed: f32,
    deg: i32,
    gust: f32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Rain {
    #[serde(rename = "1h")]
    h1: f32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Clouds {
    all: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Sys {
    #[serde(rename = "type")]
    sys_type: i32,
    id: i32,
    country: String,
    sunrise: i64,
    sunset: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    coord: Coord,
    weather: Vec<WeatherData>,
    base: String,
    main: Main,
    visibility: i32,
    wind: Wind,
    rain: Rain,
    clouds: Clouds,
    dt: i64,
    sys: Sys,
    timezone: i32,
    id: i64,
    name: String,
    cod: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("What is the Weather");
    let client = Client::new();
    let url = "https://api.openweathermap.org/data/2.5/weather?lat=44.34&lon=10.99&appid={API_ID}";
    let resp = client.get(url).send()?.text()?;
    println!("{}", resp);
    let weather_obj: Response = from_str(&resp)?;
    println!("{:?}", weather_obj.weather);
    Ok(())
}
