use std::rc::Rc;

use async_std::task::block_on;
use material_yew::text_inputs::{MatTextField, MatTextArea};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
use yew::{prelude::*, html::Scope};
use yew_router::scope_ext::RouterScopeExt;
use yewdux::dispatch::Dispatch;

use crate::{api::{types::{User, Note, CreateNote}, note::{get_notes, add_note}}, components::{button::{ MatUpdateProfileButton, MatUpdatePasswordButton, MatFabButton}, note::{MatNote, MatNoteList}}, State, routes::Route};

pub struct AddNotePage {
    state: Rc<State>,
    dispatch: Dispatch<State>,
    form: CreateNote
}

pub enum Msg {
    StateChanged(Rc<State>),
    UpdateLabel(String),
    UpdateContents(String),
    SubmitForm,
}

impl Component for AddNotePage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(Msg::StateChanged);
        let dispatch = Dispatch::<State>::subscribe_silent(callback);

        Self {
            state: dispatch.get(),
            dispatch,
            form: CreateNote::default(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StateChanged(state) => self.state = state,
            Msg::UpdateLabel(label) => self.form.label = label,
            Msg::UpdateContents(contents) => self.form.contents = contents,
            Msg::SubmitForm => {
                let cloned_form = self.form.clone();

                spawn_local(async move {
                    let note = add_note(cloned_form).await;

                    if note.is_err() {
                        todo!()
                    }
                });

                self.form = CreateNote::default();

                return true;
            }
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let navigator = ctx.link().navigator().unwrap();
        let go_back = Callback::from(move |_| navigator.push(&Route::NotesPage));

        html! {
            <div align="center">
                    <h1> {"Add new note"} </h1>
                    <MatFabButton icon="arrow_back" onclick={go_back}/>

                    <div align="center" style="padding: 2px; border-style: solid; border-width: 2px; border-color: blue; border-radius: 10px; width: 95%; margin-bottom: 10px">
                        <MatTextField 
                            label="Label" 
                            size=1000
                            oninput={ctx.link().callback(|s: String| Msg::UpdateLabel(s))}/><br/>
                        <MatTextArea 
                            label="Contents" 
                            helper="Your note contents"
                            cols=1000
                            rows=5
                            oninput={ctx.link().callback(|s: String| Msg::UpdateContents(s))}/><br/>

                        <MatFabButton onclick={ctx.link().callback(|_| Msg::SubmitForm)} icon="check"/>
                    </div>
            </div>
        }
    }
}

