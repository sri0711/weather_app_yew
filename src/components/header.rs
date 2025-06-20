use yew::prelude::*;
use yew_bootstrap::component::{BrandType, Button, NavBar};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub place: Callback<String>,
}

#[function_component]
pub fn Header(props: &Props) -> Html {
    let brand = BrandType::BrandSimple {
        text: AttrValue::from("Weather App"),
        url: Some(AttrValue::from("/")),
    };
    let input_value = use_state(|| String::from(""));
    let on_change = {
        let input_value = input_value.clone();
        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
            if let Some(input) = input {
                input_value.set(input.value());
            }
        })
    };

    let input_value_for_submit = input_value.clone();
    let input_value_for_input = input_value.clone();
    let place_callback = props.place.clone();
    let submit = Callback::from(move |_e: MouseEvent| {
        place_callback.emit((*input_value_for_submit).clone());
        input_value_for_submit.set("".to_owned());
    });

    html! {
        <NavBar class="navbar-expand-lg navbar-dark bg-dark w-100" brand={brand}>
            <div class="w-100 d-flex ms-auto justify-content-end">
                <div class="col-auto mx-2">
                    <input type="text" value={(*input_value_for_input).clone()} onchange={on_change} class="form-control" placeholder="Enter your place"/>
                </div>
                <Button onclick={submit} class="btn-outline-dark">{"Search"}</Button>
            </div>
        </NavBar>
    }
}
