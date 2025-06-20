use yew::prelude::*;

pub struct WeatherPage {
    pub lat: String,
    pub long: String,
}

impl Component for WeatherPage {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            lat: "".to_owned(),
            long: "".to_owned(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(<>
            {"hi"}
            </>)
    }
}
