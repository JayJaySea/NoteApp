use material_yew::text_inputs::{MatTextField, MatTextArea};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{api::{note::{get_notes, delete_note, update_note}, types::UpdateNote}, components::button::MatFabButton};


#[function_component(MatNoteList)]
pub fn mat_note_list() -> HtmlResult {

    let notes = use_state(|| vec![]);
        {
            let notes = notes.clone();
            use_effect_with_deps(move |_| {
                let notes = notes.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_notes = get_notes().await;
                    if let Ok(fetched_notes) = fetched_notes {
                        notes.set(fetched_notes);
                    }
                });
                || ()
            }, ());
        }

    let notes = notes.iter().map(|note| html! {
        <MatNote id={note.id} label={note.label.clone()} contents={note.contents.clone()}/>
    }).collect::<Html>();

    Ok(html!{
        <div>
            {notes}
        </div>
    })
}

pub struct MatNote {
    update_form: UpdateNote,
    updating: bool,
    pub deleted: bool
}

pub enum Msg {
    SubmitUpdateForm,
    UpdatingNote,
    UpdateLabel(String),
    UpdateContents(String),
    DeleteNote
}

#[derive(Properties, PartialEq)]
pub struct NoteProps {
    pub id: Uuid,
    pub label: String,
    pub contents: String,
}

impl Component for MatNote {
    type Message = Msg;
    type Properties = NoteProps;

    fn create(ctx: &Context<Self>) -> Self {
        let update_form = UpdateNote {
            id: ctx.props().id,
            label: ctx.props().label.clone(),
            contents: ctx.props().contents.clone(),
        };

        Self {
            update_form,
            updating: false,
            deleted: false
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SubmitUpdateForm => {

                let cloned_form = self.update_form.clone();
                spawn_local(async move {
                    let response = update_note(cloned_form).await;

                    if response.is_err() {
                        todo!()
                    }
                });

                self.updating = false;
                return true;
            },
            Msg::UpdatingNote => {
                self.updating = true;
                return true;
            },
            Msg::UpdateLabel(label) => self.update_form.label = label,
            Msg::UpdateContents(contents) => self.update_form.contents = contents,
            Msg::DeleteNote => {
                let id = ctx.props().id;
                spawn_local(async move {
                    let deleted = delete_note(id).await;

                    if deleted.is_err() {
                        todo!()
                    }
                });

                self.deleted = true;
                return true;
            }
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.deleted {
            return html!{
                <div></div>
            }
        }

        html! {
            <div align="center" style="padding: 2px; border-style: solid; border-width: 2px; border-color: blue; border-radius: 10px; width: 95%; margin-bottom: 10px">
                if self.updating {
                    <MatTextField 
                        label="Label" 
                        size=1000
                        value={self.update_form.label.clone()}
                        oninput={ctx.link().callback(|s: String| Msg::UpdateLabel(s))}/><br/>
                    <MatTextArea 
                        value={self.update_form.contents.clone()}
                        label="Contents" 
                        helper="Your note contents"
                        cols=1000
                        rows=5
                        oninput={ctx.link().callback(|s: String| Msg::UpdateContents(s))}/><br/>
                }
                else {
                    <h3 align="center"> {self.update_form.label.clone()} </h3>
                    <p style="white-space: pre-wrap;"> {self.update_form.contents.clone()} </p>
                }

                <div align="center">
                    if self.updating {
                        <MatFabButton icon="check" onclick={ctx.link().callback(|_| Msg::SubmitUpdateForm)}/>
                    }
                    else {
                        <MatFabButton icon="edit" onclick={ctx.link().callback(|_| Msg::UpdatingNote)}/>
                        <MatFabButton icon="delete" onclick={ctx.link().callback(|_| Msg::DeleteNote)}/>
                    }
                </div>
            </div>
        }
    }
}
