use material_yew::{text_inputs::{MatTextField, TextFieldType}, button::MatButton, tabs::{MatTabBar, MatTab}};
use uuid::Uuid;
use yew::{prelude::*, html::Scope};

use crate::{api::types::User, components::button::{MatLogoutButton, MatUpdateProfileButton, MatUpdatePasswordButton}};


pub struct ProfilePage;

pub enum Msg {
}

impl Component for ProfilePage {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {

        false
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let user = User {
            id: Uuid::new_v4(),
            email: "example@ex.com".to_string(),
            username: "example".to_string()
        };

        html! {
            <div align="center">
                <h2> {"Your Profile"} </h2>
                <p> {format!("Email: {}", user.email)} </p>
                <p> {format!("Username: {}", user.username)} </p>
                <MatUpdateProfileButton/>
                <MatUpdatePasswordButton/>
                <MatLogoutButton/>
            </div>
        }
    }
}
