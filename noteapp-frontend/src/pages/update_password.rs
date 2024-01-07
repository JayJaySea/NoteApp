use std::rc::Rc;

use gloo::dialogs::alert;
use material_yew::{text_inputs::{MatTextField, TextFieldType}, button::MatButton, tabs::{MatTabBar, MatTab}};
use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*, html::Scope};
use yewdux::{functional::use_store, dispatch::Dispatch};

use crate::{api::{types::{CreateUser, LoginUser, User, UpdatePassword}, user::{register_user, login_user, update_password}}, components::button::MatLoginButton, State};


pub struct UpdatePasswordPage {
    form: UpdatePassword,
}

pub enum Msg {
    SubmitForm,
    UpdateCurrentPassword(String),
    UpdateNewPassword(String),
}

impl Component for UpdatePasswordPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            form: UpdatePassword::default(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SubmitForm => {
                let cloned_form = self.form.clone();
                spawn_local(async move {
                    let response = update_password(cloned_form).await;

                    if response.is_err() {
                        alert(&"Incorrect password, try again");
                        return;
                    }
                });

                return true;
            }
            Msg::UpdateCurrentPassword(value) => self.form.password = value,
            Msg::UpdateNewPassword(value) => self.form.new_password = value,
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div align="center">
                <h2> {"Update your password"} </h2>
                <MatTextField 
                    field_type={TextFieldType::Password} 
                    label="Current Password" 
                    oninput={ctx.link().callback(|s: String| Msg::UpdateCurrentPassword(s))}/><br/>
                <MatTextField 
                    field_type={TextFieldType::Password} 
                    oninput={ctx.link().callback(|s: String| Msg::UpdateNewPassword(s))}
                    label="New Password" /><br/>
                <p onclick={ctx.link().callback(|_| Msg::SubmitForm)}><MatButton label="Update Password"/></p>
            </div>
        }
    }
}
