use gloo::console::log;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct PlaceToLatLong {
    pub name: String,
    pub local_names: LocalNames,
    pub lat: f64,
    pub lon: f64,
    pub country: String,
    pub state: String,
}

#[derive(Serialize, Deserialize)]
pub struct LocalNames {
    pub de: String,
    pub ur: String,
    pub ta: String,
    pub en: String,
    pub pl: String,
    pub lt: String,
    pub ja: String,
    pub tr: String,
    pub zh: String,
    pub ko: String,
    pub te: String,
    pub cs: String,
    pub hi: String,
    pub bn: String,
    pub he: String,
    pub mr: String,
    pub ar: String,
    pub kn: String,
    pub uk: String,
    pub ru: String,
    pub ml: String,
    pub fr: String,
}

pub async fn get_place_to_lat_long(place: String) -> PlaceToLatLong {
    let url = format!(
        "https://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid=3f04ddd0162fe6a258a0cb36c44b305b",
        place
    );
    let response = reqwest::get(url).await.unwrap().text().await.unwrap();
    let response: Vec<PlaceToLatLong> = serde_json::from_str(&response.to_owned()).unwrap();
    response.into_iter().next().unwrap()
}
