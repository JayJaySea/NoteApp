use gloo::console::info;
use material_yew::{text_inputs::{MatTextField, TextFieldType}, button::MatButton, tabs::{MatTabBar, MatTab}};
use yew::{prelude::*, html::Scope};


pub struct RegisterPage {
    form: FormData,
}

#[derive(Default)]
struct FormData {
    email: String,
    username: String,
    password: String,
    confirm_password: String
}

pub enum Msg {
    SubmitForm,
    UpdateUsername(String),
    UpdateEmail(String),
    UpdatePassword(String),
    UpdateConfirmPassword(String),
}

impl Component for RegisterPage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            form: FormData::default()
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SubmitForm => {
                wasm_logger::init(wasm_logger::Config::default());
                info!("Registered");
            }
            Msg::UpdateUsername(value) => self.form.username = value,
            Msg::UpdateEmail(value) => self.form.email = value,
            Msg::UpdatePassword(value) => self.form.password = value,
            Msg::UpdateConfirmPassword(value) => self.form.confirm_password = value,
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div align="center" class="register-page">
                <h2> {"Create new account"} </h2>
                <p><MatTextField field_type={TextFieldType::Email} label="Email"/></p>
                <p><MatTextField label="Username" /></p>
                <p><MatTextField field_type={TextFieldType::Password} label="Password" /></p>
                <p><MatTextField field_type={TextFieldType::Password} label="Confirm Password" /></p>
                <p onclick={ctx.link().callback(|_| Msg::SubmitForm)}><MatButton label="Register"/></p>
            </div>
        }
    }
}
