use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use noteapp_frontend::{routes::{switch, Route}, components::navigator::{MatPreauthNavigatorTab, MatAuthNavigatorTab}, api::{types::User, user::get_user}, State};
use yewdux::prelude::*;


#[function_component(App)]
fn app() -> Html {
    let (state, dispatch) = use_store::<State>();

    spawn_local(async move {
        let user = get_user().await;
        if let Ok(user) = user {
            dispatch.reduce_mut(|state| state.auth = Some(user));
        }
    });

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
