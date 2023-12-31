use std::rc::Rc;

use gloo::dialogs::alert;
use material_yew::{text_inputs::{MatTextField, TextFieldType}, button::MatButton, tabs::{MatTabBar, MatTab}};
use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*, html::Scope};
use yew_router::{hooks::use_navigator, scope_ext::RouterScopeExt};
use yewdux::{functional::use_store, dispatch::Dispatch};

use crate::{api::{types::{CreateUser, LoginUser, User, UpdateUser}, user::{register_user, login_user, update_user, get_user}}, components::button::MatLoginButton, State};


pub struct UpdateProfilePage {
    form: UpdateUser,
    dispatch: Dispatch<State>,
    state: Rc<State>,
}

pub enum Msg {
    StateChanged(Rc<State>),
    SubmitForm,
    UpdateEmail(String),
    UpdateUsername(String),
    UpdatePassword(String),
}

impl Component for UpdateProfilePage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(Msg::StateChanged);
        let dispatch = Dispatch::<State>::subscribe_silent(callback);
        let state = dispatch.get();
        let user = state.auth.clone().unwrap_or_default();

        let form = UpdateUser {
            email: user.email,
            username: user.username,
            password: String::new()
        };

        Self {
            form,
            state,
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
                    let response = update_user(cloned_form).await;

                    if response.is_err() {
                        alert(&"Incorrect password, try again");
                        return;
                    }

                    let user = get_user().await;
                    if let Ok(user) = user {
                        dispatch.reduce_mut(|state| state.auth = Some(user));
                    }
                });

                return true;
            }
            Msg::UpdateEmail(value) => self.form.email = value,
            Msg::UpdateUsername(value) => self.form.username = value,
            Msg::UpdatePassword(value) => self.form.password = value,
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div align="center">
                <h2> {"Update your profile"} </h2>
                <MatTextField 
                    field_type={TextFieldType::Email} 
                    label="Email" 
                    value={self.form.email.clone()}
                    oninput={ctx.link().callback(|s: String| Msg::UpdateEmail(s))}/><br/>
                <MatTextField 
                    oninput={ctx.link().callback(|s: String| Msg::UpdateUsername(s))}
                    value={self.form.username.clone()}
                    label="Username" /><br/>
                <MatTextField 
                    oninput={ctx.link().callback(|s: String| Msg::UpdatePassword(s))}
                    field_type={TextFieldType::Password} 
                    label="Current Password" /><br/>
                <p onclick={ctx.link().callback(|_| Msg::SubmitForm)}><MatButton label="Update Data"/></p>
            </div>
        }
    }
}
