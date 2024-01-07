use material_yew::tabs::{MatTabBar, MatTab};
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::routes::Route;

#[function_component(MatPreauthNavigatorTab)]
pub fn mat_preauth_nav() -> Html {
    let navigator = use_navigator().unwrap();
    let go_login = {
        let navigator = navigator.clone();

        Callback::from(move |_: String| navigator.push(&Route::LoginPage))
    };

    navigator.push(&Route::LoginPage);

    let go_register = {
        let navigator = navigator.clone();
        Callback::from(move |_: String| navigator.push(&Route::RegisterPage))
    };

    html! {
        <MatTabBar>
            <MatTab icon="login" label="Login" stacked=true oninteracted={go_login} />
            <MatTab icon="app_registration" label="Register" stacked=true  oninteracted={go_register}/>
        </MatTabBar>
    }
}

#[function_component(MatAuthNavigatorTab)]
pub fn mat_auth_nav() -> Html {
    let navigator = use_navigator().unwrap();
    let go_profile = {
        let navigator = navigator.clone();

        Callback::from(move |_: String| navigator.push(&Route::ProfilePage))
    };

    navigator.push(&Route::NotesPage);

    let go_notes = {
        let navigator = navigator.clone();
        Callback::from(move |_: String| navigator.push(&Route::NotesPage))
    };

    html! {
        <MatTabBar>
            <MatTab icon="notes" label="Notes" stacked=true  oninteracted={go_notes}/>
            <MatTab icon="person" label="Profile" stacked=true oninteracted={go_profile} />
        </MatTabBar>
    }
}
