use std::rc::Rc;

use async_std::task::block_on;
use material_yew::{text_inputs::{MatTextField, MatTextArea} };
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
use yew::{prelude::*, html::Scope};
use yew_router::scope_ext::RouterScopeExt;
use yewdux::dispatch::Dispatch;

use crate::{api::{types::{User, Note, CreateNote}, note::{get_notes, add_note}}, components::{button::{MatUpdateProfileButton, MatUpdatePasswordButton, MatFabButton}, note::{MatNote, MatNoteList}}, State, routes::Route};


pub struct NotesPage {
    state: Rc<State>,
    dispatch: Dispatch<State>,
    adding: bool,
    new_note_form: CreateNote
}

pub enum Msg {
    StateChanged(Rc<State>),
    NoteRefresh,
    UpdateAdding(bool),
    UpdateNewLabel(String),
    UpdateNewContents(String),
    SubmitNewNote,
}

impl Component for NotesPage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(Msg::StateChanged);
        let dispatch = Dispatch::<State>::subscribe_silent(callback);

        Self {
            state: dispatch.get(),
            dispatch,
            adding: false,
            new_note_form: CreateNote::default(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StateChanged(state) => self.state = state,
            Msg::NoteRefresh => return true,
            Msg::UpdateAdding(adding) => {
                self.adding = adding;
                return true;
            },
            Msg::UpdateNewLabel(label) => self.new_note_form.label = label,
            Msg::UpdateNewContents(contents) => self.new_note_form.contents = contents,
            Msg::SubmitNewNote => {
                let cloned_form = self.new_note_form.clone();

                block_on(async move {
                    let note = add_note(cloned_form).await;

                    if note.is_err() {
                        todo!()
                    }
                });

                self.new_note_form = CreateNote::default();
                self.adding = false;

                return true;
            }
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let navigator = ctx.link().navigator().unwrap();

        let go_add_page = Callback::from(move |_| navigator.push(&Route::AddNotePage));
        html! {
            <div align="center">
                <h2> {"Your Notes"} </h2>
                <MatFabButton icon="add" onclick={go_add_page}/>
                <MatNoteList/>
            </div>
        }
    }
}

