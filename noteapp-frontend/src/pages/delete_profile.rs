use std::rc::Rc;

use gloo::dialogs::alert;
use material_yew::{text_inputs::{MatTextField, TextFieldType}, button::MatButton, tabs::{MatTabBar, MatTab}};
use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*, html::Scope};
use yew_router::{hooks::use_navigator, scope_ext::RouterScopeExt};
use yewdux::{functional::use_store, dispatch::Dispatch};

use crate::{api::{types::{CreateUser, LoginUser, User, UpdateUser, AnyPassword}, user::{register_user, login_user, update_user, delete_user}}, components::button::MatLoginButton, State};


pub struct DeleteProfilePage {
    form: AnyPassword,
    dispatch: Dispatch<State>,
    state: Rc<State>,
}

pub enum Msg {
    StateChanged(Rc<State>),
    SubmitForm,
    UpdatePassword(String),
}

impl Component for DeleteProfilePage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(Msg::StateChanged);
        let dispatch = Dispatch::<State>::subscribe_silent(callback);

        Self {
            form: AnyPassword::default(),
            state: dispatch.get(),
            dispatch,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StateChanged(state) => {
                self.state = state
            }
            Msg::SubmitForm => {
                let cloned_form = self.form.clone();
                let dispatch = self.dispatch.clone();
                spawn_local(async move {
                    let response = delete_user(cloned_form).await;

                    if response.is_err() {
                        alert(&"Incorrect password, try again");
                        return;
                    }

                    dispatch.reduce_mut(|state| state.auth = None);
                });

                return true;
            }
            Msg::UpdatePassword(value) => self.form.password = value,
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div align="center">
                <h2> {"Are you sure? Type in your password to confirm."} </h2>
                <MatTextField 
                    oninput={ctx.link().callback(|s: String| Msg::UpdatePassword(s))}
                    field_type={TextFieldType::Password} 
                    label="Current Password" /><br/>
                <p onclick={ctx.link().callback(|_| Msg::SubmitForm)}><MatButton label="Delete Profile"/></p>
            </div>
        }
    }
}
