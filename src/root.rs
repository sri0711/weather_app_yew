use crate::pages::index_page;
use yew::prelude::*;
use yew_router::{prelude::*, switch};

#[function_component]
pub fn App() -> Html {
    html! {
         <BrowserRouter>
            <Switch<Route> render={switch} /> // router configurations <BrowserRouter>
        </BrowserRouter>
    }
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Index,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Index => html! {<index_page::IndexPage/>},
        Route::About => html! { <h1>{ "About" }</h1> },
        Route::Contact => html! { <h1>{ "Contact" }</h1> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
