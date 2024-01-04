use material_yew::{text_inputs::{MatTextField, TextFieldType}, button::MatButton, tabs::{MatTabBar, MatTab}};
use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*, html::Scope};
use yew_router::hooks::use_navigator;

use crate::{api::{types::{CreateUser, LoginUser}, user::{register_user, login_user}}, components::button::MatLoginButton};


pub struct LoginPage {
    form: LoginFormData,
}

#[derive(Default, Clone)]
struct LoginFormData {
    email: String,
    password: String,
}

pub enum Msg {
    SubmitForm,
    UpdateEmail(String),
    UpdatePassword(String),
}

impl Component for LoginPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            form: LoginFormData::default()
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SubmitForm => {
                let cloned_form = self.form.clone();
                spawn_local(async move {
                    let token = login_user(cloned_form.into()).await;

                    // if let Ok(_) = token {

                    // }
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
                <MatLoginButton/>
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
