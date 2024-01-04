use yew::prelude::*;
use yew_router::prelude::*;
use noteapp_frontend::{routes::{switch, Route}, components::navigator::{MatPreauthNavigatorTab, MatAuthNavigatorTab}, api::types::User, State};
use yewdux::prelude::*;


#[function_component(App)]
fn app() -> Html {
    let (state, _) = use_store::<State>();

    html! {
        <div>
        <BrowserRouter>
            if state.auth.is_some() {
                <MatAuthNavigatorTab/>
            }
            else {
                <MatPreauthNavigatorTab/>
            }
            <Switch<Route> render={switch} />
        </BrowserRouter>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
