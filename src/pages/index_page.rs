use std::str::FromStr;

use crate::components::header;
use gloo::console::log;
use serde_json::from_str;
use stylist::{ast::Sheet, yew::styled_component, Style};
use yew::prelude::*;
use yew_hooks::prelude::*;
struct LocationObject {
    lat: f64,
    long: f64,
}

#[styled_component]
pub fn IndexPage() -> Html {
    let css = include_str!("./index.css");
    let sheet = Sheet::from_str(css).unwrap();
    let style = Style::new(sheet).unwrap();
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

    html! {
        <div>
        <div class="root">
            <header::Header/>
            <h1>{ "Welcome to the Weather App!" }</h1>
            <h2>{get_location.loading}</h2>
            {
                if let Some(location) = location_data {
                    html! {
                        <>
                            <h1>{location.lat}</h1>
                            <h1>{location.long}</h1>
                        </>
                    }
                } else {
                    html! {
                        <h1>{ "Location data is loading..." }</h1>
                    }
                }
            }
            <p>{ "This is the home page." }</p>
        </div>
    </div>
    }
}
