use gloo::console::log;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct PlaceToLatLong {
    pub name: Option<String>,
    pub local_names: Option<LocalNames>,
    pub lat: f64,
    pub lon: f64,
    pub country: Option<String>,
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LocalNames {
    pub de: Option<String>,
    pub ur: Option<String>,
    pub ta: Option<String>,
    pub en: Option<String>,
    pub pl: Option<String>,
    pub lt: Option<String>,
    pub ja: Option<String>,
    pub tr: Option<String>,
    pub zh: Option<String>,
    pub ko: Option<String>,
    pub te: Option<String>,
    pub cs: Option<String>,
    pub hi: Option<String>,
    pub bn: Option<String>,
    pub he: Option<String>,
    pub mr: Option<String>,
    pub ar: Option<String>,
    pub kn: Option<String>,
    pub uk: Option<String>,
    pub ru: Option<String>,
    pub ml: Option<String>,
    pub fr: Option<String>,
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
