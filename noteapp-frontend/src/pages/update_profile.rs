use std::rc::Rc;

use material_yew::{text_inputs::{MatTextField, TextFieldType}, button::MatButton, tabs::{MatTabBar, MatTab}};
use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*, html::Scope};
use yewdux::{functional::use_store, dispatch::Dispatch};

use crate::{api::{types::{CreateUser, LoginUser, User}, user::{register_user, login_user}}, components::button::MatLoginButton, State};


pub struct UpdateProfilePage {
    form: UpdateProfileFormData,
    dispatch: Dispatch<State>,
    state: Rc<State>,
}

#[derive(Default, Clone)]
struct UpdateProfileFormData {
    email: String,
    password: String,
}

pub enum Msg {
    StateChanged(Rc<State>),
    SubmitForm,
    UpdateEmail(String),
    UpdateUsername(String),
}

impl Component for UpdateProfilePage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(Msg::StateChanged);
        let dispatch = Dispatch::<State>::subscribe_silent(callback);

        Self {
            form: UpdateProfileFormData::default(),
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
                spawn_local(async move {
                    // let token = login_user(cloned_form.into()).await;

                    // if let Ok(_) = token {

                    // }
                });

                return true;
            }
            Msg::UpdateEmail(value) => self.form.email = value,
            Msg::UpdateUsername(value) => self.form.password = value,
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let user = self.state.auth.clone().unwrap_or(User::default());

        html! {
            <div align="center">
                <h2> {"Update your profile"} </h2>
                <MatTextField 
                    field_type={TextFieldType::Email} 
                    label="Email" 
                    value={user.email}
                    oninput={ctx.link().callback(|s: String| Msg::UpdateEmail(s))}/><br/>
                <MatTextField 
                    oninput={ctx.link().callback(|s: String| Msg::UpdateUsername(s))}
                    value={user.username}
                    label="Username" /><br/>
                <p onclick={ctx.link().callback(|_| Msg::SubmitForm)}><MatButton label="Update Data"/></p>
            </div>
        }
    }
}
