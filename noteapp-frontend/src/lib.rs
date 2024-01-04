use api::types::User;
use yewdux::store::Store;

pub mod api;
pub mod components;
pub mod routes;
pub mod pages;
pub mod store;

#[derive(Default, Clone, PartialEq, Store)]
pub struct State {
    pub auth: Option<User>,
}
