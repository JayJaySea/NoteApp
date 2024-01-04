use std::rc::Rc;

use material_yew::{text_inputs::{MatTextField, TextFieldType, MatTextArea}, button::MatButton, tabs::{MatTabBar, MatTab}};
use uuid::Uuid;
use yew::{prelude::*, html::Scope};
use yewdux::dispatch::Dispatch;

use crate::{api::types::User, components::button::{MatLogoutButton, MatUpdateProfileButton, MatUpdatePasswordButton, MatUpdateButton, MatDeleteButton}, State};


pub struct MatNote {
    update_form: UpdateNoteForm,
    updating: bool
}

#[derive(Default)]
pub struct UpdateNoteForm {
    pub label: String,
    pub contents: String,
}

pub enum Msg {
    SubmitUpdateForm,
    UpdatingNote,
    UpdateLabel(String),
    UpdateContents(String)
}

#[derive(Properties, PartialEq)]
pub struct NoteProps {
    pub label: String,
    pub contents: String,
}

impl Component for MatNote {
    type Message = Msg;
    type Properties = NoteProps;

    fn create(ctx: &Context<Self>) -> Self {
        let update_form = UpdateNoteForm {
            label: ctx.props().label.clone(),
            contents: ctx.props().contents.clone(),
        };

        Self {
            update_form,
            updating: false
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SubmitUpdateForm => (),
            Msg::UpdatingNote => {
                self.updating = !self.updating;
                return true;
            },
            Msg::UpdateLabel(label) => self.update_form.label = label,
            Msg::UpdateContents(contents) => self.update_form.contents = contents,
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let delete = { Callback::from(|_| ()) };

        html! {
            <div align="center" style="border-style: solid; border-width: 2px; border-color: blue; corner-radius: 10px; width: 95%; margin-bottom: 10px">
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
                    <MatUpdateButton onclick={ctx.link().callback(|_| Msg::UpdatingNote)}/>
                    <MatDeleteButton onclick={delete}/>
                </div>
            </div>
        }
    }
}
