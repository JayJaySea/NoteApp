use gloo::dialogs::alert;
use material_yew::{button::MatButton, fab::MatFab};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yewdux::functional::use_store;

use crate::{routes::Route, State, api::{types::User, user::logout_user}};

#[derive(Properties, PartialEq)]
pub struct SubmitButtonProps {
    pub label: String,
}

#[function_component(SubmitButton)]
pub fn button(props: &SubmitButtonProps) -> Html {
    html! {
        <button type="submit" >
            {props.label.clone()}
        </button>
    }
}

#[function_component(MatLoginButton)]
pub fn login_button() -> Html {
    let navigator = use_navigator().unwrap();
    let (_, dispatch) = use_store::<State>();

    let login = {
        let navigator = navigator.clone();
        let dispatch = dispatch.clone();

        Callback::from(move |_| {

            let user = User {
                id: Uuid::new_v4(),
                email: "example@ex.com".to_string(),
                username: "example".to_string()
            };

            dispatch.reduce_mut(|state| state.auth = Some(user));
            navigator.push(&Route::ProfilePage);
       })
    };

    html! {
        <p onclick={login}><MatButton label="Log In"/></p>
    }
}

#[function_component(MatUpdateProfileButton)]
pub fn update_profile_button() -> Html {
    let navigator = use_navigator().unwrap();

    let update = {
        let navigator = navigator.clone();

        Callback::from(move |_| {
            navigator.push(&Route::UpdateProfilePage)
        })
    };


    html! {
        <p onclick={update}><MatButton label="Update Profile"/></p>
    }
}

#[function_component(MatDeleteProfileButton)]
pub fn update_delete_button() -> Html {
    let navigator = use_navigator().unwrap();

    let delete = {
        let navigator = navigator.clone();

        Callback::from(move |_| {
            navigator.push(&Route::DeleteProfilePage)
        })
    };


    html! {
        <p onclick={delete}><MatButton label="Delete Profile"/></p>
    }
}

#[function_component(MatUpdatePasswordButton)]
pub fn update_password_button() -> Html {
    let navigator = use_navigator().unwrap();

    let update = {
        let navigator = navigator.clone();

        Callback::from(move |_| {
            navigator.push(&Route::UpdatePasswordPage)
        })
    };

    html! {
        <p onclick={update}><MatButton label="Update Password"/></p>
    }
}


#[derive(Properties, PartialEq)]
pub struct IconButtonProps {
    pub onclick: Callback<MouseEvent>,
    pub icon: AttrValue
}

#[function_component(MatFabButton)]
pub fn update_button(props: &IconButtonProps) -> Html {

    html! {
        <span onclick={props.onclick.clone()}><MatFab icon={props.icon.clone()} mini=true/></span>
    }
}

