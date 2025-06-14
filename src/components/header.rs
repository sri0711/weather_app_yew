use yew::prelude::*;
use yew_bootstrap::component::{BrandType, Button, NavBar};

#[function_component]
pub fn Header() -> Html {
    let brand = BrandType::BrandSimple {
        text: AttrValue::from("Weather App"),
        url: Some(AttrValue::from("/")),
    };
    html! {
        <NavBar class="navbar-expand-lg navbar-dark bg-dark w-100" brand={brand}>
            <div class="w-100 d-flex ms-auto">
                    <Button class="btn-outline-dark">{"Search"}</Button>
            </div>
        </NavBar>
    }
}
