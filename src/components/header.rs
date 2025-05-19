use yew::prelude::*;
use yew_bootstrap::component::{BrandType, NavBar, NavDropdownItem, NavItem};

#[function_component]
pub fn Header() -> Html {
    let brand = BrandType::BrandSimple {
        text: AttrValue::from("Yew Bootstrap"),
        url: Some(AttrValue::from("/")),
    };
    html! {
        <NavBar nav_id={"test-nav"} class="navbar-expand-lg navbar-dark bg-dark" brand={brand}>
            <NavItem text="Home" url={AttrValue::from("/")} />
        </NavBar>
    }
}
