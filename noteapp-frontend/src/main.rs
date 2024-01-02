use material_yew::tabs::{MatTabBar, MatTab};
use yew::prelude::*;
use yew_router::prelude::*;
use noteapp_frontend::routes::{switch, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
        <MatTabBar>
            <MatTab icon="login" label="Login" stacked=true />
            <MatTab icon="app_registration" label="Register" stacked=true />
        </MatTabBar>
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
