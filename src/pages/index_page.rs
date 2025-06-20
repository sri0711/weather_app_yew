use std::str::FromStr;

use crate::{components::header, pages::weather_page::WeatherPage};
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

    // get place from header
    let location_input = Callback::from(|message| log!("from index page {}", message));

    html! {
        <div class={style}>
        <div class="root">
            <header::Header place={location_input}/>
            {
                if let Some(location) = location_data {
                    html! {
                        <>
                            <WeatherPage/>
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
