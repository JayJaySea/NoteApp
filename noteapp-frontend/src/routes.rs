use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{register::RegisterPage, login::LoginPage, profile::ProfilePage, update_profile::UpdateProfilePage, update_password::UpdatePasswordPage, notes::NotesPage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/register")]
    RegisterPage,
    #[at("/login")]
    LoginPage,
    #[at("/profile")]
    ProfilePage,
    #[at("/update_profile")]
    UpdateProfilePage,
    #[at("/update_password")]
    UpdatePasswordPage,
    #[at("/notes")]
    NotesPage,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::RegisterPage => html! {<RegisterPage/> },
        Route::LoginPage => html! {<LoginPage/> },
        Route::ProfilePage => html! {<ProfilePage/> },
        Route::UpdateProfilePage => html! {<UpdateProfilePage/> },
        Route::UpdatePasswordPage => html! {<UpdatePasswordPage/> },
        Route::NotesPage => html! {<NotesPage/>},
    }
}
