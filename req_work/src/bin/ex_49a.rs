use reqwest::blocking::Client;
use std::error::Error;
use std::io::stdin;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Getting the geolocation lat / lon");
    let client = Client::new();
    let mut city_rw = String::new();
    println!("Provide the city: ");
    stdin().read_line(&mut city_rw)?;
    let city = city_rw.trim();
    let api_key = "API_ID";
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={city}&appid={api_key}");
    let resp = client.get(url).send()?.text()?;
    println!("{}", resp);
    Ok(())
}
