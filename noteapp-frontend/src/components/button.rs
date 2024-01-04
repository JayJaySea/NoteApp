use material_yew::{button::MatButton, fab::MatFab};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yewdux::functional::use_store;

use crate::{routes::Route, State, api::types::User};

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

#[function_component(MatLogoutButton)]
pub fn logout_button() -> Html {
    let navigator = use_navigator().unwrap();
    let (_, dispatch) = use_store::<State>();

    let logout = {
        let navigator = navigator.clone();
        let dispatch = dispatch.clone();

        Callback::from(move |_| {
            dispatch.reduce_mut(|state| state.auth = None);
            navigator.push(&Route::LoginPage)
        })
    };


    html! {
        <p onclick={logout}><MatButton label="Log Out"/></p>
    }
}

#[function_component(MatUpdateProfileButton)]
pub fn update_profile_button() -> Html {
    let navigator = use_navigator().unwrap();

    let logout = {
        let navigator = navigator.clone();

        Callback::from(move |_| {
            navigator.push(&Route::UpdateProfilePage)
        })
    };


    html! {
        <p onclick={logout}><MatButton label="Update Profile"/></p>
    }
}

#[function_component(MatUpdatePasswordButton)]
pub fn update_password_button() -> Html {
    let navigator = use_navigator().unwrap();

    let logout = {
        let navigator = navigator.clone();

        Callback::from(move |_| {
            navigator.push(&Route::UpdatePasswordPage)
        })
    };

    html! {
        <p onclick={logout}><MatButton label="Update Password"/></p>
    }
}


#[derive(Properties, PartialEq)]
pub struct ClickableButtonProps {
    pub onclick: Callback<MouseEvent>,
}

#[function_component(MatUpdateButton)]
pub fn update_button(props: &ClickableButtonProps) -> Html {

    html! {
        <span onclick={props.onclick.clone()}><MatFab icon="edit" mini=true/></span>
    }
}


#[function_component(MatDeleteButton)]
pub fn delete_button(props: &ClickableButtonProps) -> Html {

    html! {
        <span onclick={props.onclick.clone()}><MatFab icon="delete" mini=true/></span>
    }
}


#[function_component(MatAddButton)]
pub fn delete_button(props: &ClickableButtonProps) -> Html {

    html! {
        <span onclick={props.onclick.clone()}><MatFab icon="add" mini=true/></span>
    }
}

