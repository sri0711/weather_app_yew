use crate::components::header;
use yew::prelude::*;
use yew_hooks::prelude::*;

struct LocationObject {
    lat: f64,
    long: f64,
}

#[function_component]
pub fn IndexPage() -> Html {
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
        <>
            <header::Header/>
            <h1>{ "Welcome to the Weather App!" }</h1>
            <h1>{get_location.loading}</h1>
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
        </>
    }
}
