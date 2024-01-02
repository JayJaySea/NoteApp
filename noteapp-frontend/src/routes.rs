use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::register::RegisterPage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/register")]
    RegisterPage,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::RegisterPage => html! {<RegisterPage/> },
    }
}
