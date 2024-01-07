use std::rc::Rc;
use gloo::dialogs::alert;
use material_yew::button::MatButton;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;
use yewdux::dispatch::Dispatch;

use crate::{components::button::{MatUpdateProfileButton, MatUpdatePasswordButton, MatDeleteProfileButton}, State, api::user::logout_user, routes::Route};


pub struct ProfilePage{
    state: Rc<State>,
    dispatch: Dispatch<State>
}

pub enum Msg {
    StateChanged(Rc<State>),
    LogOut
}

impl Component for ProfilePage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(Msg::StateChanged);
        let dispatch = Dispatch::<State>::subscribe_silent(callback);

        Self {
            state: dispatch.get(),
            dispatch
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StateChanged(state) => self.state = state,
            Msg::LogOut => {

                let navigator = ctx.link().navigator().unwrap().clone();
                let dispatch = self.dispatch.clone();

                spawn_local(async move {
                    let logout = logout_user().await;

                    if logout.is_err() {
                        alert(&"Error! Wasn't able to log out user!");
                        return;
                    }

                    dispatch.reduce_mut(|state| state.auth = None);
                    navigator.push(&Route::LoginPage)
                });

                return true;
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let user = &self.state.auth;

        html! {
            <div align="center">
                <h2> {"Your Profile"} </h2>
                if let Some(user) = user {
                    <p> {format!("Email: {}", user.email)} </p>
                    <p> {format!("Username: {}", user.username)} </p>
                    <MatUpdateProfileButton/>
                    <MatUpdatePasswordButton/>
                    <MatDeleteProfileButton/>
                    <p onclick={ctx.link().callback(|_| Msg::LogOut)}><MatButton label="Log Out"/></p>
                }
            </div>
        }
    }
}
