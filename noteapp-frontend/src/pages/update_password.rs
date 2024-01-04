use std::rc::Rc;

use material_yew::{text_inputs::{MatTextField, TextFieldType}, button::MatButton, tabs::{MatTabBar, MatTab}};
use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*, html::Scope};
use yewdux::{functional::use_store, dispatch::Dispatch};

use crate::{api::{types::{CreateUser, LoginUser, User}, user::{register_user, login_user}}, components::button::MatLoginButton, State};


pub struct UpdatePasswordPage {
    form: UpdatePasswordFormData,
}

#[derive(Default, Clone)]
struct UpdatePasswordFormData {
    current_password: String,
    new_password: String,
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
            form: UpdatePasswordFormData::default(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SubmitForm => {
                let cloned_form = self.form.clone();
                spawn_local(async move {
                    // let token = login_user(cloned_form.into()).await;

                    // if let Ok(_) = token {

                    // }
                });

                return true;
            }
            Msg::UpdateCurrentPassword(value) => self.form.current_password = value,
            Msg::UpdateNewPassword(value) => self.form.new_password = value,
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div align="center">
                <h2> {"Update your password"} </h2>
                <MatTextField 
                    field_type={TextFieldType::Email} 
                    label="Current Password" 
                    oninput={ctx.link().callback(|s: String| Msg::UpdateCurrentPassword(s))}/><br/>
                <MatTextField 
                    oninput={ctx.link().callback(|s: String| Msg::UpdateNewPassword(s))}
                    label="New Password" /><br/>
                <p onclick={ctx.link().callback(|_| Msg::SubmitForm)}><MatButton label="Update Password"/></p>
            </div>
        }
    }
}
