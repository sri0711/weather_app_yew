use crate::{components::header, helpers::locations, pages::weather_page::WeatherPage};
use gloo::console::log;
use serde_json::from_str;
use std::str::FromStr;
use stylist::Style;
use stylist::{ast::Sheet, yew::styled_component};
use yew::prelude::*;
use yew_hooks::prelude::*;
struct LocationObject {
    lat: f64,
    long: f64,
}

#[allow(non_snake_case)]
#[styled_component]
pub fn IndexPage() -> Html {
    // style configurations
    let css = include_str!("./index.css");
    let sheet = Sheet::from_str(css).unwrap();
    let style = Style::new(sheet).unwrap();

    // state for lat and long
    let lat = use_state(|| String::from(""));
    let long = use_state(|| String::from(""));

    // fetch location data
    let options = UseGeolocationOptions::new();
    options.set_enable_high_accuracy(true);
    let get_location = use_geolocation_with_options(options);
    let location_data = if !get_location.loading {
        Some(LocationObject {
            lat: get_location.latitude,
            long: get_location.longitude,
        })
    } else {
        None
    };

    // get place from header
    let lat_handle = lat.clone();
    let long_handle = long.clone();
    let location_input = Callback::from(move |message: String| {
        let lat = lat_handle.clone();
        let long = long_handle.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let responseData = locations::get_place_to_lat_long(message).await;
            lat.set(responseData.lat.to_string());
            long.set(responseData.lon.to_string());
        });
    });

    html! {
        <div class={style}>
        <div class="root">
            <header::Header place={location_input}/>
            {
                if let Some(location) = location_data {
                    html! {
                        <>
                            <WeatherPage lat={ if *lat == "" { get_location.latitude.to_string() } else { lat.to_string() }} long={ if *long == "" { get_location.longitude.to_string() } else { long.to_string() } }/>
                        </>
                    }
                } else {
                    html! {
                        <h1>{ "Location data is loading..." }</h1>
                    }
                }
            }
        </div>
    </div>
    }
}
