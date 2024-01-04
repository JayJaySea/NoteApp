use uuid::Uuid;
use yew::{prelude::*, html::Scope};

use crate::{api::types::{User, Note}, components::{button::{MatLogoutButton, MatUpdateProfileButton, MatUpdatePasswordButton, MatAddButton}, note::MatNote}};


pub struct NotesPage;

pub enum Msg {
}

impl Component for NotesPage {
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
        let notes = vec![
            Note {
                id: Uuid::new_v4(),
                label: "First Note".to_string(),
                contents: "Lorem Note dolor sit amet".to_string(),
            },
            Note {
                id: Uuid::new_v4(),
                label: "Second Note".to_string(),
                contents: "The rest of lorem\nipsum".to_string(),
            }
        ];

        let notes = notes.iter().map(|note| html! {
            <MatNote label={note.label.clone()} contents={note.contents.clone()}/>
        }).collect::<Html>();

        let add_new_note = { Callback::from(|_| ()) };

        html! {
            <div align="center">
                <h2> {"Your Notes"} </h2>
                <MatAddButton onclick={add_new_note}/>
                {notes}
            </div>
        }
    }
}
