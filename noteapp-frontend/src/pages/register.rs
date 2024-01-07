use gloo::dialogs::alert;
use material_yew::{text_inputs::{MatTextField, TextFieldType}, button::MatButton, tabs::{MatTabBar, MatTab}};
use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*, html::Scope};

use crate::api::{types::CreateUser, user::register_user};


pub struct RegisterPage {
    form: RegisterFormData,
}

#[derive(Default, Clone)]
struct RegisterFormData {
    email: String,
    username: String,
    password: String,
    confirm_password: String
}

pub enum Msg {
    SubmitForm,
    UpdateUsername(String),
    UpdateEmail(String),
    UpdatePassword(String),
    UpdateConfirmPassword(String),
}

impl Component for RegisterPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            form: RegisterFormData::default()
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SubmitForm => {
                if self.form.password != self.form.confirm_password {
                    return false;
                }

                let cloned_form = self.form.clone();
                spawn_local(async move {
                    let response = register_user(cloned_form.into()).await;

                    if response.is_err() {
                        alert("Invalid user data, try again!");
                    }
                });

                return true;
            }
            Msg::UpdateUsername(value) => self.form.username = value,
            Msg::UpdateEmail(value) => self.form.email = value,
            Msg::UpdatePassword(value) => self.form.password = value,
            Msg::UpdateConfirmPassword(value) => self.form.confirm_password = value,
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div align="center" class="register-page">
                <h2> {"Create new account"} </h2>
                <MatTextField 
                    field_type={TextFieldType::Email} 
                    label="Email" 
                    oninput={ctx.link().callback(|s: String| Msg::UpdateEmail(s))}/><br/>
                <MatTextField 
                    oninput={ctx.link().callback(|s: String| Msg::UpdateUsername(s))}
                    label="Username" /><br/>
                <MatTextField 
                    oninput={ctx.link().callback(|s: String| Msg::UpdatePassword(s))}
                    field_type={TextFieldType::Password} 
                    label="Password" /><br/>
                <MatTextField 
                    oninput={ctx.link().callback(|s: String| Msg::UpdateConfirmPassword(s))}
                    field_type={TextFieldType::Password} 
                    label="Confirm Password" /><br/>
                <p onclick={ctx.link().callback(|_| Msg::SubmitForm)}><MatButton label="Register"/></p>
            </div>
        }
    }
}

impl From<RegisterFormData> for CreateUser {
    fn from(value: RegisterFormData) -> Self {
        CreateUser {
            email: value.email,
            username: value.username,
            password: value.password
        }
    }
}
