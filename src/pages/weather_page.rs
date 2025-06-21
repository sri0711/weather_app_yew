use yew::prelude::*;

pub struct WeatherPage;

#[derive(Properties, PartialEq)]
pub struct ParametersWeatherPage {
    pub lat: String,
    pub long: String,
}

impl Component for WeatherPage {
    type Message = ();
    type Properties = ParametersWeatherPage;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(<>
            {ctx.props().lat.clone()}{"<br><p>hi</p>"}
            {ctx.props().long.clone()}
            </>)
    }
}
