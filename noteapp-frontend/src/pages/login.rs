use std::rc::Rc;

use gloo::dialogs::alert;
use material_yew::{text_inputs::{MatTextField, TextFieldType}, button::MatButton, tabs::{MatTabBar, MatTab}};
use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*, html::Scope};
use yew_router::{hooks::use_navigator, scope_ext::RouterScopeExt};
use yewdux::dispatch::Dispatch;

use crate::{api::{types::{CreateUser, LoginUser}, user::{register_user, login_user, get_user}}, components::{button::MatLoginButton, error::ErrorSnackbar}, State, routes::Route};


pub struct LoginPage {
    form: LoginFormData,
    dispatch: Dispatch<State>,
    state: Rc<State>,
}

#[derive(Default, Clone)]
struct LoginFormData {
    email: String,
    password: String,
}

pub enum Msg {
    StateChanged(Rc<State>),
    SubmitForm,
    UpdateEmail(String),
    UpdatePassword(String),
}

impl Component for LoginPage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {

        let callback = ctx.link().callback(Msg::StateChanged);
        let dispatch = Dispatch::<State>::subscribe_silent(callback);

        Self {
            state: dispatch.get(),
            dispatch,
            form: LoginFormData::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StateChanged(state) => {
                self.state = state
            }
            Msg::SubmitForm => {
                let cloned_form = self.form.clone();
                let navigator = ctx.link().navigator().unwrap();
                let dispatch = self.dispatch.clone();

                spawn_local(async move {
                    let token = login_user(cloned_form.into()).await;

                    if token.is_err() {
                        alert(&"Incorrect credentials, try again");
                        return;
                    }

                    let user = get_user().await;
                    if let Ok(user) = user {
                        dispatch.reduce_mut(|state| state.auth = Some(user));
                        navigator.push(&Route::ProfilePage);
                    }
                });

                return true;
            }
            Msg::UpdateEmail(value) => self.form.email = value,
            Msg::UpdatePassword(value) => self.form.password = value,
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div align="center" class="register-page">
                <h2> {"Log in to your account"} </h2>
                <MatTextField 
                    field_type={TextFieldType::Email} 
                    label="Email" 
                    oninput={ctx.link().callback(|s: String| Msg::UpdateEmail(s))}/><br/>
                <MatTextField 
                    oninput={ctx.link().callback(|s: String| Msg::UpdatePassword(s))}
                    field_type={TextFieldType::Password} 
                    label="Password" /><br/>
                <p onclick={ctx.link().callback(|_| Msg::SubmitForm)}><MatButton label="Log In"/></p>
            </div>
        }
    }
}

impl From<LoginFormData> for LoginUser {
    fn from(value: LoginFormData) -> Self {
        Self {
            email: value.email,
            password: value.password
        }
    }
}
