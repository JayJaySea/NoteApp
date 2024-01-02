use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SubmitButtonProps {
    pub label: String,
}

#[function_component(SubmitButton)]
pub fn input(props: &SubmitButtonProps) -> Html {
    html! {
        <button type="submit" >
            {props.label.clone()}
        </button>
    }
}
